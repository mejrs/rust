use rustc_ast as ast;
use rustc_span::{DUMMY_SP, Ident};
use thin_vec::ThinVec;

/// Enumeration of things that an attribute can be on.
#[derive(Copy, Clone, Debug)]
pub enum AttrTarget<'ast> {
    Function {
        function: &'ast ast::Fn,
    },
    // Probably should have two variants for trait methods and regular impl methods
    Method {
        function: &'ast ast::Fn,
    },

    Trait {
        // This would be nice for `#[diagnostic::_]` parsing,
        // where we'd like to look at the generics.
        trait_: &'ast ast::Trait,
    },

    // everything else
    Todo,
}

impl AttrTarget<'_> {
    pub fn allowed(checks: &[TargetChecker]) -> Vec<&'static str> {
        // This is kinda ugly but we need a default value of the ast type to see if the check would
        // succeed. The contents don't need to make sense since the `allowed!` macro forces that it
        // cannot be matched on by fields.
        //
        // Things to make this nicer would be:
        // - Implement Default for everything,
        // - use the `strum` crate to generate a fieldless version of AttrTarget
        // - Maybe some way to get discriminants of enums without needing to instantiate it
        // - Some sort of unsafe hax maybe
        let generics = ast::Generics::default();
        let fn_sig = ast::FnSig {
            header: ast::FnHeader::default(),
            decl: Box::new(ast::FnDecl {
                inputs: ThinVec::new(),
                output: ast::FnRetTy::Default(DUMMY_SP),
            }),
            span: DUMMY_SP,
        };
        let function = &ast::Fn {
            defaultness: ast::Defaultness::Final,
            ident: Ident::dummy(),
            sig: fn_sig,
            generics: generics.clone(),
            contract: None,
            body: None,
            define_opaque: None,
        };
        let trait_ = &ast::Trait {
            safety: ast::Safety::Default,
            is_auto: ast::IsAuto::No,
            ident: Ident::dummy(),
            generics,
            bounds: Vec::new(),
            items: ThinVec::new(),
        };

        // BTreeSet to handle the case with overlapping 
        let mut ret = std::collections::BTreeSet::new();
        for c in checks {
            for check in c.check {
                if check(AttrTarget::Function { function }) {
                    ret.insert("function");
                }
                if check(AttrTarget::Method { function }) {
                    ret.insert("method");
                }
                if check(AttrTarget::Trait { trait_ }) {
                    ret.insert("trait");
                }
            }
        }

        ret.into_iter().collect()
    }
}

impl<'ast> From<&'ast ast::ItemKind> for AttrTarget<'ast> {
    fn from(kind: &'ast ast::ItemKind) -> Self {
        use ast::ItemKind::*;

        match kind {
            Fn(f) => AttrTarget::Function { function: &**f },
            Trait(t) => AttrTarget::Trait { trait_: &**t },
            _ => AttrTarget::Todo,
        }
    }
}

// FIXME(bool_hater) maybe _is_in_trait_impl should really be an enum, not a bool
impl<'ast> From<(bool, &'ast ast::AssocItemKind)> for AttrTarget<'ast> {
    fn from((_is_in_trait_impl, assoc): (bool, &'ast ast::AssocItemKind)) -> Self {
        use ast::AssocItemKind::*;

        match assoc {
            Fn(f) => AttrTarget::Method { function: &**f },
            _ => AttrTarget::Todo,
        }
    }
}

#[macro_export]
macro_rules! allowed {
    (_) => {{
        $crate::TargetChecker {
            check: &[
                {
                    fn check(_: $crate::AttrTarget<'_>) -> bool {
                        true
                    }
                    check
                }
            ],
        }
    }};
    ($($variant:ident)|*) => {{
        $crate::TargetChecker {
            check: &[
                $({
                    fn check(target: $crate::AttrTarget<'_>) -> bool {
                        matches!(target, | $crate::AttrTarget::$variant { .. } )
                    }
                    check
                }),*
            ],
        }
    }};
}

pub struct TargetChecker {
    pub check: &'static [fn(crate::AttrTarget<'_>) -> bool],
}

pub struct PosError {
    pub accepted: Vec<&'static str>,
    pub found_on: &'static str,
}
