use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod p;

fn main() {
  let result = match p::parser::p_formula().parse("!a") {
    Ok(mat) => match mat {
        MatchStatus::Match((u, t)) => true,
        MatchStatus::NoMatch(u) => {
          print!("{}", u);
          false
        }
    },
    Err(e) => false
  };

  print!("{}", result);
}