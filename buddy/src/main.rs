fn main() {
    println!("Hello, world!");
}


struct Block {
    size: u64,
    children: Option<Vec<Block>>
}