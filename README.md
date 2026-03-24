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