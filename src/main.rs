mod tokenizer;

#[cfg(test)]
mod tests;

fn main() {
    println!("{:?}", tokenizer::run("x = 1"));
}
