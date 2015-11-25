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
use rustc::front::map;

declare_lint!(TEST_LINT, Warn, "  :-/  ");

struct PublicInterfaceStabilityCheck {
    names : Vec<String>
}

impl LintPass for PublicInterfaceStabilityCheck {
    fn get_lints(&self) -> LintArray {
        lint_array!(TEST_LINT)
    }
}

impl LateLintPass for PublicInterfaceStabilityCheck {
    fn check_crate(&mut self, ctx: &LateContext, _: &hir::Crate) {
        for item_id in ctx.exported_items {
            if let Some(node_item) = ctx.tcx.map.find(*item_id) {
                self.handle_node_item(node_item, ctx);
            }
        }
        self.prepare_output();
        self.output();
    }
}

impl PublicInterfaceStabilityCheck {

    fn new() -> PublicInterfaceStabilityCheck {
        PublicInterfaceStabilityCheck {
            names: Vec::with_capacity(128)
        }
    }

    fn handle_node_item(&mut self, node_item: map::Node, ctx: &LateContext) {
        match node_item {
//            NodeImplItem(_) => (),
            map::NodeItem(item) => {
                self.names.push(format!("{}", item.name));
            },
            map::NodeForeignItem(item) => {
                self.names.push(format!("{}", item.name));
                //match item.node {
                //    ForeignItemFn(
                //        FnDecl
                //}
                println!("{:#?}", node_item);
            },
            map::NodeImplItem(item) => {
                //self.names.push(format!("{}", item.name));
            },
            _ => {
                if env_is_configured("print-all-nodeitems") {
                    //println!("{:#?}", node_item);
                }
            }
        }
    }

    fn prepare_output(&mut self) {
        self.names.sort();
    }

    fn output(&self) {
        for name in self.names.iter() {
            println!("{}", name);
        }
    }
}



fn env_is_configured<'x>(key: &'x str) -> bool {
    std::env::var("NITPICKERS")
        .and_then(|res| Ok( res.find(key).is_some() )) 
        .unwrap_or(false)
}



#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(box PublicInterfaceStabilityCheck::new() as LateLintPassObject);
}
