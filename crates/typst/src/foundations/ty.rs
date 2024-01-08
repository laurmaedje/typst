use std::any::TypeId;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};

use ecow::{eco_format, EcoString};
use once_cell::sync::Lazy;

use crate::diag::{SourceResult, StrResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, func, Args, Dict, Func, NativeFuncData, Repr, Scope, Selector, Styles, Value,
};
use crate::text::{Lang, Region};
use crate::util::Static;

#[rustfmt::skip]
#[doc(inline)]
pub use typst_macros::{scope, ty};

/// Describes a kind of value.
///
/// To style your document, you need to work with values of different kinds:
/// Lengths specifying the size of your elements, colors for your text and
/// shapes, and more. Typst categorizes these into clearly defined _types_ and
/// tells you where it expects which type of value.
///
/// Apart from basic types for numeric values and [typical]($int)
/// [types]($float) [known]($str) [from]($array) [programming]($dictionary)
/// languages, Typst provides a special type for [_content._]($content) A value
/// of this type can hold anything that you can enter into your document: Text,
/// elements like headings and shapes, and style information.
///
/// # Example
/// ```example
/// #let x = 10
/// #if type(x) == int [
///   #x is an integer!
/// ] else [
///   #x is another value...
/// ]
///
/// An image is of type
/// #type(image("glacier.jpg")).
/// ```
///
/// The type of `10` is `int`. Now, what is the type of `int` or even `type`?
/// ```example
/// #type(int) \
/// #type(type)
/// ```
///
/// # Compatibility
/// In Typst 0.7 and lower, the `type` function returned a string instead of a
/// type. Compatibility with the old way will remain for a while to give package
/// authors time to upgrade, but it will be removed at some point.
///
/// - Checks like `{int == "integer"}` evaluate to `{true}`
/// - Adding/joining a type and string will yield a string
/// - The `{in}` operator on a type and a dictionary will evaluate to `{true}`
///   if the dictionary has a string key matching the type's name
#[ty(scope, cast)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type(Static<NativeTypeData>);

impl Type {
    /// Get the type for `T`.
    pub fn of<T: NativeType>() -> Self {
        T::ty()
    }

    /// The type's short name, how it is used in code (e.g. `str`).
    pub fn short_name(&self) -> &'static str {
        self.0.name
    }

    /// The type's long name, for use in diagnostics (e.g. `string`).
    pub fn long_name(&self) -> &'static str {
        self.0.long_name
    }

    /// The type's title case name, for use in documentation (e.g. `String`).
    pub fn title(&self) -> &'static str {
        self.0.title
    }

    /// Documentation for the type (as Markdown).
    pub fn docs(&self) -> &'static str {
        self.0.docs
    }

    /// Search keywords for the type.
    pub fn keywords(&self) -> &'static [&'static str] {
        self.0.keywords
    }

    /// This type's constructor function.
    pub fn constructor(&self) -> StrResult<Func> {
        self.0
            .constructor
            .as_ref()
            .map(|lazy| Func::from(*lazy))
            .ok_or_else(|| eco_format!("type {self} does not have a constructor"))
    }

    /// The type's associated scope that holds sub-definitions.
    pub fn scope(&self) -> &'static Scope {
        &(self.0).0.scope
    }

    /// Get a field from this type's scope, if possible.
    pub fn field(&self, field: &str) -> StrResult<&'static Value> {
        self.scope()
            .get(field)
            .ok_or_else(|| eco_format!("type {self} does not contain field `{field}`"))
    }

    pub fn select(&self) -> Selector {
        Selector::Type(*self)
    }

    pub fn field_id(&self, name: &str) -> Option<u8> {
        todo!()
    }

    pub fn field_name(&self, id: u8) -> Option<&'static str> {
        todo!()
    }

    /// Execute the set rule for the element and return the resulting style map.
    pub fn set(self, engine: &mut Engine, mut args: Args) -> SourceResult<Styles> {
        todo!()
    }

    /// The element's local name, if any.
    pub fn local_name(&self, lang: Lang, region: Option<Region>) -> Option<&'static str> {
        todo!()
    }

    /// Whether the type has the given capability.
    pub fn can<C>(self) -> bool
    where
        C: ?Sized + 'static,
    {
        todo!()
    }

    /// Whether the type has the given capability where the capability is
    /// given by a `TypeId`.
    pub fn can_type_id(self, type_id: TypeId) -> bool {
        todo!()
    }
}

// Type compatibility.
impl Type {
    /// The type's backward-compatible name.
    pub fn compat_name(&self) -> &str {
        self.long_name()
    }
}

#[scope]
impl Type {
    /// Determines a value's type.
    ///
    /// ```example
    /// #type(12) \
    /// #type(14.7) \
    /// #type("hello") \
    /// #type(<glacier>) \
    /// #type([Hi]) \
    /// #type(x => x + 1) \
    /// #type(type)
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// The value whose type's to determine.
        value: Value,
    ) -> Type {
        value.ty()
    }

    /// Returns a selector that filters for elements belonging to this type
    /// whose fields have the values of the given arguments.
    #[func]
    pub fn where_(
        self,
        /// The real arguments (the other argument is just for the docs).
        /// The docs argument cannot be called `args`.
        args: &mut Args,
        /// The fields to filter for.
        #[variadic]
        #[external]
        fields: Vec<Args>,
    ) -> StrResult<Selector> {
        let fields = args.to_named();
        args.items.retain(|arg| arg.name.is_none());

        let fields = fields
            .into_iter()
            .map(|(key, value)| {
                self.field_id(&key)
                    .map(|id| (id, value))
                    .ok_or_else(|| eco_format!("type {self} does not have field `{key}`"))
            })
            .collect::<StrResult<smallvec::SmallVec<_>>>()?;

        Ok(Selector::Where(self, fields))
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Type({})", self.long_name())
    }
}

impl Repr for Type {
    fn repr(&self) -> EcoString {
        self.long_name().into()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.long_name())
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.long_name().cmp(other.long_name())
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A Typst type that is defined by a native Rust type.
pub trait NativeType {
    /// The type's name.
    ///
    /// In contrast to `data()`, this is usable in const contexts.
    const NAME: &'static str;

    /// Get the type for the native Rust type.
    fn ty() -> Type {
        Type::from(Self::data())
    }

    // Get the type data for the native Rust type.
    fn data() -> &'static NativeTypeData;
}

/// Used to cast an element to a trait object for a trait it implements.
///
/// # Safety
/// If the `vtable` function returns `Some(p)`, then `p` must be a valid pointer
/// to a vtable of `Packed<Self>` w.r.t to the trait `C` where `capability` is
/// `TypeId::of::<dyn C>()`.
pub unsafe trait Capable {
    /// Get the pointer to the vtable for the given capability / trait.
    fn vtable(capability: TypeId) -> Option<*const ()>;
}

/// Defines how fields of an element are accessed.
pub trait Fields {
    /// Whether the element has the given field set.
    fn has(&self, id: u8) -> bool;

    /// Get the field with the given field ID.
    fn field(&self, id: u8) -> Option<Value>;

    /// Get the fields of the element.
    fn fields(&self) -> Dict;
}

/// Defines a native type.
#[derive(Debug)]
pub struct NativeTypeData {
    pub name: &'static str,
    pub long_name: &'static str,
    pub title: &'static str,
    pub docs: &'static str,
    pub keywords: &'static [&'static str],
    pub constructor: Lazy<Option<&'static NativeFuncData>>,
    pub scope: Lazy<Scope>,
}

impl From<&'static NativeTypeData> for Type {
    fn from(data: &'static NativeTypeData) -> Self {
        Self(Static(data))
    }
}

cast! {
    &'static NativeTypeData,
    self => Type::from(self).into_value(),
}
