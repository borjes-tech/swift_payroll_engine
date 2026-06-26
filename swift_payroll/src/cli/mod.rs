use sp_dsl::models::DslType;
use sp_engine::models::{
    deduction::{EmpDeduction, EmpDeductionError},
    emp_context::{Allowance, EmployeeContext, EmployeeIdentity},
};
use std::fs;

use crate::cli::{
    args::CliArgs,
    models::{RawAllowance, RawDeduction, RawEmployee},
};

pub mod args;
pub mod models;

fn build_allowances(raw: &RawAllowance) -> Vec<Allowance> {
    vec![
        Allowance {
            name: "transport".to_string(),
            amount: raw.transport,
        },
        Allowance {
            name: "housing".to_string(),
            amount: raw.housing,
        },
        Allowance {
            name: "meal".to_string(),
            amount: raw.meal,
        },
    ]
}

fn build_deductions(raw: &[RawDeduction]) -> Result<Vec<EmpDeduction>, EmpDeductionError> {
    raw.iter()
        .map(|d| {
            if d.engine == "cel" {
                EmpDeduction::build_from_dsl(
                    d.id.clone(),
                    d.label.clone(),
                    d.expression.clone().unwrap_or_default(),
                    DslType::CEL,
                )
            } else {
                Ok(EmpDeduction::new(
                    d.label.clone(),
                    d.amount.unwrap_or_default(),
                ))
            }
        })
        .collect()
}

fn raw_to_employee_context(raw: RawEmployee) -> EmployeeContext {
    EmployeeContext {
        identity: EmployeeIdentity {
            emp_id: raw.employee_id,
            name: Some(raw.full_name),
        },
        base_salary: raw.base_salary,
        allowances: build_allowances(&raw.allowances),
        deductions: build_deductions(&raw.deductions).expect("Error building deductions"),
        pay_period: raw.pay_period,
    }
}

pub fn run(args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = &args.input {
        let data = fs::read_to_string(path)?;
        let raw_emps: Vec<RawEmployee> = serde_json::from_str(&data)?;
        let contexts: Vec<EmployeeContext> =
            raw_emps.into_iter().map(raw_to_employee_context).collect();
        let calc = sp_engine::calculator::CalculationContext {
            emp_contexts: contexts,
        };
        match calc.calculate() {
            Ok(results) => {
                for (i, r) in results.iter().enumerate() {
                    println!(
                        "Employee {}: gross = {}, net = {}",
                        i + 1,
                        r.gross_salary,
                        r.net_salary
                    );
                }
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    } else {
        println!("No input file supplied. Args: {:?}", args);
        Ok(())
    }
}
