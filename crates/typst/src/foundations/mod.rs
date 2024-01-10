//! Foundational types and functions.

pub mod calc;
pub mod repr;
pub mod sys;

mod args;
mod array;
mod auto;
mod bool;
mod bytes;
mod cast;
mod datetime;
mod dict;
mod duration;
mod fields;
mod float;
mod func;
mod int;
mod label;
mod methods;
mod module;
mod none;
mod plugin;
mod scope;
mod selector;
mod str;
mod styles;
mod ty;
mod value;
mod version;

use std::borrow::Cow;
use std::hash::Hasher;
use std::sync::Arc;

pub use self::args::*;
pub use self::array::*;
pub use self::auto::*;
pub use self::bytes::*;
pub use self::cast::*;
pub use self::datetime::*;
pub use self::dict::*;
pub use self::duration::*;
pub use self::fields::*;
pub use self::float::*;
pub use self::func::*;
pub use self::int::*;
pub use self::label::*;
pub use self::methods::*;
pub use self::module::*;
pub use self::none::*;
pub use self::plugin::*;
pub use self::repr::Repr;
pub use self::scope::*;
pub use self::selector::*;
pub use self::str::*;
pub use self::styles::*;
pub use self::ty::*;
pub use self::value::*;
pub use self::version::*;

#[rustfmt::skip]
#[doc(hidden)]
pub use {
    ecow::{eco_format, eco_vec},
    indexmap::IndexMap,
    once_cell::sync::Lazy,
    typst_macros::elem,
};

use comemo::Prehashed;
use ecow::EcoString;
use typst_syntax::Span;

use crate::diag::{bail, SourceResult, StrResult};
use crate::engine::Engine;
use crate::eval::EvalMode;
use crate::introspection::Location;
use crate::syntax::Spanned;

/// Foundational types and functions.
///
/// Here, you'll find documentation for basic data types like [integers]($int)
/// and [strings]($str) as well as details about core computational functions.
#[category]
pub static FOUNDATIONS: Category;

/// Hook up all `foundations` definitions.
pub(super) fn define(global: &mut Scope, inputs: Dict) {
    global.category(FOUNDATIONS);
    global.define_type::<bool>();
    global.define_type::<i64>();
    global.define_type::<f64>();
    global.define_type::<Str>();
    global.define_type::<Label>();
    global.define_type::<Bytes>();
    global.define_type::<Array>();
    global.define_type::<Dict>();
    global.define_type::<Func>();
    global.define_type::<Args>();
    global.define_type::<Type>();
    global.define_type::<Module>();
    global.define_type::<Regex>();
    global.define_type::<Selector>();
    global.define_type::<Datetime>();
    global.define_type::<Duration>();
    global.define_type::<Version>();
    global.define_type::<Plugin>();
    global.define_func::<repr::repr>();
    global.define_func::<panic>();
    global.define_func::<assert>();
    global.define_func::<eval>();
    global.define_func::<style>();
    global.define_module(calc::module());
    global.define_module(sys::module(inputs));
}

/// Fails with an error.
///
/// Arguments are displayed to the user (not rendered in the document) as
/// strings, converting with `repr` if necessary.
///
/// # Example
/// The code below produces the error `panicked with: "this is wrong"`.
/// ```typ
/// #panic("this is wrong")
/// ```
#[func(keywords = ["error"])]
pub fn panic(
    /// The values to panic with and display to the user.
    #[variadic]
    values: Vec<Value>,
) -> StrResult<Never> {
    let mut msg = EcoString::from("panicked");
    if !values.is_empty() {
        msg.push_str(" with: ");
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                msg.push_str(", ");
            }
            msg.push_str(&value.repr());
        }
    }
    Err(msg)
}

/// Ensures that a condition is fulfilled.
///
/// Fails with an error if the condition is not fulfilled. Does not
/// produce any output in the document.
///
/// If you wish to test equality between two values, see
/// [`assert.eq`]($assert.eq) and [`assert.ne`]($assert.ne).
///
/// # Example
/// ```typ
/// #assert(1 < 2, message: "math broke")
/// ```
#[func(scope)]
pub fn assert(
    /// The condition that must be true for the assertion to pass.
    condition: bool,
    /// The error message when the assertion fails.
    #[named]
    message: Option<EcoString>,
) -> StrResult<NoneValue> {
    if !condition {
        if let Some(message) = message {
            bail!("assertion failed: {message}");
        } else {
            bail!("assertion failed");
        }
    }
    Ok(NoneValue)
}

#[scope]
impl assert {
    /// Ensures that two values are equal.
    ///
    /// Fails with an error if the first value is not equal to the second. Does not
    /// produce any output in the document.
    ///
    /// ```typ
    /// #assert.eq(10, 10)
    /// ```
    #[func(title = "Assert Equal")]
    pub fn eq(
        /// The first value to compare.
        left: Value,
        /// The second value to compare.
        right: Value,
        /// An optional message to display on error instead of the representations
        /// of the compared values.
        #[named]
        message: Option<EcoString>,
    ) -> StrResult<NoneValue> {
        if left != right {
            if let Some(message) = message {
                bail!("equality assertion failed: {message}");
            } else {
                bail!(
                    "equality assertion failed: value {} was not equal to {}",
                    left.repr(),
                    right.repr()
                );
            }
        }
        Ok(NoneValue)
    }

    /// Ensures that two values are not equal.
    ///
    /// Fails with an error if the first value is equal to the second. Does not
    /// produce any output in the document.
    ///
    /// ```typ
    /// #assert.ne(3, 4)
    /// ```
    #[func(title = "Assert Not Equal")]
    pub fn ne(
        /// The first value to compare.
        left: Value,
        /// The second value to compare.
        right: Value,
        /// An optional message to display on error instead of the representations
        /// of the compared values.
        #[named]
        message: Option<EcoString>,
    ) -> StrResult<NoneValue> {
        if left == right {
            if let Some(message) = message {
                bail!("inequality assertion failed: {message}");
            } else {
                bail!(
                    "inequality assertion failed: value {} was equal to {}",
                    left.repr(),
                    right.repr()
                );
            }
        }
        Ok(NoneValue)
    }
}

/// Evaluates a string as Typst code.
///
/// This function should only be used as a last resort.
///
/// # Example
/// ```example
/// #eval("1 + 1") \
/// #eval("(1, 2, 3, 4)").len() \
/// #eval("*Markup!*", mode: "markup") \
/// ```
#[func(title = "Evaluate")]
pub fn eval(
    /// The engine.
    engine: &mut Engine,
    /// A string of Typst code to evaluate.
    ///
    /// The code in the string cannot interact with the file system.
    source: Spanned<String>,
    /// The syntactical mode in which the string is parsed.
    ///
    /// ```example
    /// #eval("= Heading", mode: "markup")
    /// #eval("1_2^3", mode: "math")
    /// ```
    #[named]
    #[default(EvalMode::Code)]
    mode: EvalMode,
    /// A scope of definitions that are made available.
    ///
    /// ```example
    /// #eval("x + 1", scope: (x: 2)) \
    /// #eval(
    ///   "abc/xyz",
    ///   mode: "math",
    ///   scope: (
    ///     abc: $a + b + c$,
    ///     xyz: $x + y + z$,
    ///   ),
    /// )
    /// ```
    #[named]
    #[default]
    scope: Dict,
) -> SourceResult<Value> {
    let Spanned { v: text, span } = source;
    let dict = scope;
    let mut scope = Scope::new();
    for (key, value) in dict {
        scope.define(key, value);
    }
    crate::eval::eval_string(engine.world, &text, span, mode, scope)
}

/// An element's constructor function.
pub trait Construct {
    /// Construct an element from the arguments.
    ///
    /// This is passed only the arguments that remain after execution of the
    /// element's set rule.
    fn construct(engine: &mut Engine, args: &mut Args) -> SourceResult<Value>
    where
        Self: Sized;
}

/// An element's set rule.
pub trait Set {
    /// Parse relevant arguments into style properties for this element.
    fn set(engine: &mut Engine, args: &mut Args) -> SourceResult<Styles>
    where
        Self: Sized;
}

/// Synthesize fields on an element. This happens before execution of any show
/// rule.
pub trait Synthesize {
    /// Prepare the element for show rule application.
    fn synthesize(&mut self, engine: &mut Engine, styles: StyleChain)
        -> SourceResult<()>;
}

/// The base recipe for an element.
pub trait Show {
    /// Execute the base recipe for this element.
    fn show(&self, engine: &mut Engine, styles: StyleChain) -> SourceResult<Value>;
}

/// Post-process an element after it was realized.
pub trait Finalize {
    /// Finalize the fully realized form of the element. Use this for effects
    /// that should work even in the face of a user-defined show rule.
    fn finalize(&self, realized: Value, styles: StyleChain) -> Value;
}

/// How the element interacts with other elements.
pub trait Behave {
    /// The element's interaction behaviour.
    fn behaviour(&self) -> Behaviour;

    /// Whether this weak element is larger than a previous one and thus picked
    /// as the maximum when the levels are the same.
    #[allow(unused_variables)]
    fn larger(
        &self,
        prev: &(Cow<Value>, Behaviour, StyleChain),
        styles: StyleChain,
    ) -> bool {
        false
    }
}

/// How an element interacts with other elements in a stream.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Behaviour {
    /// A weak element which only survives when a supportive element is before
    /// and after it. Furthermore, per consecutive run of weak elements, only
    /// one survives: The one with the lowest weakness level (or the larger one
    /// if there is a tie).
    Weak(usize),
    /// An element that enables adjacent weak elements to exist. The default.
    Supportive,
    /// An element that destroys adjacent weak elements.
    Destructive,
    /// An element that does not interact at all with other elements, having the
    /// same effect as if it didn't exist, but has a visual representation.
    Ignorant,
    /// An element that does not have a visual representation.
    Invisible,
}

/// Guards content against being affected by the same show rule multiple times.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Guard {
    /// The nth recipe from the top of the chain.
    Nth(usize),
    /// The [base recipe](Show) for a kind of element.
    Base(Type),
}

/// Fields of an element.
pub trait ElementFields {
    /// The fields of the element.
    type Fields;
}

/// Tries to extract the plain-text representation of the element.
pub trait PlainText {
    /// Write this element's plain text into the given buffer.
    fn plain_text(&self, text: &mut EcoString);
}

/// Defines the `ElemFunc` for styled elements.
#[ty(Repr)]
pub struct StyledElem {
    pub child: Prehashed<Value>,
    pub styles: Styles,
}

impl PartialEq for StyledElem {
    fn eq(&self, other: &Self) -> bool {
        *self.child == *other.child
    }
}

impl Repr for StyledElem {
    fn repr(&self) -> EcoString {
        eco_format!("styled(child: {}, ..)", self.child.repr())
    }
}

/// Defines the `ElemFunc` for sequences.
#[ty(Repr)]
#[derive(Default, PartialEq)]
pub struct SequenceElem {
    pub children: Vec<Prehashed<Value>>,
}

impl Repr for SequenceElem {
    fn repr(&self) -> EcoString {
        if self.children.is_empty() {
            "[]".into()
        } else {
            eco_format!(
                "[{}]",
                repr::pretty_array_like(
                    &self.children.iter().map(|c| c.repr()).collect::<Vec<_>>(),
                    false
                )
            )
        }
    }
}
