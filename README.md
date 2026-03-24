# Introduction:</p>
A journey into Rust. Designed to focus on progress steps and learned lessons.


## Memory:</p>
Two elements STACK and HEAP, most programmers know those two.</p> Thre are some key features:</p>

### Stack: </p>
Afiliated with Fixed memory data items.</p>

### Heap: </p>
Afiliated with variable memory items sizes.</p>


```mermaid

flowchart TD;


    ReadMe[MEMORY] --> MEMRef
    MEMRef[TYPES]
    
    
    MEMRef --> STACK[STACK Spec]
    MEMRef --> HEAP[HEAP Spec]

    STACK --> FM[Fixed Memory]
    HEAP --> VM[Varied Memory]
    STACK --> ORDL[FILO]
    HEAP --> ORDF[FIFO]

    style ReadMe fill:#f9f,stroke:#333,stroke-width:4px
    style Rust fill:#bbf,stroke:#333,stroke-width:2px

```