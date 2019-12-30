use crate::lang;
#[test]
fn calculator0() {
    let mut errors = Vec::new();

    let expr = lang::ExprsParser::new()
        .parse(&mut errors, "22 * 3 + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 3) + 3)]");

    // let expr = lang::ExprsParser::new()
    //     .parse(&mut errors, "22 * 44 + 66, *3")
    //     .unwrap();
    // assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");

    // let expr = lang::ExprsParser::new().parse(&mut errors, "*").unwrap();
    // assert_eq!(&format!("{:?}", expr), "[(error * error)]");

    // assert_eq!(errors.len(), 4);
}

#[test]
fn test1() {
    let mut errors = Vec::new();
    let expr = lang::ExprsParser::new()
        .parse(&mut errors, "x = 1")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "");
}
