use std::fmt::Write;
use crate::{parser::ast::{Expression, Literal, Statement, Variable}, transpiler::{error::TranspilerResult, TranspilerContext, TranspilerImpl}};

#[derive(Default)]
pub struct BashTranspiler {
    ctx: TranspilerContext,
}

impl<'a> TranspilerImpl<'a> for BashTranspiler {
    fn ctx(&self) -> &TranspilerContext {
        &self.ctx
    }

    fn transpile_stmt(&self, statement: &'a Statement) -> TranspilerResult<String> {
        match statement {
            Statement::Variable(var) => self.transpile_var(var),
            _ => todo!()
        }
    }

    fn transpile_expr(&self, expression: &'a Expression) -> TranspilerResult<String> {
        match expression {
            // Expression::Block(block) => self.transpile_block(block),
            Expression::Literal(literal) => self.transpile_literal(literal),
            // Expression::Arithmetic(arithmetic) => self.transpile_arithmetic(arithmetic),
            _ => todo!()
        }
    }
}

impl BashTranspiler {
    fn transpile_var(&self, var: &Variable) -> TranspilerResult<String> {
        let mut s = String::new();

        write!(s, "{}", var.name)?;
        
        if let Some(value) = &var.value {
            let value = self.transpile_expr(&value.value)?;

            write!(s, "={}", value)?;
        }

        writeln!(s)?;

        Ok(s)
    }

    fn transpile_literal(&self, literal: &Literal) -> TranspilerResult<String> {
        Ok(match literal {
            Literal::String(string) => string.to_string(),
            Literal::Boolean(bool) => bool.to_string(),
            Literal::Integer(int) => int.to_string()
        })
    }

    // fn transpile_arithmetic(&self, arithmetic: )
}