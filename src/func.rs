//! Tools for building custom functions.

/// Useful things for creating functions.
pub mod prelude {
    pub use async_trait::async_trait;
    pub use crate::layout::prelude::*;
    pub use crate::layout::Commands;
    pub use crate::layout::Command::{self, *};
    pub use crate::style::*;
    pub use crate::syntax::expr::*;
    pub use crate::syntax::parsing::{
        parse, FuncArgs, FuncBody, FuncCall, FuncHeader, ParseState,
    };
    pub use crate::syntax::span::{Span, SpanVec, Spanned};
    pub use crate::syntax::tree::{DynamicNode, SyntaxNode, SyntaxTree};
    pub use crate::syntax::value::*;
    pub use crate::{Pass, Feedback};
    pub use super::*;
}

use prelude::*;

/// Extra methods on `Option`s used for function argument parsing.
pub trait OptionExt<T>: Sized {
    /// Call `f` with `val` if this is `Some(val)`.
    fn with(self, f: impl FnOnce(T));

    /// Report an error about a missing argument with the given name and span if
    /// the option is `None`.
    fn or_missing(self, span: Span, arg: &str, f: &mut Feedback) -> Self;
}

impl<T> OptionExt<T> for Option<T> {
    fn with(self, f: impl FnOnce(T)) {
        if let Some(val) = self {
            f(val);
        }
    }

    fn or_missing(self, span: Span, arg: &str, f: &mut Feedback) -> Self {
        if self.is_none() {
            error!(@f, span, "missing argument: {}", arg);
        }
        self
    }
}

/// Generate `unexpected argument` errors for all remaining arguments.
pub fn drain_args(args: FuncArgs, f: &mut Feedback) {
    for arg in args.pos.0 {
        error!(@f, arg.span, "unexpected argument");
    }

    for arg in args.key.0 {
        error!(@f, arg.span, "unexpected argument");
    }
}

/// Generate an error if there is function body even though none was expected.
pub fn expect_no_body(body: FuncBody, f: &mut Feedback) {
    if let Some(body) = body {
        error!(@f, body.span, "unexpected body");
    }
}
