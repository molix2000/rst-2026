# Strings:</p>

In Rust, a string literal is a sequence of characters enclosed in double quotes. They are stored in the program's binary and are technically **slices** pointing to read-only memory.

### 1. Basic String Literal
The most common form is the standard string literal. Its type is `&'static str` (a string slice with a "static" lifetime).

```rust
let greeting = "Hello, world!";
```

---

### 2. Raw String Literals
If you need to include backslashes or double quotes without escaping them (like in a Windows file path or a Regex pattern), you use the **raw string** syntax: `r"..."`.

```rust
// Without raw strings, you'd need "C:\\Users\\Desktop"
let path = r"C:\Users\Desktop"; 

// Use hashes if your string contains double quotes
let json_data = r#"{"key": "value"}"#;
```

---

### 3. Multi-line Strings
Rust allows string literals to span multiple lines. The newline characters and leading spaces will be included in the string.

```rust
let multi_line = "This is a string
that spans multiple
lines.";
```

> **Pro Tip:** If you want to break a long line in your code but *don't* want a newline character in the actual string, use a backslash `\` at the end of the line:

```rust
let long_string = "This is a very long string \
                   that continues on the next line \
                   without a newline character.";
```

---

### 4. Byte String Literals
If you need an array of bytes (`&[u8]`) instead of a UTF-8 string, prefix the literal with `b`.

```rust
let bytes = b"ASCII only"; 
// type is &[u8; 10]
```

---

### Key Distinction: `&str` vs `String`
It is important to remember that a string literal is **immutable** and has a fixed size. To create a growable, heap-allocated string from a literal, you must convert it:

```rust
let managed_string = "hello".to_string();
// OR
let managed_string = String::from("hello");
```

## Why String</p>

[Lack of string_copy](https://medium.com/@varun-doshi/why-rust-strings-dont-implement-copy-bf1635d7a723)