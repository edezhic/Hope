use crate::core::*;
use core::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Being => write!(f, "="),
            Token::O(op) => write!(f, "O"),
            Token::Val(value) => write!(f, "V"),
            Token::C(case) => write!(f, "C"),
            Token::F(flow) => match flow {
                Flow::Break => write!(f, "."),
                Flow::ExpressionStart => write!(f, "("),
                Flow::ExpressionEnd => write!(f, ")"),
                _ => write!(f, "F"),
            },
            Token::Mod(modifier) => match modifier {
                Modifier::Binding => write!(f, "b"),
                Modifier::Selection => write!(f, "s"),
                Modifier::Targeting => write!(f, "t"),
                Modifier::None => write!(f, ""),
            },
            Token::S(set) => match set {
                Set::StructStart => write!(f, "{{"),
                Set::StructEnd => write!(f, "}}"),
                Set::ListStart => write!(f, "["),
                Set::ListEnd => write!(f, "]"),
            },
            Token::Term(_) => write!(f, "T"),
            Token::This => write!(f, "_"),
            Token::Cmd(_) => write!(f, "Cmd"),
        }
    }
}