mod abi;

use prost::Message;
// 声明 abi.rs
pub use abi::*;

use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use photon_rs::transform::SamplingFilter;

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// 让ImageSpec 可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        // to base64
        let mut buf = String::new();
        CUSTOM_ENGINE.encode_string(data, &mut buf);
        buf
    }
}

// 让 ImageSpec 可以通过一个字符串创建。比如 s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = CUSTOM_ENGINE.decode(value)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数，photon_rs 相应的方法里面需要字符串
impl filter::Filter {
    pub fn to_str(self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

// 在我们定义的 SampleFilter 和 photon_rs 的 SamplingFilter 间转换
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

// 提供一些辅助函数，让创建一个 spec 的过程简单一些
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn base64_used_works() {
        use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
        const CUSTOM_ENGINE: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        let mut buf = String::new();
        general_purpose::STANDARD.encode_string(b"hello world~", &mut buf);
        println!("{}", buf); // aGVsbG8gd29ybGR+

        buf.clear();

        // encode
        CUSTOM_ENGINE.encode_string(b"hello internet~", &mut buf);
        println!("{}", buf); // aGVsbG8gaW50ZXJuZXR-

        // decode
        let bytes_url = CUSTOM_ENGINE.decode(&buf).unwrap();
        println!("{:?}", bytes_url);
    }

    #[test]
    fn encoded_spec_could_be_decode() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let images_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = images_spec.borrow().into();
        assert_eq!(images_spec, s.as_str().try_into().unwrap());
    }
}