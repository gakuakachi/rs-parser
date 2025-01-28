mod tokernizer;
fn main() {
    println!("Hello, world!");
    let v = "11aa11a";
    println!("{}|{:?}", v, tokernizer::number(v));
}
