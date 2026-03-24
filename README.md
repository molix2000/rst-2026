# Introduction:</p>
A journey into Rust. Designed to focus on progress steps and learned lessons.


## Memory:</p>
Two elements STACK and HEAP, most programmers know those two.</p> Thre are some key features:</p>

### Stack


```mermaid
graph TD; 
          MEMORY }|..|{ PROCESS : has
          MEMORY ||--o{ ORDER : places
          MEMORY ||--o{ STACK : "FIXED SIZE DATA"
          PROCESS ||--o{ HEAP : receives
          INVOICE ||--|{ HEAP : covers
          HEAP ||--|{ HEAP-ITEM : includes
          DATA-"" varied size items CATEGORY ||--|{ DATA : contains
          DATA ||--o{ DATA-ITEM : "data in"

```