use std::{cell::Cell, fmt};

use super::Statement;

/// Displays an item surrounded by brackets, e.g. `[foo]`
pub struct DisplayBracketed<T>(pub T);
impl<T: fmt::Display> fmt::Display for DisplayBracketed<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", &self.0)
    }
}

/// Displays a slice of items surrounded by brackets, e.g. `[foo]`
pub struct DisplayList<'a, T>(pub &'a [T]);
impl<'a, T: fmt::Display> fmt::Display for DisplayList<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", DisplayCsv::new(self.0.iter()))
    }
}

/// Displays an item surrounded by parentheses, e.g. `(foo)`
pub struct DisplayParenthesized<T>(pub T);
impl<T: fmt::Display> fmt::Display for DisplayParenthesized<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", &self.0)
    }
}

/// Displays a slice of items surrounded by parentheses, e.g. `(foo)`
pub struct DisplayTuple<'a, T>(pub &'a [T]);
impl<'a, T: fmt::Display> fmt::Display for DisplayTuple<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", DisplayCsv::new(self.0.iter()))
    }
}

/// Displays one or more items separated by commas, e.g. `foo, bar`
pub struct DisplayCsv<T>(Cell<Option<T>>);
impl<T, I> DisplayCsv<I>
where
    T: fmt::Display,
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> Self {
        Self(Cell::new(Some(iter)))
    }
}
impl<T, I> fmt::Display for DisplayCsv<I>
where
    T: fmt::Display,
    I: Iterator<Item = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iter = self.0.take().unwrap();
        for (i, item) in iter.enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

pub struct DisplayStatement<'a> {
    pub statement: &'a Statement,
    pub indent: usize,
}
impl DisplayStatement<'_> {
    const INDENT: &str = "    ";

    fn write_indent(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.indent {
            f.write_str(Self::INDENT)?;
        }
        Ok(())
    }
}
impl<'a> fmt::Display for DisplayStatement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indent(f)?;
        match self.statement {
            Statement::Let(ref expr) => {
                writeln!(f, "let {} = {}", expr.name, expr.value)?;
                for statement in expr.body.iter() {
                    writeln!(f, "{}", statement.display(self.indent))?;
                }
                Ok(())
            }
            Statement::Enforce(ref expr) => {
                write!(f, "enf {}", expr)
            }
            Statement::EnforceIf(ref expr, ref selector) => {
                write!(f, "enf {} when {}", expr, selector)
            }
            Statement::EnforceAll(ref expr) => {
                write!(f, "enf {}", expr)
            }
            Statement::Expr(ref expr) => write!(f, "{}", expr),
        }
    }
}
