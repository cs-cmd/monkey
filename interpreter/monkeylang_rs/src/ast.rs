use crate::token::{Token, TokenType};
// Node types
pub trait BaseNode {
    fn token_literal(&self) -> Option<&String>;
}

pub trait Statement: BaseNode {
    fn statement_node(&self);
}

pub trait Expression: BaseNode {
    fn expression_node(&self);
}
//*****************************************************************************

// Rust requires knowledge of size of data-types at compile time.
// Use a `Box` to let Rust know the size of `Statement`s, since it
// is a trait.
// The `Program` struct will be the root of the program and be made
// up of `statement`s
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        return Program { statements: vec![] };
    }

    pub fn add_statement(&mut self, st: Box<dyn Statement>) -> () {
        self.statements.push(st);
    }
}

impl BaseNode for Program {
    fn token_literal(&self) -> Option<&String> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return None;
        }
    }
}
//*****************************************************************************
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn new(name: String) -> Identifier {
        return Identifier {
            token: Token::new_with_ref(TokenType::IDENT, &name),
            value: name,
        };
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl BaseNode for Identifier {
    fn token_literal(&self) -> Option<&String> {
        return Some(&self.token.literal);
    }
}
//*****************************************************************************
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(t: Token, n: String, v: Box<dyn Expression>) -> LetStatement {
        return LetStatement {
            token: t,
            name: Identifier::new(n),
            value: v,
        };
    }
}

impl BaseNode for LetStatement {
    fn token_literal(&self) -> Option<&String> {
        return Some(&self.token.literal);
    }
}

impl Expression for LetStatement {
    fn expression_node(&self) {}
}
//*****************************************************************************
