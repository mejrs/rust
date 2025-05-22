use std::ops::Range;

use crate::{ParseError, ParseMode, Parser};

#[derive(Debug, PartialEq)]
pub struct InlineAsm;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AsmModifier<'input> {
    pub modifier: &'input str,
    pub offset: Option<Range<usize>>,
}

impl ParseMode for InlineAsm {
    type Ret<'input> = AsmModifier<'input>;

    type Err = ParseError;

    fn parse_modifier<'input>(this: &mut Parser<'input, InlineAsm>) -> Self::Ret<'input> {
        if !this.consume(':') {
            return AsmModifier { modifier: "", offset: None };
        }

        let start_idx = this.input_vec_index;

        let modifier = this.word();
        let offset = if !modifier.is_empty() {
            let start = this.input_vec_index2range(start_idx).start;
            let end = this.input_vec_index2range(this.input_vec_index).start;
            Some(start..end)
        } else {
            None
        };
        AsmModifier { modifier, offset }
    }
}
