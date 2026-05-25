use crate::prelude::*;
use ruff_python_ast::{AstParamMutability, AstParamOwnership, Parameter};

#[derive(Default)]
pub struct FormatParameter;

impl FormatNodeRule<Parameter> for FormatParameter {
    fn fmt_fields(&self, item: &Parameter, f: &mut PyFormatter) -> FormatResult<()> {
        let Parameter {
            range: _,
            node_index: _,
            name,
            annotation,
            convention,
        } = item;

        match (convention.ownership, convention.mutability) {
            (AstParamOwnership::Borrow, AstParamMutability::Immutable) => {}
            (AstParamOwnership::Borrow, AstParamMutability::Mutable) => {
                token("mut").fmt(f)?;
                space().fmt(f)?;
            }
            (AstParamOwnership::Own, AstParamMutability::Immutable) => {
                token("own").fmt(f)?;
                space().fmt(f)?;
            }
            (AstParamOwnership::Own, AstParamMutability::Mutable) => {
                token("own").fmt(f)?;
                space().fmt(f)?;
                token("mut").fmt(f)?;
                space().fmt(f)?;
            }
        }

        name.format().fmt(f)?;

        if let Some(annotation) = annotation.as_deref() {
            token(":").fmt(f)?;

            if f.context().comments().has_leading(annotation)
                && !f.context().is_expression_parenthesized(annotation.into())
            {
                hard_line_break().fmt(f)?;
            } else {
                space().fmt(f)?;
            }

            annotation.format().fmt(f)?;
        }

        Ok(())
    }
}
