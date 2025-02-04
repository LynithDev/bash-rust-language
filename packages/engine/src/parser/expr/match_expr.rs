use std::{collections::HashMap, rc::Rc};

use super::{literal::Literal, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchExpr(pub Expression, pub HashMap<Literal, Rc<Expression>>);