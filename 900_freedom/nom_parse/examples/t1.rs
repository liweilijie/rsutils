use nom::{bytes::complete::tag, IResult};

fn parse(input: &str) -> IResult<&str, &str> {
    tag("#")(input)
}

fn main() {
    let (remain, pattern) = parse("#ffff7d").unwrap();
    println!("{}, {}", remain, pattern);
}
