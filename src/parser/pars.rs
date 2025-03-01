use crate::lexer::lex::Token;

enum StatementType {
    Return(Exp),
}

// --------------------
//    Basic node types
// --------------------
struct Program {
    function_declaration: FunctionDeclaration,
}

struct FunctionDeclaration {
    name: String,
    statement: Statement,
}

struct Statement {
    statement: StatementType,
}
struct Exp {
    constant: i32,
}

enum RuleType {
    Progam(Program),
    FunctionDeclaration(FunctionDeclaration),
    Statement(Statement),
    Exp(Exp),
}

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    node_type: RuleType,
}

impl Node {
    pub fn new(node_type: RuleType) -> Self {
        Node {
            left: None,
            right: None,
            node_type,
        }
    }

    pub fn add(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }
}
// ------------------
//   Main Ast struct
// ------------------
struct Ast {
    root: Option<Box<Node>>,
}

impl Ast {
    pub fn new() -> Self {
        Ast { root: None }
    }

    pub fn add_node(&mut self, node: Node) {
        match self.root {
            None => self.root = Some(Box::new(node)),
            Some(_) => {
                let mut head = self.root.take().unwrap();
                head.add(node);
                self.root = Some(head);
            }
        }
    }
}

pub struct Parser {
    ast: Ast,
}

impl Parser {
    pub fn new() -> Self {
        let ast = Ast::new();
        Parser { ast }
    }

    pub fn process_tokens(&self, tokens: Vec<Token>) {
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
