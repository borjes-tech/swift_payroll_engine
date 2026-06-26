use crate::models::deduction::EmpDeduction;

pub enum EmploymentType {
    FullTime,
    Contract,
    PartTime,
}

pub struct ComputedDeduction {
    pub id: String,
    pub label: String,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Clone)]
pub struct Allowance {
    pub name: String,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Clone)]
pub struct EmployeeIdentity {
    pub emp_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EmployeeContext {
    pub identity: EmployeeIdentity,
    pub base_salary: rust_decimal::Decimal,
    pub allowances: Vec<Allowance>,
    pub deductions: Vec<EmpDeduction>,
    pub pay_period: Option<String>,
}

impl EmployeeContext {
    pub fn calculate_gross(&self) -> rust_decimal::Decimal {
        let mut total = self.base_salary;
        for allowance in &self.allowances {
            total += allowance.amount;
        }
        // Gross salary is base plus allowances; no double counting of base salary.
        total
    }

    pub fn calculate_net(&self, gross_salary: rust_decimal::Decimal) -> rust_decimal::Decimal {
        let deductions = &self.deductions;
        let net_salary = gross_salary
            - deductions
                .iter()
                .map(|d| d.amount)
                .sum::<rust_decimal::Decimal>();
        net_salary
    }
}
