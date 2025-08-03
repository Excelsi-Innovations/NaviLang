# NaviLang - Declarative Flow Modeling Language

## ✨ Overview
NaviLang is a minimalist declarative language designed to describe, document, and organize logical flows of applications, processes, and services in a simple, intuitive, and visualizable way.

It can be used for:
- API modeling and documentation
- Process and workflow diagramming
- Service and dependency management
- Project and system planning
- Business process visualization

## 🎯 Objective
Enable anyone, even without deep technical knowledge, to describe complex logic using a language similar to natural text that can be automatically converted into diagrams and executable representations.

## 🧠 Key Concepts

### Isolated Contexts
NaviLang allows the division of large systems into **isolated contexts** (similar to *bounded contexts* in DDD). This approach:
- Facilitates focus on specific parts of the system
- Enables collaboration between multiple teams
- Reduces parsing and rendering time
- Ensures scalability and clarity in documentation
- Prevents naming conflicts across different domains

Example:
```navilang
CONTEXT Authentication {
  VAR User
  User DOES Login
  User CALLS AuthAPI
  AuthAPI RETURNS 200 OK
}

CONTEXT Payment {
  VAR Order
  Order CREATED BY User
  Order GOES TO Payment
  Payment USES Gateway
}
```

### Interactivity and Visual Integration
The graphical interface can be integrated with tools like Figma or a custom interface, allowing:
- Drag and drop block manipulation
- Real-time visualization and editing
- Export and import of NaviLang snippets
- Context navigation with modular focus
- Collaborative editing with version control

## 🖋️ Syntax

### Core Keywords
| Keyword | Function | Example |
|---------|----------|---------|
| `VAR` | Declares an entity, object, or state | `VAR User` |
| `GOES TO` | Defines a transition between two elements | `Login GOES TO Dashboard` |
| `CREATED BY` | Defines origin/creator of an entity | `Order CREATED BY User` |
| `IF` `THEN` | Expresses a condition | `IF Payment IS OK THEN Order GOES TO Delivery` |
| `CALLS` | Service or method invocation | `User CALLS AuthAPI` |
| `RECEIVES` | Expected response | `AuthAPI RECEIVES Token` |
| `DOES` | Action performed by entity | `User DOES Login` |
| `USES` | Dependency or resource utilization | `Payment USES Gateway` |
| `RETURNS` | Expected result from a call | `AuthAPI RETURNS 200 OK` |
| `WHEN` | Defines a trigger event | `WHEN Order IS Created THEN Payment GOES TO Processing` |
| `CONTEXT` | Defines an independent logical scope | `CONTEXT Auth { ... }` |
| `AFTER` | Sequential dependency | `Validation AFTER Input` |
| `BEFORE` | Prerequisite relationship | `Authorization BEFORE Payment` |
| `PARALLEL` | Concurrent execution | `PARALLEL EmailNotification AND SMSNotification` |

### Advanced Keywords
| Keyword | Function | Example |
|---------|----------|---------|
| `RETRY` | Retry mechanism | `API CALLS Service RETRY 3` |
| `TIMEOUT` | Time constraint | `Database TIMEOUT 5s` |
| `ASYNC` | Asynchronous operation | `ASYNC Email GOES TO Queue` |
| `BATCH` | Batch processing | `BATCH Orders GOES TO Processing` |
| `LOOP` | Iteration construct | `LOOP WHILE Orders EXIST` |
| `BREAK` | Loop termination | `IF Error THEN BREAK` |
| `CONTINUE` | Loop continuation | `IF Skip THEN CONTINUE` |

### Identifiers and Naming
- **Names** can contain letters, numbers, underscores, and accents
- Spaces can be used with quotes: `VAR "Admin Dashboard"`
- CamelCase and snake_case are both supported
- Context-scoped naming prevents conflicts

### Data Types and Constraints
```navilang
VAR User:Entity
VAR Count:Number
VAR Status:Enum[Active, Inactive, Pending]
VAR Email:String[REQUIRED]
VAR CreatedAt:DateTime[AUTO]
```

## 🎓 What is BNF (Backus-Naur Form)?
BNF is a formal notation used to define the **grammar of languages** (like programming languages or DSLs). It specifies which symbol combinations are valid in the language, helping in the construction of **parsers and interpreters**.

### NaviLang BNF Grammar
```bnf
<program> ::= <statement> | <statement> <program>

<statement> ::= <variable_declaration>
             | <transition>
             | <creation>
             | <invocation>
             | <reception>
             | <return>
             | <action>
             | <usage>
             | <conditional>
             | <event>
             | <context>

<variable_declaration> ::= "VAR" <identifier> [":"<type>]
<transition> ::= <identifier> "GOES TO" <identifier>
<creation> ::= <identifier> "CREATED BY" <identifier>
<invocation> ::= <identifier> "CALLS" <identifier>
<reception> ::= <identifier> "RECEIVES" <identifier>
<return> ::= <identifier> "RETURNS" <value>
<action> ::= <identifier> "DOES" <action_name>
<usage> ::= <identifier> "USES" <identifier>
<conditional> ::= "IF" <condition> "THEN" <statement>
<event> ::= "WHEN" <condition> "THEN" <statement>
<context> ::= "CONTEXT" <identifier> "{" <program> "}"

<identifier> ::= [a-zA-Z][a-zA-Z0-9_]* | "\"" [^"]+ "\""
<type> ::= [a-zA-Z][a-zA-Z0-9_]*
<condition> ::= <identifier> "IS" <value>
<value> ::= [a-zA-Z0-9_\"\s]+
<action_name> ::= [a-zA-Z][a-zA-Z0-9_\s]*
```

## 🛠️ Complete Examples

### E-commerce Flow
```navilang
CONTEXT UserManagement {
  VAR User:Entity
  VAR Session:Object
  
  User DOES Registration
  User CALLS AuthService
  AuthService RETURNS Session
  Session GOES TO UserDashboard
}

CONTEXT OrderProcessing {
  VAR Order:Entity
  VAR Payment:Service
  VAR Inventory:Service
  
  Order CREATED BY User
  Order CALLS Inventory
  IF Inventory IS Available THEN Order GOES TO Payment
  IF Inventory IS Unavailable THEN Order GOES TO Backorder
  
  Payment USES StripeGateway
  IF Payment IS Success THEN Order GOES TO Fulfillment
  IF Payment IS Failed THEN Order GOES TO Cancelled
}

CONTEXT Notifications {
  VAR EmailService:Service
  VAR SMSService:Service
  
  WHEN Order IS Confirmed THEN PARALLEL EmailService AND SMSService
  EmailService CALLS SendGrid
  SMSService CALLS TwilioAPI
}
```

### API Documentation Flow
```navilang
CONTEXT UserAPI {
  VAR GetUserEndpoint:Endpoint
  VAR CreateUserEndpoint:Endpoint
  
  GetUserEndpoint RECEIVES UserID
  GetUserEndpoint CALLS UserDatabase
  GetUserEndpoint RETURNS UserData
  
  CreateUserEndpoint RECEIVES UserPayload
  CreateUserEndpoint DOES Validation
  IF Validation IS Success THEN CreateUserEndpoint CALLS UserDatabase
  CreateUserEndpoint RETURNS 201 Created
}
```

### DevOps Pipeline
```navilang
CONTEXT CIPipeline {
  VAR CodeCommit:Event
  VAR BuildStage:Process
  VAR TestStage:Process
  VAR DeployStage:Process
  
  WHEN CodeCommit IS Pushed THEN BuildStage GOES TO Running
  BuildStage USES DockerContainer
  BuildStage AFTER Success GOES TO TestStage
  
  TestStage DOES UnitTests
  TestStage DOES IntegrationTests
  IF TestStage IS Success THEN DeployStage GOES TO Production
}
```

## 🔄 Expected Output Formats

### JSON Structure
```json
{
  "contexts": [
    {
      "name": "Authentication",
      "variables": ["User", "Session"],
      "flows": [
        {
          "from": "User",
          "action": "DOES",
          "to": "Login"
        }
      ]
    }
  ]
}
```

### Visualization Formats
- **Mermaid** diagrams for flowcharts and sequence diagrams
- **PlantUML** for UML-style documentation
- **D3.js** for interactive web visualizations
- **Graphviz DOT** for complex graph layouts
- **Navi View** for a custom visual editor and interactive diagrams
  
### Documentation Generation
- **OpenAPI/Swagger** specifications for APIs
- **Markdown** documentation with embedded diagrams
- **Confluence** pages with interactive elements
- **PDF** reports for stakeholder presentations

## 🧩 Use Cases

| Context | Application | Benefits |
|---------|-------------|----------|
| **APIs** | Describe endpoints, authentication, dependencies | Clear API contracts, automated documentation |
| **Projects** | Represent task dependencies between teams | Better project coordination, clear handoffs |
| **Systems** | Document microservices and integrations | System architecture clarity, dependency mapping |
| **Teams** | Create collaborative diagrams with versioning | Improved communication, shared understanding |
| **DevOps** | Map pipelines, stages, triggers, and events | Pipeline visualization, automation clarity |
| **Business** | Model business processes and workflows | Process optimization, compliance documentation |

## 💥 Problems NaviLang Solves

### Communication Gaps
- **Tech-to-Business**: Bridges the gap between technical and non-technical stakeholders
- **Cross-Team**: Enables clear communication across different departments
- **Documentation**: Reduces misunderstandings through visual representation

### Process Visibility
- **Distributed Systems**: Provides clear view of complex, distributed processes
- **Hidden Dependencies**: Makes implicit dependencies explicit
- **Bottlenecks**: Identifies process bottlenecks and optimization opportunities

### Documentation Challenges
- **Outdated Docs**: Living documentation that stays current with code
- **Scattered Information**: Centralized process documentation
- **Version Control**: Track changes in business logic and processes

### Scalability Issues
- **Large Teams**: Modular approach for enterprise-scale development
- **Complex Systems**: Break down complexity into manageable contexts
- **Maintenance**: Easier updates and modifications to existing flows

## 🚀 Future Extensions

### Type System Enhancement
```navilang
VAR Order:Entity {
  id: UUID[REQUIRED]
  status: OrderStatus[ENUM]
  amount: Money[POSITIVE]
  createdAt: DateTime[AUTO]
}
```

### Grouping and Organization
```navilang
GROUP PaymentAPIs {
  CONTEXT StripeIntegration { ... }
  CONTEXT PayPalIntegration { ... }
}
```

### Visual Annotations
```navilang
LABEL "Critical Path" ON Order GOES TO Payment
NOTE "This process requires manual review" ON AdminApproval
ICON "warning" ON ErrorHandling
COLOR "red" ON FailureStates
```

### Advanced Features
- **Execution Simulation**: Test flows before implementation
- **Test Generation**: Automatically generate test cases from flows
- **GPT Integration**: AI-assisted flow generation and optimization
- **Visual Editor**: Figma-style drag & drop interface
- **CI/CD Integration**: Validate flows in deployment pipelines
- **Real-time Monitoring**: Connect flows to actual system metrics
- **A/B Testing**: Model different flow variations
- **Performance Analysis**: Identify optimization opportunities

### Integration Capabilities
- **Version Control**: Git integration for flow versioning
- **Issue Tracking**: Link flows to Jira/GitHub issues
- **Monitoring Tools**: Connect to Datadog, New Relic metrics
- **Code Generation**: Generate boilerplate code from flows
- **Database Schema**: Generate database migrations from entity definitions

---

## 📁 Obsidian Organization Structure

```
NaviLang/
├── 📁 Core Concepts/
│   ├── Syntax Reference
│   ├── Context Isolation
│   └── Flow Modeling
├── 📁 Examples/
│   ├── E-commerce Flows
│   ├── API Documentation
│   └── DevOps Pipelines
├── 📁 Integration Guides/
│   ├── Mermaid Export
│   ├── OpenAPI Generation
│   └── CI/CD Integration
└── 📁 Advanced Features/
    ├── Type System
    ├── Visual Annotations
    └── Future Roadmap
```

## 🔗 Related Concepts
- [[Domain Driven Design]]
- [[Business Process Modeling]]
- [[API Documentation]]
- [[System Architecture]]
- [[DevOps Practices]]

---

**Next Steps**: Implement parser + interactive visual editor for isolated contexts with real-time collaboration features.