#![allow(unused)]
extern crate lazy_static;
extern crate phf;

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

// 假设我们正在构建一个Web浏览器引擎。 在成千上万要关注的事情中，我们应该能够呈现彩色文本。
// 应该看起来像是以蓝色字体设置的段落。 但蓝色是人类可读的颜色名称，计算机只能读懂数字。 定义Color结构体：
#[derive(Debug, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// 我们无法创建静态HashMap并初始化数据：key为颜色名称、value为颜色。 首先想到，我们可以使用模式匹配来按名称查找颜色：
// 缺点是匹配字符串切片是一个线性递增的搜索：拥有的颜色越多，find_color就越慢。我们可以创建一个静态HashMap吗？
pub fn find_color_slow(name: &str) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "amber" => Some(Color { r: 255, g: 191, b: 0 }),
        // hundreds of other names...
        "zinnwaldite brown" => Some(Color { r: 44, g: 22, b: 8 }),
        _ => None,
    }
}


// lazy_static! 是一个允许以非平凡的方式初始化的静态变量的包。
// 例如，预先计算的常规表达式，例如docopt中使用的表达式，或静态HashMap。
lazy_static! {
    static ref COLORS_MAP: Mutex<HashMap<&'static str, Color>> = {
        let mut map = HashMap::new();
        map.insert("amber", Color {r:255, g:191, b: 0});
        // ...
        map.insert("zinnwaldite brown", Color{r:44, g: 22, b:8});
        Mutex::new(map)
    };
}

pub fn find_color_lazy_static(name: &str) -> Option<Color> {
    // COLORS_MAP将在首次访问时进行初始化。 我们现在可以安全地将其视为常规静态变量。
    COLORS_MAP.lock().unwrap().get(name.to_lowercase().as_str()).cloned()
}

/// phf 版本
/// HashMap使用有点慢的哈希算法（引用文档）来避免DoS攻击。 在数据量足够大的map中有可能发生冲突。
/// 另一方面，phf使用完美散列（散列，保证不冲突）来构建编译时map。 这样我们就可以在运行时进行有效的恒定时间查找。
lazy_static! {
    static ref COLORS: Mutex<phf::Map<&'static str, Color>> = Mutex::new(phf::phf_map! {
      "amber" => Color {r:255, g: 191, b:0},
      "zinnwaldite brown" => Color {r:44, g:22, b:8},
    });
}

pub fn find_color_phf(name: &str) -> Option<Color> {
    COLORS.lock().unwrap().get(name.to_lowercase().as_str()).cloned()
}


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_match_loop(b: &mut Bencher) {
        b.iter(|| find_color_slow("White"))
    }

    #[bench]
    fn bench_lazy_static_map(b: &mut Bencher) {
        b.iter(||find_color_lazy_static("White"))
    }

    #[bench]
    fn bench_phf_map(b: &mut Bencher) {
        b.iter(||find_color_phf("White"))
    }
}