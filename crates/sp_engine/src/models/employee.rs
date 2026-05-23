use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeContext {
    pub employee_id: String,
    pub base_salary: Decimal,
}
