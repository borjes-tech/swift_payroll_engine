#[cfg(test)]
mod basic {
    use fake::{Fake, Faker};
    use fake::{faker::name::raw::*, locales::*};
    use rust_decimal::{Decimal, prelude::FromPrimitive};
    use sp_engine::calculator::CalculationContext;
    use sp_engine::models::{
        deduction::EmpDeduction,
        emp_context::{Allowance, EmployeeContext, EmployeeIdentity},
    };

    #[test]
    fn test_basic_01_calculation() {
        let employees = build_random_employees(100_000);

        let calculator = CalculationContext {
            emp_contexts: employees,
        };

        match calculator.calculate() {
            Ok(results) => {
                for (i, r) in results.iter().enumerate() {
                    println!(
                        "Employee {}: gross = {}, net = {}",
                        i + 1,
                        r.gross_salary,
                        r.net_salary
                    );
                }
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
    }

    fn build_random_employees(count: u32) -> Vec<EmployeeContext> {
        (0..count)
            .map(|_| EmployeeContext {
                identity: EmployeeIdentity {
                    emp_id: uuid::Uuid::new_v4().to_string(),
                    name: Name(EN).fake(),
                },
                base_salary: Decimal::from_u64((10_000u64..200_000).fake::<u64>()).unwrap(),
                allowances: build_random_allowances(20),
                deductions: build_deductions(23),
                pay_period: None,
            })
            .collect()
    }

    fn build_deductions(count: u32) -> Vec<EmpDeduction> {
        (0..count)
            .map(|_| EmpDeduction {
                name: Faker.fake::<String>(),
                amount: Decimal::from_u64((10_000u64..200_000).fake::<u64>()).unwrap(),
                dsl_expr: None,
            })
            .collect()
    }

    fn build_random_allowances(count: u32) -> Vec<Allowance> {
        (0..count)
            .map(|_| Allowance {
                name: Faker.fake::<String>(),
                amount: Decimal::from_u64((10_000u64..200_000).fake::<u64>()).unwrap(),
            })
            .collect()
    }
}
