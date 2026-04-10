# Introduction:</p>
A journey into Rust. Designed to focus on progress steps and learned lessons.


## Memory:</p>
Two elements STACK and HEAP, most programmers know those two.</p> There are some key features:</p>

### Stack: </p>
Affiliated with Fixed memory data items.</p>

### Heap: </p>
Affiliated with variable memory items sizes.</p>


```mermaid

flowchart TD;


    ReadMe[MEMORY] --> MEMRef
    MEMRef[TYPES]
    
    
    MEMRef --> STACK[STACK Spec]
    MEMRef --> HEAP[HEAP Spec]

    STACK --> FM[Fixed Memory]
    HEAP --> VM[Varied Memory]
    STACK --> ORDL[LIFO]
    HEAP --> ORDF[FIFO]
    
    STACK --> SS[FAST]
    HEAP --> HS[SLOW]

    STACK --> DC[Data Close]
    HEAP --> DF[DATA Far]
    
    STACK --> LF[Local Function Variables]
    HEAP --> PD[Pointers to Data]

    style ReadMe fill:#f9f,stroke:#333,stroke-width:4px
    style Rust fill:#bbf,stroke:#333,stroke-width:2px

```

## Data Types:</p>

They live on either one of the two areas of memory that have been mentioned before. </p>

### String</p>
Extra help from this article:
</p>
https://www.brandons.me/blog/why-rust-strings-seem-hard

### String_slice is &str:</p>
This the best way once can describe the two.</p> symbol '&' and str. </p> Its an immutable reference to a sequence of UTF-8 characters. </p> 

Great reference here:
[Strings/Rust](https://dev.to/alexmercedcoder/in-depth-guide-to-working-with-strings-in-rust-1522) 

The '&str' is stored on 'STACK'.</p>

### Detailed examples:</p>

(Borrow & Strings)[https://dev.to/stevepryde/rust-string-vs-str-1l93]

(Strings and slices)[https://dev.to/alexmercedcoder/in-depth-guide-to-working-with-strings-in-rust-1522]


### Equalising strings</p>
In the example below we can see two strings equalized and how this works.</p>

```
fn main() {
    // 1. &str (borrowed string slice)
    let borrowed_str: &str = "Hello, world!";

    // 2. Convert to owned String using to_owned()
    let owned_string: String = borrowed_str.to_owned();

    // Now you can modify it
    let mut mutable_string = owned_string;
    mutable_string.push_str(".. and more!");
    println!("{}", mutable_string);
}


```
Ref: [link-Thursday/206-03-26](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

[Ref for study : 2026/03/27](https://users.rust-lang.org/t/why-am-i-able-to-mutate-a-string-literal/39778/16)

[Ref on playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cf4c7bd80f57b8f341f3089f0e1e66d1)

[String vs str](https://dev.to/dsysd_dev/string-vs-str-in-rust-understanding-the-fundamental-differences-for-efficient-programming-4og8)


### Functions:</p>
The storage arrangement of the function components can vary, below is a simple explanation.</p>

In Rust, the answer depends on whether you are talking about the function code itself or function pointers and closures.</p>

To be precise: functions are not "stored" on the stack or the heap in the way data is.
1. The Function Code (Text Segment). </p>

The actual compiled instructions of a function reside in the Text Segment (or Code Segment) of the program's memory. This is a read-only area of memory loaded when the program starts. It is neither the stack nor the heap.</p>

2. Function Pointers (fn) </p>

When you pass a function around as a value using a function pointer (the fn type), the pointer itself is just a memory address. </p>

    On the Stack: If you declare a variable let f = my_function;, the address of my_function is stored on the stack.</p>

    Size: A function pointer is the size of a usize (e.g., 8 bytes on a 64-bit system).</p>

3. Closures</p>

Closures are more complex because they can "capture" variables from their environment. How they are stored depends on their type:</p>
Stack Storage (Default)</p>

By default, Rust creates a unique, anonymous struct for every closure. If you define a closure within a function, that struct is stored on the stack.</p>
Rust</p>
```
let x = 10;
let closure = |y| x + y; // This struct lives on the stack
```
Heap Storage (Boxed)</p>

If you need to return a closure from a function or store it in a way that outlives the current scope, you must "Box" it. This moves the closure's data to the heap.
Rust</p>
```
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    let x = 10;
    Box::new(move |y| x + y) // The closure is now on the heap
}
```
### Summary Table: </p>
Entity	Primary Location	Notes
Compiled Code	Text Segment	Read-only; loaded at execution start.
Function Pointer	Stack	Just a usize address pointing to the Text Segment.</p>
Normal Closure	Stack	An anonymous struct containing captured variables.
Boxed Closure	Heap	Used for dynamic dispatch (dyn Fn) or returning closures.</p>

The "Stack Frame" nuance: While the code isn't on the stack, every time a function is called, a</p> new stack frame is created. This frame stores the function's local variables and return address, but it is destroyed as soon as the function returns. </p>


### Structural Organization:</p>

To make the code more distributed , separated by files within same folder,</p>
The use of mod and the name of the separate file was used in the chapter-4-3 example.</p>

```
mod loopmania;
use loopmania::loopmania;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
}

```

### Note: </p>
The 'use' keyword was followed by a more specific indication to what function to use.</p>
One can still use the '*' to include multiple.


### Tuple deconstructor:</p>

In this example below one can see that</p> statement below equalize:</p> 
(s2,len) to calculate_length(s1);

It may be confusing but the calculate_lenght,</p> returns two parameters, so that would be correct.</p>

```
fn main() {
let s1 = String::from("hello");
let (s2, len) = calculate_length(s1);
println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
let length = s.len(); // len() returns the length of a String
(s, length)
}
```
This can be a way of getting around the rust ownership model, as string one which was owned by main or within the function that declared it 1st. It now has passed its context and length to calculate_length.</p>

### Cleaner way:</p>
Perhaps this is better?</p>

```
fn main() {
    let s1 = String::from("hello");
    // We pass a reference (&s1) instead of the whole string
    let len = calculate_length(&s1); 
    
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```

### Ownership:</p>

One of the most important aspects of Rust, every thing must by owned by some one only. </p>
Here below, one can see that the person structure was obtaining a value initially, then it passed the ownership to the save_person, the latter saved the value to the database. </p>

If one wants or needs to call the value of  the structure, an error would be encounterred. To correct or provide an alternative one can create a return value in the save_person fn, this would lead to the ability to re-assign the person structure value to person two. </p> Thus facilitating to it to be used within the println! macro.</p>

```
fn struct_handle() {

    let person = get_person();
    let person2 = save_person(person);
    println!("Name= {}", person2.name);
}

fn save_person(person: Person){

    DB::write(person);
    return person;
}
fn get_person (name:String) -> person {

    println!("Please input person name");
    person {
        name: stdin.read_line().unwrap(),
        job: "Software Engineer".to_string(),
    }

}

### Heap and Stack:</p>
Any thing with unknown length will go to heap, Box will always reside on heap segment..</P>