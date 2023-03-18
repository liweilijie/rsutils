use anyhow::{anyhow, Result};
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use image::imageops::overlay;
use image::GenericImageView;
use image::{load_from_memory, DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;
use std::fmt::Display;
use std::thread;
use std::time::Instant;

fn url2image(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    fn to_anyhow(e: impl Display) -> anyhow::Error {
        anyhow!(e.to_string())
    }

    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;

    tab.wait_until_navigated()?;

    let jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        Some(75),
        None,
        true,
    )?;

    println!("time spent on url2image: {}ms", start.elapsed().as_millis());
    Ok(load_from_memory(&jpeg_data)?)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    let code = QrCode::new(url.as_bytes())?;

    // Render the bits into an image.
    let buf = code.render::<Luma<u8>>().build();
    println!(
        "time spent on gen_qrcode: {}ms",
        start.elapsed().as_millis()
    );
    Ok(DynamicImage::ImageLuma8(buf))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let start = Instant::now();
    let x = bottom.width() - top.width() - 10;
    let y = bottom.height() - top.height() - 10;
    overlay(
        bottom,
        top,
        (x as i64).try_into().unwrap(),
        (y as i64).try_into().unwrap(),
    );
    println!(
        "time spent on do_overlay: {}ms",
        start.elapsed().as_millis()
    );
}

pub fn web2image(url: &str, output: &str, format: ImageFormat) -> Result<()> {
    let url = url.to_owned();
    let url1 = url.clone();
    let bottom_handle = thread::spawn(move || url2image(&url).unwrap());
    let qrcode_handle = thread::spawn(move || gen_qrcode(&url1).unwrap());
    let mut bottom = bottom_handle.join().unwrap();
    let qrcode = qrcode_handle.join().unwrap();
    do_overlay(&mut bottom, &qrcode);

    let start = Instant::now();
    bottom.save_with_format(output, format)?;
    println!("time spent on web2image: {}ms", start.elapsed().as_millis());
    Ok(())
}
