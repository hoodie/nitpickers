#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

extern crate syntax;

// Load rustc as a plugin to get macros
#[macro_use]
extern crate rustc;

extern crate rustc_front;

use syntax::ast;
use rustc::lint::{LateContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
use rustc::plugin::Registry;
use rustc_front::hir;

declare_lint!(TEST_LINT, Warn, "  :-/  ");

struct PublicInterfaceStabilityCheck;

impl LintPass for PublicInterfaceStabilityCheck {
    fn get_lints(&self) -> LintArray {
        lint_array!(TEST_LINT)
    }
}

impl LateLintPass for PublicInterfaceStabilityCheck {
    fn check_crate(&mut self, ctx: &LateContext, _: &hir::Crate) {
        for item_id in ctx.exported_items {
            if let Some(item) = ctx.tcx.map.find(*item_id) {
                println!("pub {:#?}", item);
            }
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(box PublicInterfaceStabilityCheck as LateLintPassObject);
}
