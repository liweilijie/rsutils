mod web2image;
use std::{ffi::OsStr, path::Path};

use crate::web2image::web2image;
use clap::Parser;
use image::ImageFormat;
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "liwei", version = "0.1", about = "web2image", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, value_parser = valid_filename, default_value = "./tmp/snapshot.jpg")]
    output: String,

    /// Number of times to greet
    #[arg(short, long, value_parser = valid_url)]
    url: String,
}

fn get_image_format(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => None,
            }
        })
}

/// "/tmp/abc.pdf" => "/tmp" exists, pdf (png | jpg | jpeg)
fn valid_filename(name: &str) -> Result<String, String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|p| p.is_dir().then_some(p));
    let ext = get_image_format(path);

    if parent.is_none() || ext.is_none() {
        return Err("file path must be exists and file must be jpg, jpeg or png.".into());
    }

    Ok(name.into())
}

fn valid_url(url: &str) -> Result<String, String> {
    match Url::parse(url) {
        Ok(_) => Ok(url.into()),
        Err(_) => Err("You must provide a valid url.".into()),
    }
}

fn main() {
    let args = Args::parse();

    println!("{args:#?}");

    let format = get_image_format(Path::new(&args.output)).unwrap();

    web2image(&args.url, &args.output, format).unwrap();
}
