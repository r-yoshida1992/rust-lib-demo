mod generator;
pub fn print_random_numbner() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
