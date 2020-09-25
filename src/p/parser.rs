use parcel::prelude::v1::*;
use parcel::{MatchStatus, left, right, join};
use crate::p::Formula;

// copied from https://github.com/ncatelli/spasm/blob/master/src/parser/combinators.rs#L41
pub fn whitespace<'a>() -> impl Parser<'a, &'a str, char> {
  move |input: &'a str| match input.chars().next() {
      Some(next) if next.is_whitespace() && next != '\n' => {
          Ok(MatchStatus::Match((&input[1..], next)))
      }
      _ => Ok(MatchStatus::NoMatch(input)),
  }
}

pub fn alphabetic<'a>() -> impl Parser<'a, &'a str, char> {
  move |input: &'a str| match input.chars().next() {
      Some(next) if next.is_alphabetic() => Ok(MatchStatus::Match((&input[1..], next))),
      _ => Ok(MatchStatus::NoMatch(input)),
  }
}

pub fn expect_character<'a>(expected: char) -> impl Parser<'a, &'a str, char> {
  move |input: &'a str| match input.chars().next() {
      Some(next) if next == expected => Ok(MatchStatus::Match((&input[1..], next))),
      _ => Ok(MatchStatus::NoMatch(input)),
  }
}

pub fn expect_str<'a>(expected: &'static str) -> impl Parser<'a, &'a str, String> {
  move |input: &'a str| {
      let preparse_input = input;
      let expected_len = expected.len();
      let next: String = input.chars().take(expected_len).collect();
      if &next == expected {
          Ok(MatchStatus::Match((&input[expected_len..], next)))
      } else {
          Ok(MatchStatus::NoMatch(preparse_input))
      }
  }
}

pub fn eof<'a>() -> impl Parser<'a, &'a str, char> {
  move |input: &'a str| match input.chars().next() {
      Some(_) => Ok(MatchStatus::NoMatch(input)),
      None => Ok(MatchStatus::Match((&input[0..], ' '))),
  }
}

pub fn p_formula<'a>() -> impl Parser<'a, &'a str, Formula> {
  left(join(formula(), eof()))
}

fn formula<'a>() -> impl Parser<'a, &'a str, Formula> {
  formula_variable().or(|| formula_negation())
}

fn formula_variable<'a>() -> impl Parser<'a, &'a str, Formula> {
  alphabetic().map(|ch| {
    Formula::Variable(ch.to_string())
  })
}

fn formula_negation<'a>() -> impl Parser<'a, &'a str, Formula> {
  right(join(expect_str("!"), formula())).map(|formula| {
    Formula::Not(Box::new(formula))
  })
}
