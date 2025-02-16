use crate::physical_plan::state::ExecutionState;
use crate::prelude::*;
use polars_core::frame::groupby::GroupsProxy;
use polars_core::prelude::*;
use std::borrow::Cow;
use std::sync::Arc;

pub struct ColumnExpr(Arc<str>, Expr);

impl ColumnExpr {
    pub fn new(name: Arc<str>, expr: Expr) -> Self {
        Self(name, expr)
    }
}

impl PhysicalExpr for ColumnExpr {
    fn as_expression(&self) -> &Expr {
        &self.1
    }
    fn evaluate(&self, df: &DataFrame, state: &ExecutionState) -> Result<Series> {
        match state.get_schema() {
            None => df.column(&self.0).cloned(),
            Some(schema) => {
                let (idx, _, _) = schema
                    .get_full(&self.0)
                    .ok_or_else(|| PolarsError::NotFound(self.0.to_string()))?;
                Ok(df.get_columns()[idx].clone())
            }
        }
    }
    #[allow(clippy::ptr_arg)]
    fn evaluate_on_groups<'a>(
        &self,
        df: &DataFrame,
        groups: &'a GroupsProxy,
        state: &ExecutionState,
    ) -> Result<AggregationContext<'a>> {
        let s = self.evaluate(df, state)?;
        Ok(AggregationContext::new(s, Cow::Borrowed(groups), false))
    }
    fn to_field(&self, input_schema: &Schema) -> Result<Field> {
        let field = input_schema.get_field(&self.0).ok_or_else(|| {
            PolarsError::NotFound(format!(
                "could not find column: {} in schema: {:?}",
                self.0, &input_schema
            ))
        })?;
        Ok(field)
    }
}
