use sp_dsl::models::{DslType, PayrollRuleContext};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct EmpDeduction {
    pub name: String,
    pub amount: rust_decimal::Decimal,
    pub dsl_expr: Option<sp_dsl::models::PayrollRuleContext>,
}

#[derive(Debug, Error)]
pub enum EmpDeductionError {
    #[error("employee deduction error")]
    GeneralError,
    #[error("unsupported DSL type")]
    UnsupportedDsl,
}

impl EmpDeduction {
    pub fn build_from_dsl(
        dsl_id: String,
        dsl_name: String,
        dsl_code: String,
        dsl_type: DslType,
    ) -> Result<Self, EmpDeductionError> {
        let payroll_rule = PayrollRuleContext {
            rule_id: dsl_id,
            rule_name: dsl_name.clone(),
            dsl: dsl_code,
            dsl_type,
        };

        Ok(Self {
            name: dsl_name,
            amount: rust_decimal::Decimal::new(0, 0),
            dsl_expr: Some(payroll_rule),
        })
    }

    pub fn new(name: String, amount: rust_decimal::Decimal) -> Self {
        Self {
            name,
            amount,
            dsl_expr: None,
        }
    }
}
