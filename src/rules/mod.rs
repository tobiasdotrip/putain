pub mod builtin;

use crate::shell::CommandContext;

#[derive(Debug, Clone)]
pub struct Correction {
    pub fixed_command: String,
    pub rule_name: String,
    pub confidence: f64, // 0.0 to 1.0
}

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn suggest(&self, ctx: &CommandContext) -> Option<Correction>;
}

pub fn all_builtin_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(builtin::SudoRule),
        Box::new(builtin::GitPushUpstreamRule),
        Box::new(builtin::GitCheckoutNewBranchRule),
        Box::new(builtin::TypoCommandRule),
        Box::new(builtin::CdTypoRule),
    ]
}
