use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DslType {
    OdooHRMS,
    FrappeHRMS,
    Rhai,
    CEL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRule {
    pub rule_id: String,
    pub rule_name: String,
    pub dsl: String,
    pub dsl_type: DslType,
}

impl PayrollRule {}
