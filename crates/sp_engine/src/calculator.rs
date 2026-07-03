use crate::models::deduction::EmpDeduction;

use super::models::emp_context::EmployeeContext;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("calculation failed: {0}")]
    CalculationError(String),
}

pub struct CalculationContext {
    pub emp_contexts: Vec<EmployeeContext>,
}

pub struct CalculationResult {
    pub gross_salary: rust_decimal::Decimal,
    pub net_salary: rust_decimal::Decimal,
    pub deductions: Vec<EmpDeduction>,
}

impl CalculationContext {
    pub fn calculate(&self) -> Result<Vec<CalculationResult>, EngineError> {
        let mut results: Vec<CalculationResult> = Vec::new();
        for emp_context in &self.emp_contexts {
            let mut gross_salary = emp_context.calculate_gross().round_dp_with_strategy(
                2,
                rust_decimal::RoundingStrategy::MidpointAwayFromZero,
            );
            gross_salary.rescale(2);

            let mut deductions = emp_context.deductions.clone();

            let mut total_deduction = rust_decimal::Decimal::new(0, 0);
            for d in deductions.iter_mut() {
                if let Some(rule) = &d.dsl_expr {
                    let amount = rule
                        .evaluate(gross_salary)
                        .unwrap_or(rust_decimal::Decimal::new(0, 0));
                    d.amount = amount;
                }
                d.amount.rescale(2);
                total_deduction += d.amount;
            }

            let mut net_salary = gross_salary
                - total_deduction.round_dp_with_strategy(
                    2,
                    rust_decimal::RoundingStrategy::MidpointAwayFromZero,
                );
            net_salary.rescale(2);

            results.push(CalculationResult {
                gross_salary,
                net_salary,
                deductions,
            });
        }
        Ok(results)
    }

    pub fn evaluate_deductions() {}
}
