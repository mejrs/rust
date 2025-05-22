use std::convert::Infallible;

use crate::{ParseError, ParseMode, Parser, Suggestion};

#[derive(Debug, PartialEq)]
pub struct Diagnostic;

impl ParseMode for Diagnostic {
    type Ret<'input> = Option<Infallible>;

    type Err = ParseError;

    fn parse_modifier<'input>(this: &mut Parser<'input, Diagnostic>) -> Self::Ret<'input> {
        if !this.consume(':') {
            return None;
        }

        let start = this.input_vec_index;
        let Some((_, string_start, _)) = this.input_vec.get(start) else {
            return None;
        };

        let modifier = this.string(*string_start);
        if !modifier.is_empty() {
            let start = this.input_vec_index2range(start).start;
            let end = this.input_vec_index2range(this.input_vec_index).start;
            this.errors.push(ParseError {
                description: "expected no format specifier`".into(),
                note: None,
                label: modifier.into(),
                span: start..end,
                secondary_label: None,
                suggestion: Suggestion::None,
            });
        };

        None
    }
}
