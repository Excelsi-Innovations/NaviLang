flowchart TD
    A["NaviLang Source"] --> B["File Reader"]
    B --> C["Tokenizer"]
    C --> D["Lexical Analyzer"]
    D --> E["Token Stream"]
    E --> F["Syntax Validator"]
    F --> G["Parser Engine"]
    G --> H["AST Builder"]
    H --> I["Context Resolver"] & J["Type Checker"] & K["Flow Validator"] & L["Dependency Resolver"]
    I --> M["Semantic Analyzer"]
    J --> M
    K --> M
    L --> M
    M --> N["Diagram Generator"] & O["API Generator"] & P["Documentation Generator"] & Q["Code Generator"]
    N --> R["Flow Interpreter"]
    O --> S["Interactive Visualizer"]
    P --> T["Multi-format Exporter"]
    Q --> R
    R --> U["Final Output"]
    S --> U
    T --> U
    C -.-> ERR["Error Handler"]
    D -.-> ERR
    ERR -.-> B

     A:::input
     B:::input
     C:::lexical
     D:::lexical
     E:::lexical
     F:::syntax
     G:::syntax
     H:::syntax
     I:::semantic
     J:::semantic
     K:::semantic
     L:::semantic
     M:::semantic
     N:::generator
     O:::generator
     P:::generator
     Q:::generator
     R:::output
     S:::output
     T:::output
     U:::output
     ERR:::error
    classDef input fill:#9d4edd,stroke:#9d4edd,stroke-width:2px,color:#fff
    classDef lexical fill:#ff9500,stroke:#ff9500,stroke-width:2px,color:#fff
    classDef syntax fill:#00d4ff,stroke:#00d4ff,stroke-width:2px,color:#fff
    classDef semantic fill:#00ff88,stroke:#00ff88,stroke-width:2px,color:#000
    classDef generator fill:#ff6b6b,stroke:#ff6b6b,stroke-width:2px,color:#fff
    classDef output fill:#ffd93d,stroke:#ffd93d,stroke-width:2px,color:#000
    classDef error fill:#ff4757,stroke:#ff4757,stroke-width:2px,color:#fff
    style F fill:#2962FF
    style G fill:#2962FF
    style H fill:#2962FF


