use crate::{as_stmt_kind, ast};

ast!(Include(String));
as_stmt_kind!(Include);