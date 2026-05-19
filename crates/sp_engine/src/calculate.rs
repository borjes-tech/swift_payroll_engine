use crate::model::{ActiveDeduction, EmployeeContext};
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum CalculateError {}

pub struct CalculateResult {
    pub basic_salary: Decimal,
    pub deductions: Vec<ActiveDeduction>,
    pub net_salary: Decimal,
    pub employee_context: EmployeeContext,
}

pub struct CalculateContext {
    pub emp_context: EmployeeContext,
}

impl CalculateContext {}
