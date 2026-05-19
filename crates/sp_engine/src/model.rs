use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeContext {
    pub employee_id: String,
    pub grade: u8,
    pub step: u8,
    pub basic_salary: Decimal,
    pub employment_type: EmploymentType,
    pub flags: Vec<String>,
    pub worked_days: u32,
    pub total_working_days: u32,
    pub active_deductions: Vec<ActiveDeduction>,
    pub ytd_gross: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentType {
    Permanent,
    Contract,
    Casual,
    Probation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDeduction {
    pub id: String,
    pub label: String,
    pub monthly_amount: Decimal,
    pub category: DeductionCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeductionCategory {
    Loan,
    SalaryAdvance,
    Overpayment,
    UnionDues,
    CoopShares,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub code: String,
    pub name: String,
    pub sequence: u32,
    pub category: PolicyCategory,
    pub eligibility: EligibilityRule,
    pub calculation: CalculationRule,
    pub appears_on_payslip: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyCategory {
    Earning,
    Deduction,
    Tax,
    Pension,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EligibilityRule {
    Always,
    All {
        conditions: Vec<EligibilityCondition>,
    },

    Any {
        conditions: Vec<EligibilityCondition>,
    },

    Expression {
        expr: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EligibilityCondition {
    GradeRange { min: u8, max: u8 },
    HasFlag { flag: String },
    EmploymentType { types: Vec<EmploymentType> },
    MinStep { step: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CalculationRule {
    Fixed { amount: Decimal },

    PercentOfBasic { rate: Decimal },

    PercentOfGross { rate: Decimal },

    GradeBased { grade_amounts: HashMap<u8, Decimal> },

    ProgressiveTax { brackets: Vec<TaxBracket> },

    Expression { expr: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBracket {
    pub lower: Decimal,
    pub upper: Option<Decimal>,
    pub rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub policy_code: String,
    pub label: String,
    pub amount: Decimal,
    pub calculation_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollResult {
    pub employee_id: String,
    pub pay_period: PayPeriod,
    pub basic_salary: Decimal,
    pub earnings: Vec<LineItem>,
    pub deductions: Vec<LineItem>,
    pub gross: Decimal,
    pub total_deductions: Decimal,
    pub net: Decimal,
    pub status: PayrollStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPeriod {
    pub year: u16,
    pub month: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayrollStatus {
    Ok,
    NetBelowMinimum,
    ProrationApplied,
    FlaggedForReview,
}
