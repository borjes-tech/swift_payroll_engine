use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CalculateError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateContext {
    pub emp_contx: models::employee::EmployeeContext,
    pub payroll_rules: String,
}

impl CalculateContext {}
