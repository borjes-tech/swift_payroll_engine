use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayrollRuleError {
    #[error("invalid DSL type")]
    InvalidDslType,

    #[error("evaluation failed: {0}")]
    Evaluation(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DslType {
    OdooHRMS,
    FrappeHRMS,
    Rhai,
    CEL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRuleContext {
    pub rule_id: String,
    pub rule_name: String,
    pub dsl: String,
    pub dsl_type: DslType,
}

impl PayrollRuleContext {
    pub fn evaluate(
        &self,
        gross: rust_decimal::Decimal,
    ) -> Result<rust_decimal::Decimal, PayrollRuleError> {
        use crate::engines::DslEngineImple;
        match self.dsl_type {
            DslType::OdooHRMS => Ok(rust_decimal::Decimal::new(0, 0)),
            DslType::FrappeHRMS => Ok(rust_decimal::Decimal::new(0, 0)),
            DslType::Rhai => {
                let engine = crate::engines::rhai::RhaiDslEngine {
                    name: self.rule_name.clone(),
                    expression: self.dsl.clone(),
                };
                let res = engine.envaluate(gross);
                if res.success {
                    Ok(res.value.unwrap_or_default())
                } else {
                    Err(PayrollRuleError::Evaluation(res.error.unwrap_or_default()))
                }
            }
            DslType::CEL => {
                let engine = crate::engines::cel::CelEngine {
                    name: self.rule_name.clone(),
                    expression: self.dsl.clone(),
                };
                let res = engine.envaluate(gross);
                if res.success {
                    Ok(res.value.unwrap_or_default())
                } else {
                    Err(PayrollRuleError::Evaluation(res.error.unwrap_or_default()))
                }
            }
        }
    }
}
