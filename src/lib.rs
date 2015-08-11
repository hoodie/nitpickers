#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

extern crate syntax;

// Load rustc as a plugin to get macros
#[macro_use]
extern crate rustc;

use syntax::ast;
use rustc::lint::{Context, LintPass, LintPassObject, LintArray};
use rustc::plugin::Registry;

declare_lint!(TEST_LINT, Warn, "  :-/  ");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(TEST_LINT)
    }

    fn check_item(&mut self, _/*cx*/: &Context, it: &ast::Item) {
        //if it.ident.name.as_str() == "lintme" {
        //    cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
        //}
        
        let name = it.ident.name.as_str();
        if it.vis == ast::Visibility::Public {
            println!("pub {}", name);
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box Pass as LintPassObject);
}
