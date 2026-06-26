pub mod cel;
pub mod frappe_erpnext;
pub mod lua;
pub mod odoo;
pub mod rhai;

pub struct DslEngineResult {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
    pub name: String,
    pub value: Option<rust_decimal::Decimal>,
}

pub trait DslEngineImple {
    fn envaluate(&self, gross: rust_decimal::Decimal) -> DslEngineResult;
}
