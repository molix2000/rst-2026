# RST 2026 Project: 

This is rst project 01 progress steps, log, system setup for deployment, (CICD/Devops), Functional programming and ML related aspect for 2026.



## Matching:</p>

One of the interesting consepts of Rust. I didn't come accross this way. From the Rust book I have picket an example which was displayed in test-3-8, in a final itteration. </p>

Best display from a graphical view would be the diagram below:</p>

```mermaid
    A[Input Value: scrutinee] -->|Compare| B1{Pattern 1}
    A -->|Compare| B2{Pattern 2}
    A -->|Compare| B3{Pattern ...}
    A -->|Compare| B4{Wildcard _}

    B1 -- Yes --> C1[Execute Code 1]
    B2 -- Yes --> C2[Execute Code 2]
    B3 -- Yes --> C3[Execute Code ...]
    B4 -- Yes --> C4[Execute Catch-all]

    C1 --> D[Return Value]
    C2 --> D
    C3 --> D
    C4 --> D
...

A simple snipet to conclude the idea:</p>

```
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    _ => 0, // Catch-all
}
```
