---
references:
  - "File: /src/main.rs"
generationTime: 2026-06-18T19:46:39.207Z
---
flowchart TD
    A["Start main()"] --> B["Read input string<br/>(arla)"]
    B --> C["Parse to usize"]
    C --> D{Parse<br/>successful?}
    D -->|Yes| E["Print index"]
    D -->|No| F["Print error<br/>and exit"]
    E --> G["Read guess string"]
    G --> H["Generate secret number<br/>1-100"]
    H --> I["Parse guess to u32"]
    I --> J{Parse<br/>successful?}
    J -->|Yes| K["Store parsed<br/>number in guess"]
    J -->|No| L["Print Error<br/>Use default 33"]
    K --> M["Compare guess<br/>with secret_number"]
    L --> M
    M --> N{Which<br/>condition?}
    N -->|Greater| O["Print: This is<br/>greater value"]
    N -->|Less| P["Print: This is<br/>a less value"]
    N -->|Equal| Q["Print: This<br/>is great"]
    O --> R["End main()"]
    P --> R
    Q --> R
