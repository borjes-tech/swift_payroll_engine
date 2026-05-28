use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollInput {
    pub emp_contexts: Vec<models::cal_context::CalculateContext>,
}
