use crate::engines::DslEngineImple;
use rhai::Engine as RhaiEngine;
use rust_decimal::prelude::FromPrimitive;

pub struct RhaiDslEngine {
    pub name: String,
    pub expression: String,
}

impl DslEngineImple for RhaiDslEngine {
    fn envaluate(&self, gross: rust_decimal::Decimal) -> super::DslEngineResult {
        let engine = RhaiEngine::new();

        let mut scope = rhai::Scope::new();
        scope.push(
            "gross",
            rust_decimal::prelude::ToPrimitive::to_f64(&gross).unwrap_or(0.0),
        );

        match engine.eval_expression_with_scope::<rhai::Dynamic>(&mut scope, &self.expression) {
            Ok(val) => {
                let decimal_opt = if let Ok(i) = val.as_int() {
                    rust_decimal::Decimal::from_i64(i)
                } else if let Ok(f) = val.as_float() {
                    rust_decimal::Decimal::from_f64(f)
                } else {
                    None
                };

                super::DslEngineResult {
                    success: decimal_opt.is_some(),
                    message: if decimal_opt.is_some() {
                        Some("Rhai evaluation succeeded".to_string())
                    } else {
                        None
                    },
                    error: if decimal_opt.is_none() {
                        Some("Expression did not evaluate to a numeric value".to_string())
                    } else {
                        None
                    },
                    name: self.name.clone(),
                    value: decimal_opt,
                }
            }
            Err(e) => super::DslEngineResult {
                success: false,
                message: None,
                error: Some(format!("Rhai evaluation error: {e}")),
                name: self.name.clone(),
                value: None,
            },
        }
    }
}
