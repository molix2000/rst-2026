# Details:</p>
In my work I have come accross the two patterns below, either is to be followed for correctness, no mix and match between them will work, this is to do with how the structures holding values or ones refering to types are presented.

A- Option:</p>

```
fn main() {
    // Explicitly typed as a 3-element tuple of string slices
    let leicester: (&str, &str, &str) = ("Mazinger", "GoRobot", "Goldrake");
    
    println!("l-one is {0}, l-2 is {1}, l-3 is {2}", leicester.0, leicester.1, leicester.2);
}

```

B- Option:</p>


```
fn main() {
    // Array of 3 string slices
    let leicester: [&str; 3] = ["Mazinger", "GoRobot", "Goldrake"];
    
    println!("l-one is {0}, l-2 is {1}, l-3 is {2}", leicester[0], leicester[1], leicester[2]);
}

```

If one wants to present the type lets say string, then the number of elements, then they need to put in square brackets only not (). 