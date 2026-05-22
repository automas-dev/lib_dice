use nom::{
    bytes::complete::tag, bytes::complete::take_while1, character::complete::space0,
    combinator::map_res, combinator::opt, IResult, Parser,
};

/// Parse an decimal integer from an input string
fn parse_int<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(take_while1(|c: char| c.is_digit(10)), T::from_str).parse(input)
}

/// Parse dice notation with count and type
fn parse_dice(input: &str) -> IResult<&str, (usize, u8)> {
    let (input, (count, _, die)) = (parse_int::<usize>, tag("d"), parse_int::<u8>).parse(input)?;
    Ok((input, (count, die)))
}

/// Parse bonus extension to dice notation
fn parse_bonus(input: &str) -> IResult<&str, i32> {
    let (input, _) = (space0, tag("+"), space0).parse(input)?;
    let (input, bonus) = parse_int::<i32>(input)?;
    Ok((input, bonus))
}

/// Parse string into DiceRoll using the format {count}d{sides} with optional +{bonus}
pub fn parse_roll(input: &str) -> IResult<&str, (usize, u8, i32)> {
    let (input, (count, dice)) = parse_dice(input)?;
    let (input, bonus) = opt(parse_bonus).parse(input)?;
    Ok((input, (count, dice, bonus.unwrap_or(0))))
}

#[test]
fn test_parse_int() {
    assert_eq!(IResult::Ok(("", 0)), parse_int::<i32>("0"));
    assert_eq!(IResult::Ok(("d", 1)), parse_int::<i32>("1d"));
    assert_eq!(IResult::Ok(("d3", 12)), parse_int::<i32>("12d3"));
}

#[test]
fn test_parse_dice() {
    assert_eq!(IResult::Ok(("", (1, 8))), parse_dice("1d8"));
}

#[test]
fn test_parse_bonus() {
    assert_eq!(IResult::Ok(("", 0)), parse_bonus("+0"));
    assert_eq!(IResult::Ok(("", 1)), parse_bonus("+1"));
    assert_eq!(IResult::Ok(("", 1)), parse_bonus(" + 1"));
}

#[test]
fn test_parse_roll() {
    assert_eq!(IResult::Ok(("", (1, 8, 0))), parse_roll("1d8"));
    assert_eq!(IResult::Ok(("", (2, 8, 3))), parse_roll("2d8+3"));
    assert_eq!(IResult::Ok(("", (2, 8, 3))), parse_roll("2d8 + 3"));
}
