use rustc_ast::{AttrStyle, Item, ItemKind, ast};
use rustc_attr_parsing::parser::ArgParser;
use rustc_attr_parsing::{AttributeParser, AttributeSafety, ParsedDescription, ShouldEmit};
use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_feature::template;
use rustc_hir::{AttrPath, Target};
use rustc_parse::parser::Recovery;
use rustc_span::{Span, sym};

pub(crate) mod rustc_on_unimplemented {
    use super::*;
    pub(crate) fn expand(
        ecx: &mut ExtCtxt<'_>,
        _expand_span: Span,
        _meta_item: &ast::MetaItem,
        args: &ArgParser,
        mut item: Annotatable,
    ) -> Vec<Annotatable> {
        match &mut item {
            Annotatable::Item(box Item { span, kind: ItemKind::Trait(trait_), .. }) => {
                let span = *span;
                let on_unimplemented = AttributeParser::parse_single_args(
                    ecx.sess,
                    span,
                    span,
                    AttrStyle::Inner,
                    AttrPath {
                        segments: vec![sym::rustc_on_unimplemented].into_boxed_slice(),
                        span,
                    },
                    None,
                    AttributeSafety::Normal,
                    ParsedDescription::Macro,
                    span,
                    ecx.current_expansion.lint_node_id,
                    // Doesn't matter what the target actually is here.
                    Target::Trait,
                    Some(ecx.ecfg.features),
                    ShouldEmit::ErrorsAndLints { recovery: Recovery::Allowed },
                    args,
                    rustc_attr_parsing::parse_rustc_on_unimplemented,
                    &template!(List: &[r#"/*opt*/ message = "...", /*opt*/ label = "...", /*opt*/ note = "...""#]),
                );
                trait_.on_unimplemented = on_unimplemented;
            }
            _ => {}
        }

        vec![item]
    }
}
