use regex::Regex;

pub enum Type {
    I32(i32),
    F64(f64),
    Str(String),
}

fn main() {
    let q_re = Regex::new("\"(\\.|[^\"])*\"").unwrap();
    let s = String::from("a = 1");
    let q_count = s.matches("\"").count();
    // let s_wo_q = if q_count == 0 {
    //     s
    // } else {
    //     s.split('\"').step_by(2).collect()
    // };
    let s_wo_q = s.split('\"').step_by(2).collect();
    println!("{}", s_wo_q);
    println!("Hello, world!");
    println!("hello2");
}
