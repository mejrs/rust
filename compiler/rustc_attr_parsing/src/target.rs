use rustc_ast as ast;

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
