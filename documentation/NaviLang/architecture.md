# NaviLang Core Architecture

## 🏗️ System Overview

The NaviLang compiler follows a traditional multi-stage compilation pipeline, transforming human-readable declarative flow descriptions into multiple output formats including diagrams, documentation, and executable code.

## 📊 Main Architecture Pipeline

```mermaid
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
```

## 🔍 Pipeline Stages Detailed

### 1. Input Stage (Purple)
- **NaviLang Source**: Raw .navi files containing declarative flow descriptions
- **File Reader**: Handles file I/O, supports multiple input formats (.navi, .txt, stdin)

### 2. Lexical Analysis Stage (Orange)
- **Tokenizer**: Breaks source text into tokens using the `logos` crate
- **Lexical Analyzer**: Validates tokens and handles lexical errors
- **Token Stream**: Structured sequence of validated tokens

### 3. Syntax Analysis Stage (Blue)
- **Syntax Validator**: Ensures token sequences follow NaviLang grammar
- **Parser Engine**: Recursive descent parser implementing BNF grammar
- **AST Builder**: Constructs Abstract Syntax Tree representation

### 4. Semantic Analysis Stage (Green)
- **Context Resolver**: Manages isolated contexts and scoping rules
- **Type Checker**: Validates entity types, constraints, and compatibility
- **Flow Validator**: Detects dead ends, infinite loops, and logical errors
- **Dependency Resolver**: Analyzes execution order and dependencies
- **Semantic Analyzer**: Orchestrates all semantic checks and builds semantic model

### 5. Code Generation Stage (Red)
- **Diagram Generator**: Creates Mermaid, GraphViz, PlantUML outputs
- **API Generator**: Produces OpenAPI/Swagger specifications
- **Documentation Generator**: Generates Markdown, HTML, PDF documentation
- **Code Generator**: Creates executable code templates and boilerplate

### 6. Output Stage (Yellow)
- **Flow Interpreter**: Executes and simulates flows
- **Interactive Visualizer**: Provides real-time visualization capabilities
- **Multi-format Exporter**: Handles various output formats and targets
- **Final Output**: Unified output delivery system

### 7. Error Handling (Red)
- **Error Handler**: Global error management with recovery mechanisms
- Provides precise error locations and helpful suggestions
- Supports graceful degradation for batch processing

## 🗂️ Module Architecture

```mermaid
graph TD
    subgraph "navilang-core Rust Project"
        CLI["CLI Interface (main.rs)"]
        LIB["Public API (lib.rs)"]
        
        subgraph "Core Modules"
            READER["reader/"]
            LEXER["lexer/"]
            PARSER["parser/"]
            ANALYZER["analyzer/"]
            GENERATOR["generator/"]
        end
        
        subgraph "Supporting Modules"
            ERROR["error/"]
            UTILS["utils/"]
        end
        
        subgraph "External Dependencies"
            LOGOS["logos - Tokenization"]
            SERDE["serde - Serialization"]
            CLAP["clap - CLI Framework"]
            MIETTE["miette - Error Reporting"]
            PETGRAPH["petgraph - Graph Operations"]
        end
    end
    
    CLI --> LIB
    LIB --> READER
    LIB --> LEXER
    LIB --> PARSER
    LIB --> ANALYZER
    LIB --> GENERATOR
    
    READER --> ERROR
    LEXER --> ERROR
    LEXER --> LOGOS
    PARSER --> ERROR
    ANALYZER --> PETGRAPH
    GENERATOR --> SERDE
    CLI --> CLAP
    ERROR --> MIETTE
    
    All_Modules --> UTILS
    
    classDef core fill:#4fc3f7,stroke:#0277bd,stroke-width:2px
    classDef support fill:#81c784,stroke:#388e3c,stroke-width:2px
    classDef external fill:#f48fb1,stroke:#c2185b,stroke-width:2px
    
    class READER,LEXER,PARSER,ANALYZER,GENERATOR core
    class ERROR,UTILS support
    class LOGOS,SERDE,CLAP,MIETTE,PETGRAPH external
```

## 🔄 Data Flow

```mermaid
sequenceDiagram
    participant User as User/CLI
    participant FR as File Reader
    participant LEX as Lexer
    participant PARSE as Parser
    participant SEM as Semantic Analyzer
    participant GEN as Generator
    participant OUT as Output System
    
    User->>FR: navilang parse file.navi
    FR->>FR: Read source file
    FR->>LEX: Raw text content
    
    LEX->>LEX: Tokenize (logos)
    LEX->>PARSE: Token stream
    
    PARSE->>PARSE: Build AST
    PARSE->>SEM: AST nodes
    
    par Parallel Semantic Analysis
        SEM->>SEM: Context resolution
    and
        SEM->>SEM: Type checking
    and
        SEM->>SEM: Flow validation
    and
        SEM->>SEM: Dependency analysis
    end
    
    SEM->>GEN: Semantic model
    
    par Multiple Output Generation
        GEN->>GEN: Generate Mermaid
    and
        GEN->>GEN: Generate OpenAPI
    and
        GEN->>GEN: Generate Documentation
    and
        GEN->>GEN: Generate Code Templates
    end
    
    GEN->>OUT: Generated outputs
    OUT->>User: Files (diagrams, docs, code)
    
    Note over LEX,OUT: Error handling at each stage
    Note over SEM: Concurrent semantic checks
    Note over GEN: Multiple format support
```

## 🎯 Design Principles

### 1. **Modularity**
- Each stage is a separate, testable module
- Clear interfaces between components
- Plugin architecture for generators

### 2. **Error-First Design**
- Comprehensive error handling at every stage
- Precise error locations with helpful suggestions
- Graceful degradation for partial failures

### 3. **Performance**
- Parallel semantic analysis where possible
- Incremental parsing for large files
- Memory-efficient AST representation

### 4. **Extensibility**
- Plugin system for custom output generators
- Support for new keywords and syntax extensions
- Modular generator architecture

### 5. **Type Safety**
- Strong typing throughout Rust implementation
- Compile-time guarantees for correctness
- Safe memory management

## 🧩 Core Data Structures

### AST Representation
```rust
pub enum ASTNode {
    Program { contexts: Vec<Context> },
    Context { name: String, statements: Vec<Statement> },
    Statement { kind: StatementKind, span: Span },
}

pub enum StatementKind {
    VarDeclaration { name: String, type_info: Option<TypeInfo> },
    Transition { from: String, to: String },
    Action { subject: String, action: String, object: Option<String> },
    Conditional { condition: Condition, then_stmt: Box<Statement> },
    // ... other statement types
}
```

### Semantic Model
```rust
pub struct SemanticModel {
    pub contexts: HashMap<String, ContextModel>,
    pub dependencies: DependencyGraph,
    pub type_bindings: HashMap<String, TypeInfo>,
    pub flow_graph: FlowGraph,
}
```

### Error Types
```rust
pub enum NaviLangError {
    SyntaxError { message: String, span: SourceSpan },
    SemanticError { message: String, span: SourceSpan },
    TypeError { expected: String, found: String, span: SourceSpan },
    IoError(std::io::Error),
}
```

## 🚀 Implementation Strategy

### Phase 1: Foundation (Weeks 1-4)
- Basic CLI and file reading
- Complete tokenizer with error handling
- Simple parser for basic statements
- AST representation

### Phase 2: Advanced Parsing (Weeks 5-8)
- Complex statement parsing (contexts, conditionals)
- Enhanced error reporting
- Complete AST with metadata

### Phase 3: Semantic Analysis (Weeks 9-12)
- Context resolution and type checking
- Flow validation and dependency analysis
- Semantic model construction

### Phase 4: Code Generation (Weeks 13-16)
- Multiple output format generators
- Documentation and diagram generation
- Integration and optimization

## 📋 Quality Assurance

### Testing Strategy
- **Unit Tests**: Each module thoroughly tested
- **Integration Tests**: End-to-end compilation pipeline
- **Performance Tests**: Large file handling and memory usage
- **Regression Tests**: Prevent breaking changes

### Error Handling
- Precise error locations with line/column numbers
- Helpful error messages with suggestions
- Error recovery for batch processing
- Comprehensive error reporting with `miette`

### Documentation
- API documentation with `rustdoc`
- Architecture documentation (this file)
- User guides and examples
- Contributing guidelines

---

**Related Documents:**
- [Product Requirements Document](../../docs/PRD_NaviLang_Core.md)
- [Development Roadmap](../../docs/Development_Roadmap_Phase1.md)
- [NaviLang Overview](./Overview.md)


