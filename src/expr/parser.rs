use super::token::*;
use crate::{Expr, Infix, Prefix};
use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::map,
    error::{Error, ErrorKind},
    error_position,
    sequence::delimited,
    IResult,
};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    And,
    Or,
    Imply,
    Iff,
    LtlUntil,
    LtlSince,
}

fn parse_infix_op(input: Tokens) -> IResult<Tokens, (Precedence, Infix)> {
    let (input, op) = alt((
        and_tag,
        or_tag,
        imply_tag,
        iff_tag,
        ltl_until_tag,
        ltl_since_tag,
    ))(input)?;
    Ok((
        input,
        match op {
            Token::And => (Precedence::And, Infix::And),
            Token::Or => (Precedence::Or, Infix::Or),
            Token::Imply => (Precedence::Imply, Infix::Imply),
            Token::Iff => (Precedence::Iff, Infix::Iff),
            Token::LtlUntil => (Precedence::LtlUntil, Infix::LtlUntil),
            Token::LtlSince => (Precedence::LtlSince, Infix::LtlSince),
            _ => panic!(),
        },
    ))
}

fn parse_ident(input: Tokens) -> IResult<Tokens, String> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok[0].clone() {
            Token::Ident(name) => Ok((i1, name.replace('.', "_"))),
            _ => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn parse_ident_expr(input: Tokens) -> IResult<Tokens, Expr> {
    parse_ident(input).map(|(input, ident)| (input, Expr::Ident(ident)))
}

fn parse_literal(input: Tokens) -> IResult<Tokens, bool> {
    let (i1, t1) = take(1usize)(input)?;
    assert!(!t1.tok.is_empty());
    match t1.tok[0].clone() {
        Token::BoolLiteral(b) => Ok((i1, b)),
        _ => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
    }
}

fn parse_lit_expr(input: Tokens) -> IResult<Tokens, Expr> {
    map(parse_literal, Expr::LitExpr)(input)
}

fn parse_paren_expr(input: Tokens) -> IResult<Tokens, Expr> {
    delimited(lparen_tag, parse_expr, rparen_tag)(input)
}

fn parse_prefix_expr(input: Tokens) -> IResult<Tokens, Expr> {
    let (i1, op) = alt((
        not_tag,
        next_tag,
        ltl_globally_tag,
        ltl_finally_tag,
        ltl_next_tag,
        ltl_once_tag,
    ))(input)?;
    let (i2, e) = parse_atom_expr(i1)?;
    match op {
        Token::Not => Ok((i2, Expr::PrefixExpr(Prefix::Not, Box::new(e)))),
        Token::Next => Ok((i2, Expr::PrefixExpr(Prefix::Next, Box::new(e)))),
        Token::LtlGlobally => Ok((i2, Expr::PrefixExpr(Prefix::LtlGlobally, Box::new(e)))),
        Token::LtlFinally => Ok((i2, Expr::PrefixExpr(Prefix::LtlFinally, Box::new(e)))),
        Token::LtlNext => Ok((i2, Expr::PrefixExpr(Prefix::LtlNext, Box::new(e)))),
        Token::LtlOnce => Ok((i2, Expr::PrefixExpr(Prefix::LtlOnce, Box::new(e)))),
        _ => Err(nom::Err::Error(error_position!(input, ErrorKind::Tag))),
    }
}

fn parse_atom_expr(input: Tokens) -> IResult<Tokens, Expr> {
    alt((
        parse_lit_expr,
        parse_ident_expr,
        parse_paren_expr,
        parse_prefix_expr,
    ))(input)
}

fn parse_infix_expr(
    input: Tokens,
    left: Expr,
    op: Infix,
    precedence: Precedence,
) -> IResult<Tokens, Expr> {
    let (input, right) = parse_pratt_expr(input, precedence)?;
    Ok((input, Expr::InfixExpr(op, Box::new(left), Box::new(right))))
}

fn go_parse_pratt_expr(input: Tokens, precedence: Precedence, left: Expr) -> IResult<Tokens, Expr> {
    match parse_infix_op(input) {
        Ok((i1, (peek_precedence, op))) if precedence < peek_precedence => {
            let (i2, left2) = parse_infix_expr(i1, left, op, peek_precedence)?;
            go_parse_pratt_expr(i2, precedence, left2)
        }
        _ => Ok((input, left)),
    }
}

fn parse_pratt_expr(input: Tokens, precedence: Precedence) -> IResult<Tokens, Expr> {
    let (i1, left) = parse_atom_expr(input)?;
    go_parse_pratt_expr(i1, precedence, left)
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Expr> {
    parse_pratt_expr(input, Precedence::Lowest)
}

pub fn parse_tokens(input: Tokens) -> Result<Expr, nom::Err<nom::error::Error<Tokens<'_>>>> {
    let (input, expr) = parse_expr(input)?;
    assert!(input.tok.is_empty());
    Ok(expr)
}
