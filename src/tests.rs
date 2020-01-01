mod parser_tests {
    use crate::tokenizer;
    use crate::tokenizer::Tokens;

    #[test]
    fn test1() {
        let s = "x = 1";
        let res = tokenizer::run(s).unwrap();
        assert_eq!(
            res,
            vec![vec![
                (Tokens::Identifier("x"), 0),
                (Tokens::EqSign, 2),
                (Tokens::Int64(1), 4)
            ]]
        );
    }

    #[test]
    fn test2() {
        let s = "x=1";
        let res = tokenizer::run(s).unwrap();
        assert_eq!(
            res,
            vec![vec![
                (Tokens::Identifier("x"), 0),
                (Tokens::EqSign, 1),
                (Tokens::Int64(1), 2)
            ]]
        );
    }

    #[test]
    fn test3() {
        let s = "x\x00=1";
        let res = tokenizer::run(s).unwrap();
        assert_eq!(
            res,
            vec![vec![
                (Tokens::Identifier("x"), 0),
                (
                    Tokens::Error(
                        "Parsing error. Char \u{0} cannot be parsed. Not implemented".to_string(),
                        1,
                        0,
                        1
                    ),
                    1
                ),
                (Tokens::EqSign, 2),
                (Tokens::Int64(1), 3)
            ]]
        );
    }

    #[test]
    fn test4() {
        let s = "x = 1;1";
        let res = tokenizer::run(s).unwrap();
        assert_eq!(
            res,
            vec![
                vec![
                    (Tokens::Identifier("x"), 0),
                    (Tokens::EqSign, 2),
                    (Tokens::Int64(1), 4)
                ],
                vec![(Tokens::Int64(1), 6)]
            ]
        );
    }

    #[test]
    fn test5() {
        let s = "x = \"Hello World\";1";
        let res = tokenizer::run(s).unwrap();
        assert_eq!(
            res,
            vec![
                vec![
                    (Tokens::Identifier("x"), 0),
                    (Tokens::EqSign, 2),
                    (Tokens::Str("Hello World"), 4)
                ],
                vec![(Tokens::Int64(1), 18)]
            ]
        );
    }

    #[test]
    fn test6() {
        let s = "x = \"Hello World";
        let res = tokenizer::run(s).unwrap();
        assert_eq!(
            res,
            vec![vec![
                (Tokens::Identifier("x"), 0),
                (Tokens::EqSign, 2),
                (
                    Tokens::Error(
                        "No matching end quote for quote at line 0, column 4".to_string(),
                        16,
                        0,
                        15
                    ),
                    4
                )
            ]]
        );
    }
}

use crate::env::{BaseTypeEnum, Env, StringT};
use crate::vm::VM;
use std::cell::RefCell;

#[test]
fn test1() {
    let mut env = Env::new();
    env.globals.insert(
        "x".to_string(),
        RefCell::new(BaseTypeEnum::Str(StringT::Str(Box::new(String::from(
            "abcdef",
        ))))),
    );
    let res = env.globals.remove("x").unwrap().into_inner();
    assert_eq!(
        res,
        BaseTypeEnum::Str(StringT::Str(Box::new(String::from("abcdef"))))
    );
}

#[test]
pub fn test2() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    struct Str(String);

    impl Drop for Str {
        fn drop(&mut self) {
            println!("Dropped")
        }
    }

    {
        let mut m = HashMap::new();
        m.insert("x", RefCell::new(Str("hello".to_string())));
        println!("1");
        m.remove("x");
        println!("2");
        assert_eq!(1, 1);
    }
    println!("3");
    //Out:
    // 1
    // Dropped
    // 2
    // 3
}

#[test]
fn test3() {
    let s = "1 + 1";
    let vm = VM::new();
    let res = vm.execute(s);
    assert_eq!(1i64, 2i64);
}
