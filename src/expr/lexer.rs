use super::token::Token;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

macro_rules! syntax {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name(s: &str) -> IResult<&str, Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

syntax! {and_operator, "&&", Token::And}
syntax! {or_operator, "||", Token::Or}
syntax! {not_operator, "!", Token::Not}
syntax! {iff_operator, "<->", Token::Iff}
syntax! {imply_operator, "->", Token::Imply}
syntax! {lparen_punctuation, "(", Token::LParen}
syntax! {rparen_punctuation, ")", Token::RParen}

pub fn lex_operator(input: &str) -> IResult<&str, Token> {
    alt((
        and_operator,
        or_operator,
        not_operator,
        iff_operator,
        imply_operator,
    ))(input)
}

pub fn lex_punctuations(input: &str) -> IResult<&str, Token> {
    alt((lparen_punctuation, rparen_punctuation))(input)
}

fn lex_reserved_ident(input: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag(".")))),
        )),
        |s| match s {
            "next" => Token::Next,
            "TRUE" => Token::BoolLiteral(true),
            "true" => Token::BoolLiteral(true),
            "FALSE" => Token::BoolLiteral(false),
            "false" => Token::BoolLiteral(false),
            "F" => Token::LtlFinally,
            "G" => Token::LtlGlobally,
            "U" => Token::LtlUntil,
            "X" => Token::LtlNext,
            "O" => Token::LtlOnce,
            "S" => Token::LtlSince,
            _ => Token::Ident(s.to_string()),
        },
    )(input)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((lex_operator, lex_punctuations, lex_reserved_ident))(input)
}

pub fn lex_tokens(input: &str) -> Result<Vec<Token>, nom::Err<nom::error::Error<&str>>> {
    many0(delimited(multispace0, lex_token, multispace0))(input).map(|(remain, token)| {
        assert!(remain.is_empty());
        token
    })
}
