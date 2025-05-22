use std::ops::Range;

pub use Alignment::*;
pub use Count::*;
pub use Position::*;

use crate::{ParseError, ParseMode, Parser, Suggestion};

#[derive(Debug, PartialEq)]
pub struct Format;

/// Specification for the formatting of an argument in the format string.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FormatSpec<'input> {
    /// Optionally specified character to fill alignment with.
    pub fill: Option<char>,
    /// Span of the optionally specified fill character.
    pub fill_span: Option<Range<usize>>,
    /// Optionally specified alignment.
    pub align: Alignment,
    /// The `+` or `-` flag.
    pub sign: Option<Sign>,
    /// The `#` flag.
    pub alternate: bool,
    /// The `0` flag.
    pub zero_pad: bool,
    /// The `x` or `X` flag. (Only for `Debug`.)
    pub debug_hex: Option<DebugHex>,
    /// The integer precision to use.
    pub precision: Count<'input>,
    /// The span of the precision formatting flag (for diagnostics).
    pub precision_span: Option<Range<usize>>,
    /// The string width requested for the resulting format.
    pub width: Count<'input>,
    /// The span of the width formatting flag (for diagnostics).
    pub width_span: Option<Range<usize>>,
    /// The descriptor string representing the name of the format desired for
    /// this argument, this can be empty or any number of characters, although
    /// it is required to be one word.
    pub ty: &'input str,
    /// The span of the descriptor string (for diagnostics).
    pub ty_span: Option<Range<usize>>,
}

/// Enum describing where an argument for a format can be located.
#[derive(Clone, Debug, PartialEq)]
pub enum Position<'input> {
    /// The argument is implied to be located at an index
    ArgumentImplicitlyIs(usize),
    /// The argument is located at a specific index given in the format,
    ArgumentIs(usize),
    /// The argument has a name.
    ArgumentNamed(&'input str),
}

impl Position<'_> {
    pub fn index(&self) -> Option<usize> {
        match self {
            ArgumentIs(i, ..) | ArgumentImplicitlyIs(i) => Some(*i),
            _ => None,
        }
    }
}

/// Enum of alignments which are supported.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum Alignment {
    /// The value will be aligned to the left.
    AlignLeft,
    /// The value will be aligned to the right.
    AlignRight,
    /// The value will be aligned in the center.
    AlignCenter,
    /// The value will take on a default alignment.
    #[default]
    AlignUnknown,
}

/// Enum for the sign flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sign {
    /// The `+` flag.
    Plus,
    /// The `-` flag.
    Minus,
}

/// Enum for the debug hex flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DebugHex {
    /// The `x` flag in `{:x?}`.
    Lower,
    /// The `X` flag in `{:X?}`.
    Upper,
}

/// A count is used for the precision and width parameters of an integer, and
/// can reference either an argument or a literal integer.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Count<'input> {
    /// The count is specified explicitly.
    CountIs(u16),
    /// The count is specified by the argument with the given name.
    CountIsName(&'input str, Range<usize>),
    /// The count is specified by the argument at the given index.
    CountIsParam(usize),
    /// The count is specified by a star (like in `{:.*}`) that refers to the argument at the given index.
    CountIsStar(usize),
    /// The count is implied and cannot be explicitly specified.
    #[default]
    CountImplied,
}

impl ParseMode for Format {
    type Ret<'input> = FormatSpec<'input>;

    type Err = ParseError;

    fn parse_modifier<'input>(this: &mut Parser<'input, Format>) -> Self::Ret<'input> {
        let mut spec = FormatSpec::default();

        if !this.consume(':') {
            return spec;
        }

        // fill character
        if let Some(&(ref r, _, c)) = this.input_vec.get(this.input_vec_index) {
            if let Some((_, _, '>' | '<' | '^')) = this.input_vec.get(this.input_vec_index + 1) {
                this.input_vec_index += 1;
                spec.fill = Some(c);
                spec.fill_span = Some(r.clone());
            }
        }
        // Alignment
        if this.consume('<') {
            spec.align = AlignLeft;
        } else if this.consume('>') {
            spec.align = AlignRight;
        } else if this.consume('^') {
            spec.align = AlignCenter;
        }
        // Sign flags
        if this.consume('+') {
            spec.sign = Some(Sign::Plus);
        } else if this.consume('-') {
            spec.sign = Some(Sign::Minus);
        }
        // Alternate marker
        if this.consume('#') {
            spec.alternate = true;
        }
        // Width and precision
        let mut havewidth = false;

        if let Some((range, _)) = this.consume_pos('0') {
            // small ambiguity with '0$' as a format string. In theory this is a
            // '0' flag and then an ill-formatted format string with just a '$'
            // and no count, but this is better if we instead interpret this as
            // no '0' flag and '0$' as the width instead.
            if let Some((r, _)) = this.consume_pos('$') {
                spec.width = CountIsParam(0);
                spec.width_span = Some(range.start..r.end);
                havewidth = true;
            } else {
                spec.zero_pad = true;
            }
        }

        if !havewidth {
            let start_idx = this.input_vec_index;
            spec.width = this.count();
            if spec.width != CountImplied {
                let end = this.input_vec_index2range(this.input_vec_index).start;
                spec.width_span = Some(this.input_vec_index2range(start_idx).start..end);
            }
        }

        if let Some((range, _)) = this.consume_pos('.') {
            if this.consume('*') {
                // Resolve `CountIsNextParam`.
                // We can do this immediately as `position` is resolved later.
                let i = this.curarg;
                this.curarg += 1;
                spec.precision = CountIsStar(i);
            } else {
                spec.precision = this.count();
            }
            spec.precision_span =
                Some(range.start..this.input_vec_index2range(this.input_vec_index).start);
        }

        let start_idx = this.input_vec_index;
        // Optional radix followed by the actual format specifier
        if this.consume('x') {
            if this.consume('?') {
                spec.debug_hex = Some(DebugHex::Lower);
                spec.ty = "?";
            } else {
                spec.ty = "x";
            }
        } else if this.consume('X') {
            if this.consume('?') {
                spec.debug_hex = Some(DebugHex::Upper);
                spec.ty = "?";
            } else {
                spec.ty = "X";
            }
        } else if let Some((range, _)) = this.consume_pos('?') {
            spec.ty = "?";
            if let Some((r, _, c)) = this.input_vec.get(this.input_vec_index) {
                match c {
                    '#' | 'x' | 'X' => this.errors.insert(
                        0,
                        ParseError {
                            description: format!("expected `}}`, found `{c}`"),
                            note: None,
                            label: "expected `'}'`".into(),
                            span: r.clone(),
                            secondary_label: None,
                            suggestion: Suggestion::ReorderFormatParameter(
                                range.start..r.end,
                                format!("{c}?"),
                            ),
                        },
                    ),
                    _ => (),
                }
            }
        } else {
            spec.ty = this.word();
            if !spec.ty.is_empty() {
                let start = this.input_vec_index2range(start_idx).start;
                let end = this.input_vec_index2range(this.input_vec_index).start;
                spec.ty_span = Some(start..end);
            }
        }

        spec
    }
}
