use crate::tokenizer;
use crate::tokenizer::Tokens;

#[test]
fn test1() {
    let s = "x = 1";
    let res = tokenizer::run(s).unwrap();
    assert_eq!(
        res,
        vec![(
            vec![
                (Tokens::Identifier("x"), 0),
                (Tokens::EqSign, 2),
                (Tokens::Int32(1), 4)
            ],
            0
        )]
    );
}

#[test]
fn test2() {
    let s = "x=1";
    let res = tokenizer::run(s).unwrap();
    assert_eq!(
        res,
        vec![(
            vec![
                (Tokens::Identifier("x"), 0),
                (Tokens::EqSign, 2),
                (Tokens::Int32(1), 4)
            ],
            0
        )]
    );
}
