mod tests;
mod tokenizer;

fn main() {
    println!("{:?}", tokenizer::run("x = 1"));
}
