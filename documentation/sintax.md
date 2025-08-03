# NaviLang - Guia de Sintaxe Completo

## 📖 Introdução

Este documento apresenta a sintaxe completa da linguagem NaviLang, uma linguagem declarativa para modelagem de fluxos de processos, APIs, workflows e documentação de sistemas.

NaviLang foi projetada para ser:
- **Intuitiva**: Sintaxe próxima à linguagem natural
- **Declarativa**: Descreve "o que" ao invés de "como"
- **Modular**: Contextos isolados para escalabilidade
- **Visual**: Gera automaticamente diagramas e documentação

## 🏗️ Estrutura Geral

### Programa Base
Um programa NaviLang consiste em uma ou mais declarações que podem estar dentro de contextos ou no escopo global:

```navilang
# Declaração global
VAR GlobalConfig

# Contexto isolado
CONTEXT Authentication {
    VAR User
    User GOES TO Dashboard
}
```

## 🔤 Identificadores e Convenções

### Regras de Nomenclatura
- **Caracteres permitidos**: Letras (a-z, A-Z), números (0-9), underscore (_), acentos
- **Início obrigatório**: Deve começar com letra
- **Case sensitivity**: Sensitivo a maiúsculas/minúsculas
- **Convenções suportadas**: CamelCase, snake_case, PascalCase

### Identificadores com Espaços
Para nomes com espaços, use aspas duplas:
```navilang
VAR "Admin Dashboard"
VAR "User Profile Settings"
"Payment Gateway" CALLS "External API"
```

### Comentários
```navilang
# Comentário de linha única
# VAR DisabledFeature

CONTEXT Example {
    # Comentário dentro de contexto
    VAR User  # Comentário no final da linha
}
```

## 🎯 Contextos (CONTEXT)

### Sintaxe Básica
```navilang
CONTEXT ContextName {
    # Declarações do contexto
}
```

### Características dos Contextos
- **Isolamento**: Previnem conflitos de nomenclatura
- **Modularidade**: Permitem divisão lógica do sistema
- **Escopo**: Variáveis e entidades são locais ao contexto
- **Aninhamento**: Contextos podem ser aninhados (implementação futura)

### Exemplos
```navilang
CONTEXT UserManagement {
    VAR User:Entity
    VAR Session:Object
    
    User DOES Registration
    Session GOES TO Dashboard
}

CONTEXT PaymentProcessing {
    VAR Order:Entity
    VAR Payment:Service
    
    Order USES Payment
    Payment RETURNS Receipt
}
```

## 📊 Declarações de Variáveis (VAR)

### Sintaxe Básica
```navilang
VAR VariableName
VAR VariableName:Type
VAR VariableName:Type[Constraints]
```

### Tipos de Dados

#### Tipos Primitivos
```navilang
VAR UserName:String
VAR UserAge:Number
VAR IsActive:Boolean
VAR CreatedAt:DateTime
```

#### Tipos Complexos
```navilang
VAR User:Entity
VAR AuthService:Service
VAR UserList:Array
VAR Config:Object
VAR Database:Repository
VAR API:Endpoint
```

#### Tipos com Enumeração
```navilang
VAR Status:Enum[Active, Inactive, Pending]
VAR Priority:Enum[Low, Medium, High, Critical]
VAR Role:Enum[User, Admin, Moderator]
```

### Constraints (Restrições)
```navilang
VAR Email:String[REQUIRED]
VAR Password:String[REQUIRED, MIN_LENGTH=8]
VAR Age:Number[MIN=18, MAX=120]
VAR CreatedAt:DateTime[AUTO]
VAR ID:String[UNIQUE, AUTO_GENERATED]
```

#### Constraints Disponíveis
- `REQUIRED`: Campo obrigatório
- `AUTO`: Valor gerado automaticamente
- `UNIQUE`: Valor único no contexto
- `MIN_LENGTH`, `MAX_LENGTH`: Limites de comprimento
- `MIN`, `MAX`: Valores mínimo e máximo
- `PATTERN`: Expressão regular (implementação futura)

## 🔄 Relacionamentos e Transições

### Transições Básicas (GOES TO)
```navilang
Login GOES TO Dashboard
User GOES TO ProfilePage
Order GOES TO PaymentProcess
```

### Criação e Origem (CREATED BY)
```navilang
Order CREATED BY User
Session CREATED BY AuthService
Report CREATED BY System
```

### Chamadas e Invocações (CALLS)
```navilang
User CALLS AuthAPI
Frontend CALLS BackendService
Service CALLS Database
```

### Recepção de Dados (RECEIVES)
```navilang
AuthAPI RECEIVES Credentials
PaymentService RECEIVES OrderData
User RECEIVES Notification
```

### Retornos (RETURNS)
```navilang
AuthAPI RETURNS Token
PaymentService RETURNS 200 OK
Database RETURNS UserData
Service RETURNS Error 404
```

### Ações (DOES)
```navilang
User DOES Login
System DOES Validation
Service DOES Processing
Admin DOES Configuration
```

### Dependências (USES)
```navilang
Payment USES Gateway
Frontend USES API
Service USES Database
Process USES ExternalLibrary
```

## 🔀 Controle de Fluxo

### Condicionais (IF/THEN)
```navilang
IF Payment IS OK THEN Order GOES TO Delivery
IF User IS Authenticated THEN Dashboard GOES TO MainPage
IF Error IS Critical THEN System DOES Shutdown
```

#### Operadores de Comparação
```navilang
IF Status IS Active THEN Process GOES TO Next
IF Count IS GREATER THAN 100 THEN Alert GOES TO Admin
IF Age IS LESS THAN 18 THEN Access GOES TO Restricted
IF Role IS NOT Admin THEN Feature GOES TO Disabled
```

### Eventos (WHEN/THEN)
```navilang
WHEN Order IS Created THEN Payment GOES TO Processing
WHEN User IS Registered THEN Email GOES TO Welcome
WHEN Error IS Detected THEN System DOES Recovery
```

### Sequenciamento

#### Dependências Sequenciais
```navilang
Validation AFTER Input
Authorization BEFORE Payment
Cleanup AFTER Processing
Backup BEFORE Update
```

#### Execução Paralela
```navilang
PARALLEL EmailNotification AND SMSNotification
PARALLEL DataValidation AND UserAuthentication
PARALLEL LogEntry AND MetricsUpdate
```

## 🔄 Controles Avançados

### Loops e Iterações
```navilang
LOOP WHILE Orders EXIST
    Order GOES TO Processing
    IF Order IS Processed THEN CONTINUE
    IF Error IS Critical THEN BREAK
END LOOP

LOOP FOR EACH User IN UserList
    User DOES Validation
    IF User IS Valid THEN User GOES TO Approved
END LOOP
```

### Mecanismos de Retry
```navilang
API CALLS Service RETRY 3
Database CALLS Connection RETRY 5 TIMEOUT 30s
Payment CALLS Gateway RETRY 2 DELAY 5s
```

### Operações Assíncronas
```navilang
ASYNC Email GOES TO Queue
ASYNC LogEntry GOES TO Database
ASYNC Notification GOES TO Users
```

### Processamento em Lote
```navilang
BATCH Orders GOES TO Processing
BATCH Users RECEIVES Notification
BATCH Files GOES TO Archive
```

### Timeouts
```navilang
Database TIMEOUT 5s
API CALLS Service TIMEOUT 10s
User WAITS Response TIMEOUT 30s
```

## 📋 Exemplos Práticos

### Sistema de Autenticação
```navilang
CONTEXT Authentication {
    VAR User:Entity
    VAR Credentials:Object
    VAR Session:Object
    VAR AuthService:Service
    
    User DOES Login
    User CALLS AuthService
    AuthService RECEIVES Credentials
    
    IF Credentials IS Valid THEN AuthService RETURNS Session
    IF Credentials IS Invalid THEN AuthService RETURNS Error
    
    WHEN Session IS Created THEN User GOES TO Dashboard
    WHEN Error IS Returned THEN User GOES TO LoginPage
}
```

### Processamento de Pedidos
```navilang
CONTEXT OrderProcessing {
    VAR Order:Entity
    VAR Payment:Service
    VAR Inventory:Service
    VAR DeliveryService:Service
    
    Order CREATED BY User
    Order USES Payment
    
    PARALLEL {
        Payment CALLS Gateway
        Inventory CALLS StockService
    }
    
    IF Payment IS Success AND Inventory IS Available 
    THEN Order GOES TO Processing
    
    Order USES DeliveryService
    DeliveryService RETURNS TrackingNumber
    
    WHEN Order IS Delivered THEN User RECEIVES Notification
}
```

### Pipeline CI/CD
```navilang
CONTEXT CIPipeline {
    VAR CodeCommit:Event
    VAR BuildStage:Service
    VAR TestStage:Service
    VAR DeployStage:Service
    
    WHEN CodeCommit IS Pushed THEN BuildStage GOES TO Active
    
    BuildStage DOES Compilation
    IF BuildStage IS Success THEN TestStage GOES TO Active
    IF BuildStage IS Failed THEN Pipeline GOES TO Failed
    
    TestStage DOES UnitTests
    TestStage DOES IntegrationTests
    
    IF TestStage IS Success THEN DeployStage GOES TO Production
    IF TestStage IS Failed THEN Pipeline GOES TO Failed
    
    DeployStage USES KubernetesCluster
    DeployStage RETURNS DeploymentStatus
}
```

### API RESTful
```navilang
CONTEXT UserAPI {
    VAR GetUserEndpoint:Endpoint
    VAR CreateUserEndpoint:Endpoint
    VAR UpdateUserEndpoint:Endpoint
    VAR DeleteUserEndpoint:Endpoint
    VAR UserDatabase:Repository
    
    GetUserEndpoint CALLS UserDatabase
    GetUserEndpoint RETURNS UserData
    
    CreateUserEndpoint RECEIVES UserData
    CreateUserEndpoint CALLS UserDatabase
    CreateUserEndpoint RETURNS 201 Created
    
    UpdateUserEndpoint RECEIVES UserData
    UpdateUserEndpoint CALLS UserDatabase
    IF User IS Found THEN UpdateUserEndpoint RETURNS 200 OK
    IF User IS NOT Found THEN UpdateUserEndpoint RETURNS 404 NotFound
    
    DeleteUserEndpoint CALLS UserDatabase
    DeleteUserEndpoint RETURNS 204 NoContent
}
```

## 🔧 Palavras-chave por Categoria

### Declarações Básicas
- `VAR` - Declaração de variável/entidade
- `CONTEXT` - Definição de contexto isolado

### Relacionamentos
- `GOES TO` - Transição/navegação
- `CREATED BY` - Relação de criação
- `CALLS` - Invocação de serviço/método
- `RECEIVES` - Recepção de dados
- `RETURNS` - Retorno de resultado
- `DOES` - Execução de ação
- `USES` - Utilização de dependência

### Controle de Fluxo
- `IF` / `THEN` - Condicionais
- `WHEN` / `THEN` - Eventos/triggers
- `AFTER` / `BEFORE` - Sequenciamento
- `PARALLEL` - Execução concorrente
- `LOOP` / `WHILE` / `FOR EACH` - Iterações
- `BREAK` / `CONTINUE` - Controle de loop

### Operações Avançadas
- `RETRY` - Mecanismo de repetição
- `TIMEOUT` - Limite de tempo
- `ASYNC` - Operação assíncrona
- `BATCH` - Processamento em lote
- `DELAY` - Atraso/pausa

### Operadores
- `IS` - Comparação de igualdade
- `IS NOT` - Comparação de diferença
- `GREATER THAN` - Maior que
- `LESS THAN` - Menor que
- `AND` - Operador lógico E
- `OR` - Operador lógico OU

## 📝 Convenções e Boas Práticas

### Nomenclatura
```navilang
# ✅ Bom - Nomes descritivos
VAR UserAuthenticationService
VAR PaymentProcessingStatus

# ❌ Evitar - Nomes genéricos
VAR Service
VAR Data
```

### Organização de Contextos
```navilang
# ✅ Contextos bem definidos e focados
CONTEXT UserAuthentication { ... }
CONTEXT PaymentProcessing { ... }
CONTEXT NotificationSystem { ... }

# ❌ Contextos muito amplos
CONTEXT Everything { ... }
```

### Estruturação de Fluxos
```navilang
# ✅ Fluxo claro e lógico
User DOES Login
User CALLS AuthService
AuthService RETURNS Token
User GOES TO Dashboard

# ✅ Uso adequado de condicionais
IF Token IS Valid THEN User GOES TO Dashboard
IF Token IS Invalid THEN User GOES TO LoginError
```

## 🔗 Referências Cruzadas

### Entre Contextos
```navilang
CONTEXT Authentication {
    VAR User:Entity
    User GOES TO "UserManagement.Dashboard"
}

CONTEXT UserManagement {
    VAR Dashboard:Page
    Dashboard RECEIVES "Authentication.User"
}
```

### Importação (Planejado)
```navilang
IMPORT CommonTypes FROM "shared/types.navi"
IMPORT UserManagement FROM "modules/users.navi"
```

## 📊 Formato de Saída

NaviLang pode ser compilado para múltiplos formatos:

### Diagramas
- **Mermaid**: Fluxogramas e diagramas de sequência
- **PlantUML**: Diagramas UML
- **GraphViz**: Grafos complexos
- **D3.js**: Visualizações interativas

### Documentação
- **Markdown**: Documentação estruturada
- **HTML**: Páginas web interativas
- **PDF**: Relatórios para stakeholders
- **OpenAPI/Swagger**: Especificações de API

### Dados Estruturados
- **JSON**: Representação estruturada
- **YAML**: Configurações legíveis
- **XML**: Integração com sistemas legados

## 📚 Gramática BNF Formal

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
             | <loop>
             | <parallel>
             | <async>
             | <batch>

<variable_declaration> ::= "VAR" <identifier> [":"<type>] [<constraints>]
<transition> ::= <identifier> "GOES TO" <identifier>
<creation> ::= <identifier> "CREATED BY" <identifier>
<invocation> ::= <identifier> "CALLS" <identifier> [<retry>] [<timeout>]
<reception> ::= <identifier> "RECEIVES" <identifier>
<return> ::= <identifier> "RETURNS" <value>
<action> ::= <identifier> "DOES" <action_name>
<usage> ::= <identifier> "USES" <identifier>
<conditional> ::= "IF" <condition> "THEN" <statement>
<event> ::= "WHEN" <condition> "THEN" <statement>
<context> ::= "CONTEXT" <identifier> "{" <program> "}"
<loop> ::= "LOOP" <loop_condition> <program> "END LOOP"
<parallel> ::= "PARALLEL" <statement_list>
<async> ::= "ASYNC" <statement>
<batch> ::= "BATCH" <statement>

<identifier> ::= [a-zA-Z][a-zA-Z0-9_]* | "\"" [^"]+ "\""
<type> ::= [a-zA-Z][a-zA-Z0-9_]* ["[" <enum_values> "]"]
<constraints> ::= "[" <constraint_list> "]"
<condition> ::= <identifier> <operator> <value>
<value> ::= [a-zA-Z0-9_\"\s]+
<action_name> ::= [a-zA-Z][a-zA-Z0-9_\s]*
<operator> ::= "IS" | "IS NOT" | "GREATER THAN" | "LESS THAN"
<retry> ::= "RETRY" <number>
<timeout> ::= "TIMEOUT" <time_value>
```

---

**Nota**: Esta é a especificação da versão atual do NaviLang. Recursos marcados como "planejado" ou "implementação futura" serão adicionados em versões posteriores.
