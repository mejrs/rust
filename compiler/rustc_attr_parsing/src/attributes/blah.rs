#![allow(warnings)]

use rustc_attr_data_structures::AttributeKind;
use rustc_span::{Span, Symbol, sym};
use thin_vec::ThinVec;

use super::{AcceptMapping, AttributeParser};
use crate::context::FinalizeContext;
use crate::session_diagnostics;

#[derive(Default)]
pub(crate) struct BlahParser;

impl AttributeParser for BlahParser {
    const ATTRIBUTES: AcceptMapping<Self> = &[(&[sym::rustc_blah], |this, cx, args| {
    })];

    fn finalize(self, _cx: &FinalizeContext<'_>) -> Option<AttributeKind> {
        Some(AttributeKind::Blah)
    }
}
