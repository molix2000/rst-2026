# RST 2026 Project: 

This is rst project 01 progress log for 2026.

#### File handling samples:</p>

This code segment show that try is a macro with similar</p> structure to println!();
```
let mut file = try!(File::open(fname).context(fname));
try!(file.read_to_string(&mut buf).context(fname));
try!(buf.parse().context(fname))

```