fn main() {
    println!("Chapter 4 2");

    //  fn main()
    //{
    // These are references to strings, so the value of
    // the text isn't stored in them. Just the address of
    // where in memory the string data is stored.
    // This is a fat pointer, containing 2 usizes,
    // the memory address of the first character and the length
    // of the string (slice).
    //
    // The actual string data for a string literal (hardcoded in
    // the source code) will be in the data segment of the process.
    // This is a read-only area in memory with data that is
    // hardcoded in the binary.
    //
    let mut x: &str = "hello";
    let y: &str = "world";

    // Let's find out where in the processes memory this string data
    // lives. Its phone number if you want. Where can we find this
    // string?
    //
    let x_addr = x.as_ptr() as usize;
    let y_addr = y.as_ptr() as usize;

    // This line will print something like:
    // Address of x (hello) = 0x55a36c3486d0
    //
    // Rust string format even has a special shorthand for this,
    // this will print exactly the same as the first print below:
    // println!( "Address of x ({}) = {:p}", x, x );
    //
    println!("Address of x ({}) = 0x{:2x}", x, x_addr);

    // Address of y (world) = 0x55a36c3486d5
    //
    // Note how the compiler layed them out exactly one after the other.
    // Five characters for "hello" and then starts "world".
    // It could have been somewhere else, but why not just put one
    // after the other.
    //
    // So if you would read 10 bytes from starting from the address of x,
    // you would get the string "helloworld".
    //
    // Also note that these addresses will change everytime you run the
    // process, as these are not physical addresses into the computers
    // memory, but rather into a virtual address space which also is
    // randomized for security reasons. This randomization actually
    // depends on the operating system running the process, not the
    // compiler.
    //
    println!("Address of y ({}) = 0x{:2x}", y, y_addr);

    // So let's mutate:
    //
    x = y;

    // Where is x pointing now?
    //
    // Address of x (world) = 0x55a36c3486d5
    //
    // Yep, it's pointing at "world".
    //
    println!("Address of x ({}) = {:p}", x, x);

    // If x is pointing at "world", then what the hell
    // is at 0x55a36c3486d0 ?
    // We still have x_addr, which we didn't change. It's
    // just a usize. Let's turn it into a string:
    //
    // WARNING: the following is undefined behavior. I'm just
    // going by the fact that in the playground, everytime I
    // ran this, "world" was stored directly after "hello".
    //
    // There is absolutely no guarantee this is the case,
    // so we will put an assert... Otherwise this might segfault.
    //
    assert!(x_addr + 5 == y_addr);

    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(x_addr as *const u8, 10) };
    let new: &'static str = std::str::from_utf8(bytes).expect("valid UTF");

    // OMG, we did it. It's all still there, exactly where we
    // put it. No strings were mutilated in the making of this program.
    //
    // Address of new (helloworld) = 0x55a36c3486d0
    //
    println!("Address of new ({}) = {:p}", new, new);
    // }

    // another example:
    let mut source: String = "hello".to_string();
    let s: &mut str = &mut source;
    s.make_ascii_uppercase();
    println!("{}", s);



    // new segment to prove transition from Slice to String:
    //
    let mut my_str: &str = "Rust Programme";
    let mut my_str_s = my_str.to_string();
    my_str_s.push_str(" Chapters,");
    my_str_s.push_str(" Teaches ");
    my_str_s.push_str(" precision ");
    my_str_s.push('!');
    println!("The String now is {}",my_str_s);



    // Example where string slice can be used to save memory not too much copy.

    fn print_title(title: &str) {
        println!("{title}");
    }

    let my_title_s: &str = "Rust Chapter 4.2";
    // print_title(format!(("{my_title_s}")));
    print_title(my_title_s);


    // Example STring vs Slice:
    let my_string = String::from("Hello, Rust!"); // A heap-allocated String
    let my_slice: &str = &my_string[0..5]; // A string slice representing "Hello"

    println!("Original String: {}", my_string);
    println!("String Slice: {}", my_slice);

    let mut source : String = "Rust 1.9.2".to_string();
    let s: &mut str = &mut source;
    s.make_ascii_uppercase();
    println!("{}", s);

    // cloning a string:

    let stringoo4: String = "High Rust Book".to_string();
    let stringoo5 = stringoo4.clone();
    println!("{}, {}, these are stringoo4 & 5", stringoo4, stringoo5);
}

