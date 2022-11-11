fn main() {
    let mut m = Block::new();
    dbg!(&m);
    m.store_str("Hejsan".to_string());
    dbg!(&m);
}

#[derive(Debug, Clone)]
struct Block {
    size: u32,
    content: Content
}
impl Block {
    fn new() -> Block {
        let size: u32 = 2u32.pow(4);
        Block {
            size: size,
            content: Content::Children(vec![
                Block::_new(size/2),
                Block::_new(size/2)
            ])
        }
    }
    fn _new(size: u32) -> Block {
        match size {
            2 => {
                Block {
                    size: 2,
                    content: Content::Memory(Memory::new(2))
                }
            },
            _ => {
                Block {
                    size: size,
                    content: Content::Children(vec![
                        Block::_new(size/2),
                        Block::_new(size/2)
                    ])
                }
            }
        }
    }
    fn store_str(&mut self, s: String) {
        let buf = s.as_bytes();
        self._store_buf(buf).unwrap();
    }

    fn _store_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        if buf.len() + 1 > self.size as usize {
            panic!("not enough space!");
        } else {
            match &mut self.content {
                Content::Children(c) => {
                    if buf.len() + 1 > self.size as usize / 2 {
                        if c[0].is_free() {
                            let mut m = Memory::new(self.size);
                            m.store(buf).unwrap();
                            self.content = Content::Memory(m);
                            Ok(())
                        } else {
                            Err("not enough space".to_string())
                        }
                    } else {
                        match c[0]._store_buf(buf) {
                            Err(_) => c[1]._store_buf(buf),
                            Ok(()) => return Ok(())
                        }
                    }
                },
                Content::Memory(m) => {
                    m.store(buf)
                }
            }
        }
    }

    fn is_free(&self) -> bool {
        match &self.content {
            Content::Children(c) => c[0].is_free(),
            Content::Memory(m) => m.pointer == 0
        }
    }
}

#[derive(Debug, Clone)]
enum Content {
    Memory(Memory),
    Children(Vec<Block>)
}


#[derive(Debug, Clone)]
struct Memory {
    pointer: usize,
    memory: Vec<RegContent>
}
impl Memory {
    fn new(size: u32) -> Memory {
        Memory { pointer: 0, memory: vec![RegContent::Byte(0); size as usize] }
    }
    fn store(&mut self, buf: &[u8]) -> Result<(), String> {
        if self.pointer + buf.len() + 1 > self.memory.len() {
            Err("not enough space!".to_string())
        } else {
            self.memory[self.pointer] = RegContent::Reserved(buf.len() as u32);
            self.pointer+=1;
            for i in 0..buf.len() {
                self.memory[self.pointer + i] = RegContent::Byte(buf[i])
            }
            self.pointer+=buf.len();
            Ok(())
        }
    }
}


#[derive(Debug, Clone)]
enum RegContent {
    Byte(u8),
    Reserved(u32)
}