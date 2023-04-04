use super::{ComprehensionContext, Expression, Identifier, Iterable, TraceBindingAccess, Variable};
use std::fmt::Display;

// BOUNDARY STATEMENTS
// ================================================================================================

#[derive(Debug, Eq, PartialEq)]
pub enum BoundaryStmt {
    Constraint(BoundaryConstraint),
    ConstraintComprehension(BoundaryConstraint, ComprehensionContext),
    Variable(Variable),
}

/// Stores the expression corresponding to the boundary constraint.
#[derive(Debug, Eq, PartialEq)]
pub struct BoundaryConstraint {
    access: TraceBindingAccess,
    boundary: Boundary,
    value: Expression,
}

impl BoundaryConstraint {
    pub fn new(access: TraceBindingAccess, boundary: Boundary, value: Expression) -> Self {
        Self {
            access,
            boundary,
            value,
        }
    }

    pub fn access(&self) -> &TraceBindingAccess {
        &self.access
    }

    pub fn boundary(&self) -> Boundary {
        self.boundary
    }

    /// Returns the constraint's value expression.
    pub fn value(&self) -> &Expression {
        &self.value
    }

    pub fn into_parts(self) -> (Boundary, TraceBindingAccess, Expression) {
        (self.boundary, self.access, self.value)
    }
}

/// Describes the type of boundary in the boundary constraint.
#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum Boundary {
    First,
    Last,
}

impl Display for Boundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Boundary::First => write!(f, "first boundary"),
            Boundary::Last => write!(f, "last boundary"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BoundaryConstraintComprehension {
    access: TraceBindingAccess,
    boundary: Boundary,
    expr: Expression,
    context: ComprehensionContext,
}

impl BoundaryConstraintComprehension {
    pub fn new(
        access: TraceBindingAccess,
        boundary: Boundary,
        expr: Expression,
        context: ComprehensionContext,
    ) -> Self {
        Self {
            access,
            boundary,
            expr,
            context,
        }
    }

    pub fn access(&self) -> &TraceBindingAccess {
        &self.access
    }

    pub fn boundary(&self) -> Boundary {
        self.boundary
    }

    /// Returns the expression that is evaluated for each member of the list.
    pub fn expression(&self) -> &Expression {
        &self.expr
    }

    /// Returns the context of the boundary constraint comprehension.
    pub fn context(&self) -> &[(Identifier, Iterable)] {
        &self.context
    }
}