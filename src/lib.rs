use swc_core::ecma::{
    ast::{Program, CallExpr, Callee, Ident, Super, Expr, MemberExpr},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        let mut jquery_ident = n.callee.clone().expr().unwrap().member().unwrap().obj.call().unwrap().callee.expr().unwrap().ident().unwrap();

        if let Callee::Expr(expr) = &n.callee {
            if let Expr::Member(MemberExpr { obj, .. }) = &**expr {
                if let CallExpr(e) = obj.call() {
                        println!("foundhoge");
                    if ident.sym.to_string() == "jQuery" {
                        println!("found")
                    }
                }
            }
        }


        // println!("hoge: {:?}", jquery_ident);
        if jquery_ident.sym.to_string() == "jQuery" {
            jquery_ident = Ident::new("$".into(), jquery_ident.span);
        }
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    // Input codes
    r#"jQuery(document).ready(function(){})"#,
    // Output codes after transformed with plugin
    r#"$(document).ready(function(){})"#
);
