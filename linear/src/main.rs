fn main() {
    // dbg!(m);

    let mut m = Memory::new(5);
    match m.store("bo".to_string()) {
        Ok(()) => println!("OK"),
        Err(e) => println!("FAIL: {}", e),
    }
    match m.store("mm".to_string()) {
        Ok(()) => println!("OK"),
        Err(e) => println!("FAIL: {}", e),
    }


    

    // let s2 = String::from_utf8_lossy(bytes);
    // println!("result: \"{}\"", s2);

}

#[derive(Debug, Clone, Copy)]
struct Register {
    byte: u8
}

impl Register {
    fn new() -> Register {
        Register {
            byte: 0u8
        }
    }
    fn set(&mut self, b: u8) {
        self.byte = b;
    }
}

#[derive(Debug, Clone)]
struct Memory {
    pointer: u8,
    memory: Vec<Register>,
    size: usize
}
impl Memory {
    fn new_memory() -> Vec<Register> {
        let mut m: Vec<Register> = vec![];
        for i in 0..4 {
            m.push(Register::new());
        }
        return m;
    }
    fn new(size: u32) -> Memory {
        Memory {
            pointer: 0,
            memory: {
                let mut m: Vec<Register> = vec![];
                for i in 0..size {
                    m.push(Register::new());
                }
                m
            },
            size: size as usize
        }
    }

    fn store(&mut self, s: String) -> Result<(), String> {
        let buf = s.as_bytes();
        return self.allocate(buf);
    }

    fn allocate(&mut self, buf: &[u8]) -> Result<(), String> {
        if buf.len() > self.size - (self.pointer as usize) {
            Err(format!("tried to enter {} bytes with {} bytes available", buf.len(), self.size - self.pointer as usize))
        } else {
            for i in 0..buf.len() {
                self.memory[self.pointer as usize + i].set(buf[i])
            }
            self.pointer += buf.len() as u8;
            dbg!(self);
            Ok(())
        }
    }
}

// pointers
