use super::{
    ConstantValueExpr, ConstraintBuilder, Expression, ListFoldingValueExpr, SemanticError,
    SymbolType, TraceAccess, VariableValueExpr, CURRENT_ROW,
};

// LIST FOLDING
// ================================================================================================

impl ConstraintBuilder {
    /// Builds a list of expressions from a list folding value. The list folding value can be either a
    /// vector, a list comprehension, or an identifier that refers to a vector.
    ///
    /// # Errors
    /// Returns an error if:
    /// - the list folding value is an identifier that does not exist in the symbol table
    /// - the list folding value is an identifier that does not refer to a vector
    pub fn build_list_from_list_folding_value(
        &self,
        lf_value_type: &ListFoldingValueExpr,
    ) -> Result<Vec<Expression>, SemanticError> {
        match lf_value_type {
            ListFoldingValueExpr::Identifier(ident) => {
                let symbol = self.symbol_table.get_symbol(ident.name())?;
                match symbol.symbol_type() {
                    SymbolType::ConstantBinding(ConstantValueExpr::Vector(list)) => {
                        Ok(list.iter().map(|value| Expression::Const(*value)).collect())
                    }
                    SymbolType::VariableBinding(variable_type) => {
                        if let VariableValueExpr::Vector(list) = variable_type {
                            Ok(list.clone())
                        } else {
                            Err(SemanticError::invalid_list_folding(
                                lf_value_type,
                                symbol.symbol_type(),
                            ))
                        }
                    }
                    SymbolType::TraceBinding(columns) => {
                        if columns.size() > 1 {
                            let trace_segment = columns.trace_segment();
                            Ok((0..columns.size())
                                .map(|i| {
                                    Expression::TraceAccess(TraceAccess::new(
                                        trace_segment,
                                        columns.offset() + i,
                                        1,
                                        CURRENT_ROW,
                                    ))
                                })
                                .collect())
                        } else {
                            Err(SemanticError::invalid_list_folding(
                                lf_value_type,
                                symbol.symbol_type(),
                            ))
                        }
                    }
                    _ => Err(SemanticError::invalid_list_folding(
                        lf_value_type,
                        symbol.symbol_type(),
                    )),
                }
            }
            ListFoldingValueExpr::Vector(vector) => Ok(vector.clone()),
            ListFoldingValueExpr::ListComprehension(lc) => Ok(self.unfold_lc(lc)?),
        }
    }
}
