use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_src_dst(l: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, tag(" to "), alphanumeric1)(l)
}

fn parse_len(l: &str) -> IResult<&str, i32> {
    preceded(tag(" = "), nom::character::complete::i32)(l)
}

pub fn parse_line(l: &str) -> IResult<&str, (i32, (&str, &str))> {
    let (rem, (src, dst)) = parse_src_dst(l)?;
    let (input, len) = parse_len(rem)?;
    Ok((input, (len, (src, dst))))
}

