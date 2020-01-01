use crate::env::Env;
use crate::tokenizer;
use crate::tokenizer::Tokens;

pub struct VM<'a> {
    env: Env<'a>,
}

impl VM<'_> {
    pub fn new<'a>() -> VM<'a> {
        VM { env: Env::new() }
    }
    pub fn execute(&self, s: &str) {
        let res = tokenizer::run(s).unwrap();
    }
}
