mod parser;
mod codegen;
mod evaluator;

use std::fmt::{Display, Formatter};
use crate::helper::DynError;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) =>
                write!(f, "split {:>04} {:>04}", addr1, addr2),
        }
    }
}

pub fn do_matching(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, is_depth)?)
}

pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST: {:?}", ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::engine::do_matching;

    #[test]
    fn test_matching() {
        assert!(do_matching("+b", "bbb", true).is_err());
        assert!(do_matching("*b", "bbb", true).is_err());
        assert!(do_matching("|b", "bbb", true).is_err());
        assert!(do_matching("?b", "bbb", true).is_err());

        assert!(do_matching("abc|def", "def", true).unwrap());
        assert!(do_matching("a*", "aa", true).unwrap());
        // assert!(do_matching("(abc)*", "abcabc", true).unwrap());
        // assert!(do_matching("(ab|cd)+", "abcdcd", true).unwrap());
    }
}