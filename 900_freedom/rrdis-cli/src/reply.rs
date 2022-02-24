use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{take_while, take_while1, take_while_m_n};
use nom::combinator::map;
use nom::multi::many_m_n;
use nom::sequence::delimited;
use nom::IResult;

pub enum Reply {
    SingleLine(String),
    Err(String),
    Int(i16),
    Batch(Option<String>),
    MultiBatch(Option<Vec<Reply>>),
    BadReply(String),
}

fn parse_single_line(i: &str) -> IResult<&str, Reply> {
    let (i, _) = tag("+")(i)?;
    let (i, resp) = take_while(|c| c != '\r' && c != '\n')(i)?;
    let (i, _) = tag("\r\n")(i)?;
    Ok((i, Reply::SingleLine(resp.to_string())))
}

fn parse_single_line2(i: &str) -> IResult<&str, Reply> {
    let (i, resp) = delimited(
        tag("+"),
        take_while(|c| c != '\r' && c != '\n'),
        tag("\r\n"),
    )(i)?;
    Ok((i, Reply::SingleLine(resp.to_string())))
}

fn parse_err(i: &str) -> IResult<&str, Reply> {
    let (i, resp) = delimited(
        tag("-"),
        take_while1(|c| c != '\r' && c != '\n'),
        tag("\r\n"),
    )(i)?;
    Ok((i, Reply::Err(resp.to_string())))
}

fn parse_int(i: &str) -> IResult<&str, Reply> {
    let (i, int) = delimited(
        tag(":"),
        take_while1(|c: char| c.is_digit(10) || c == '-'),
        tag("\r\n"),
    )(i)?;
    Ok((i, Reply::Int(int.parse().unwrap())))
}
