use std::io::{self, Write};

fn main() {
    // this should later be called multi_mut_example() //
    let mut x = 1;
    let mx1 = &mut x;
    //let mx2 = &mut x;
    // can't have a number of mutable references to the same object at the same time
    
    // but can have multiple immutable references to the same object at the same time
    // reason is cache coherence and race conditions.
    *mx1 = 2;
    // The line above is a reference to a location in memory, and the line below is a reference to the value at that location in memory.
    // Rust has pointers like *mut T *const T and they are regarded as unsafe. 
    // It also has smart pointers such  as Box<T> Rc<T> and Arc<T> , RefCell<T> and Mutex<T> which are safe to use.
    //let mx2 = &x;
    println!("X = {}", mx1);
    println!("X original - {}", x);

    let person = get_person();
    println!("Person: {} ({})", person.name, person.job);
}

fn get_person() -> Person {
    let mut name = String::new();
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    let name = name.trim_end().to_string();
    Person {
        name,
        job: "Software Engineer".to_string(),
    }
}

struct Person {
    name: String,
    job: String,
}

### Life time of a file:</p>
Its important to know that Rust give the file a certain life time. </p>
Below is the example that showws the life will not be byeond the parenthesis.</p>

```

fn rocess_file() -> bool {


    let data = generate_data();
    file {

        let file = File::open(path).expect("Failed open");
        file.write(data.raw()).expect("Failed to close file");
        // file is closed here.
        // or one can add this statement to drop the file:
        drop(file); // The file doesn't have ampersand because it's not a reference, it's a value that is moved into the file variable.
    }
}