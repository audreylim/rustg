## Rustg 

Rustg is a simple grep utility written in Rust.

### Installation

[Install](http://doc.rust-lang.org/1.0.0-beta.2/book/installing-rust.html) Rust.

### Compile

```
rustc rustg.rs
```

### Perform grep

Example:

**1. Target file**
```
rustg line example.txt
```

**2. Multiple target files**
```
rustg line example.txt example1.txt
```

**3. Pipe**
```
ls | rustg {target text} 
```
