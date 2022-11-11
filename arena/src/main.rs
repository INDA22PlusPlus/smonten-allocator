fn main() {
    println!("Hello, world!");
    let m = Memory::new();

    let s = m.str("hej".to_string());
}



struct Memory {
    pointer: u32,
}
impl Memory {
    fn new() -> Memory {
        Memory {  }
    }
    fn str(&self, str: String) -> Str {
        Str { index: 0, length: 1 }
    }
}


struct Str {
    index: u32,
    length: u32,
    arena: &Arena
}

struct Arena {

}
