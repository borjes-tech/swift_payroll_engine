use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawEmployee {
    pub employee_id: String,
    pub full_name: String,
    pub base_salary: rust_decimal::Decimal,
    pub allowances: RawAllowance,
    pub deductions: Vec<RawDeduction>,
    pub pay_period: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawAllowance {
    pub transport: rust_decimal::Decimal,
    pub housing: rust_decimal::Decimal,
    pub meal: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize)]
pub struct RawDeduction {
    pub id: String,
    pub label: String,
    pub engine: String,
    pub expression: Option<String>,
    pub amount: Option<rust_decimal::Decimal>,
}
