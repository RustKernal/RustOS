# RustOS
## Table Of Contents

----

## Introduction

## Installation

```bash
cargo component add llvm-tools-preview
cargo install bootimage
cargo install rust_os
```

```toml
[dependencies]
rust_os = "0.1.0"
```



## Usage
```rust
#![no_std]
#![no_main]
use kernal::{println, spin);
use bootloader::{entry_point,BootInfo};

entry_point!(main)

fn main(bootInfo: &'static BootInfo) -> ! {
    kernal::init();
    kernal::init_heap(bootInfo);
    println!("Hello World!");
    spin!()
}
```


## Libraries
- volatile (0.2.6)
- bootloader (0.9.8)
- spin (0.5.2)
- x86_64 (0.13.2)
- uart_16550 (0.2.0)
- pic8259_simple (0.2.0)
- pc-keyboard (0.5.0)
- lazy_static (1.0)
- linked_list_allocator (0.8.0)