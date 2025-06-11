#![allow(warnings)]

use rustc_attr_data_structures::AttributeKind;
use rustc_span::{Span, Symbol, sym};
use thin_vec::ThinVec;

use super::{AcceptMapping, AttributeParser};
use crate::context::FinalizeContext;
use crate::session_diagnostics;

#[derive(Default)]
pub(crate) struct BlahParser {
    found: bool,
}

impl AttributeParser for BlahParser {
    const ATTRIBUTES: AcceptMapping<Self> = &[(&[sym::blah], |this, cx, args| {
        this.found = true;
    })];

    fn finalize(self, _cx: &FinalizeContext<'_>) -> Option<AttributeKind> {
        if self.found { Some(AttributeKind::Blah) } else { None }
    }
}
