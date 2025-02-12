use crate::{to_expr_kind, ast};

ast!(ShellCommand(String, Option<String>));
to_expr_kind!(ShellCommand);