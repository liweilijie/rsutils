use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

// 验证电子邮件地址格式是否正确，并提取@符号之前的所有内容
fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
        ^(?P<login>[^@\s]+)@
        ([[:word:]]+\.)*
        [[:word:]]+$"
        )
        .unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );
    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);

    let s = "123-4567-89,987-6543-21";
    println!("origin str:{}", s);
    let reg_s = r"\d{3}-(\d{4})-\d{2}";
    println!("reg:{}", reg_s);
    let r = Regex::new(reg_s)?;
    if r.is_match(s) {
        println!("Found Matches:");
    }

    for (i, c) in r.captures_iter(&s).enumerate() {
        for j in 0..c.len() {
            println!("group {},{} : {}", i, j, &c[j]);
        }
    }

    Ok(())
}
