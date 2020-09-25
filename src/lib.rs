use wasm_bindgen::prelude::*;
use parcel::prelude::v1::*;
use parcel::MatchStatus;

mod p;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn parse_formula(input: &str) -> bool {
    match p::parser::p_formula().parse(input) {
        Ok(mat) => match mat {
            MatchStatus::Match((u, t)) => true,
            MatchStatus::NoMatch(_) => false
        },
        Err(e) => false
    }
}
