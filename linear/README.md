# Linear Allocator

```
let mut m = Memory::new(5);
//create mutable memory instance of 5 bytes

m.store("hello".to_string());
// m tries to store String, returns Result<(), String>

```