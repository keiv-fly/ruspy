extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .process_file("src/lang.lalrpop")
        .unwrap();

    println!("cargo:rerun-if-changed=src/lang.lalrpop");
}
