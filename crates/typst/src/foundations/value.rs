use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use ecow::EcoString;
use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smallvec::smallvec;

use crate::diag::{SourceResult, StrResult};
use crate::engine::Engine;
use crate::eval::ops;
use crate::foundations::{
    Args, Array, AutoValue, Bytes, CastInfo, Datetime, Dict, Duration, FromValue, Func,
    Guard, IntoValue, Label, Module, NativeType, NoneValue, Plugin, Recipe, Reflect,
    Repr, Scope, Selector, SequenceElem, Str, Style, Styles, Type, Version,
};
use crate::introspection::{Location, Meta, MetaElem};
use crate::layout::{
    Abs, AlignElem, Alignment, Angle, Axes, Em, Fr, Length, MoveElem, PadElem, Ratio,
    Rel, Sides,
};
use crate::model::{Destination, EmphElem, StrongElem};
use crate::symbols::Symbol;
use crate::syntax::{ast, Span};
use crate::text::UnderlineElem;
use crate::visualize::{Color, Gradient, Pattern};

/// A computational value.
#[derive(Default, Clone)]
pub struct Value {
    _private: (),
}

impl Value {
    pub const fn inline<T: Copy + 'static>(_data: T) -> Self {
        Self { _private: () }
    }

    pub fn new<T: 'static>(_data: T) -> Self {
        Self { _private: () }
    }

    pub fn none() -> Self {
        NoneValue.into_value()
    }

    /// Create a new sequence element from multiples elements.
    pub fn sequence(iter: impl IntoIterator<Item = Self>) -> Self {
        todo!()
    }

    /// Style this content with a style entry.
    pub fn styled(mut self, style: impl Into<Style>) -> Self {
        todo!()
    }

    /// Style this content with a full style map.
    pub fn styled_with_map(mut self, styles: Styles) -> Self {
        todo!()
    }

    /// Style this content with a recipe, eagerly applying it if possible.
    pub fn styled_with_recipe(
        self,
        engine: &mut Engine,
        recipe: Recipe,
    ) -> SourceResult<Self> {
        todo!()
    }

    /// Create a numeric value from a number with a unit.
    pub fn numeric(pair: (f64, ast::Unit)) -> Self {
        let (v, unit) = pair;
        match unit {
            ast::Unit::Pt => Abs::pt(v).into_value(),
            ast::Unit::Mm => Abs::mm(v).into_value(),
            ast::Unit::Cm => Abs::cm(v).into_value(),
            ast::Unit::In => Abs::inches(v).into_value(),
            ast::Unit::Rad => Angle::rad(v).into_value(),
            ast::Unit::Deg => Angle::deg(v).into_value(),
            ast::Unit::Em => Em::new(v).into_value(),
            ast::Unit::Fr => Fr::new(v).into_value(),
            ast::Unit::Percent => Ratio::new(v / 100.0).into_value(),
        }
    }

    /// Whether the value is of the concrete type.
    pub fn is<T>(&self) -> bool {
        todo!()
    }

    /// Cast the value to a concrete type.
    pub fn to<T>(&self) -> Option<&Packed<T>> {
        todo!()
    }

    /// Cast the value to a concrete type.
    pub fn to_mut<T>(&mut self) -> Option<&mut Packed<T>> {
        todo!()
    }

    /// Cast the value to a concrete type.
    pub fn to_packed<T>(self) -> Result<Packed<T>, Self> {
        todo!()
    }

    /// The type of this value.
    pub fn ty(&self) -> Type {
        todo!()
    }

    /// Try to cast the value into a specific type.
    pub fn cast<T: FromValue>(self) -> StrResult<T> {
        T::from_value(self)
    }

    /// Try to access a field on the value.
    pub fn get_by_id(&self, id: u8) -> Option<Value> {
        todo!()
    }

    /// Try to access a field on the value.
    pub fn field(&self, field: &str) -> StrResult<Value> {
        todo!()
    }

    /// The associated scope, if this is a function, type, or module.
    pub fn scope(&self) -> Option<&Scope> {
        todo!()
    }

    /// The name, if this is a function, type, or module.
    pub fn name(&self) -> Option<&str> {
        todo!()
    }

    /// Try to extract documentation for the value.
    pub fn docs(&self) -> Option<&'static str> {
        todo!()
    }

    /// Attach a span to the value, if possible.
    pub fn spanned(self, span: Span) -> Self {
        todo!()
    }
    /// Set the label of the content.
    pub fn labelled(mut self, label: Label) -> Self {
        todo!()
    }
    /// Attach a span to the value, if possible.
    pub fn span(&self) -> Span {
        todo!()
    }

    /// Whether the contained element has the given capability.
    pub fn can<C>(&self) -> bool
    where
        C: ?Sized + 'static,
    {
        todo!()
    }

    /// Cast to a trait object if the contained element has the given
    /// capability.
    pub fn with<C>(&self) -> Option<&C>
    where
        C: ?Sized + 'static,
    {
        todo!()
    }

    /// Cast to a mutable trait object if the contained element has the given
    /// capability.
    pub fn with_mut<C>(&mut self) -> Option<&mut C>
    where
        C: ?Sized + 'static,
    {
        todo!()
    }

    pub fn location(&self) -> Option<Location> {
        todo!()
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }

    /// Extracts the plain text of this content.
    pub fn plain_text(&self) -> EcoString {
        todo!()
    }

    /// Set the location of the content.
    pub fn set_location(&mut self, location: Location) {
        todo!()
    }

    /// Disable a show rule recipe.
    pub fn guarded(mut self, guard: Guard) -> Self {
        todo!()
    }

    /// Whether the content needs to be realized specially.
    pub fn needs_preparation(&self) -> bool {
        todo!()
    }

    /// Check whether a show rule recipe is disabled.
    pub fn is_guarded(&self, guard: Guard) -> bool {
        todo!()
    }

    /// Whether no show rule was executed for this content so far.
    pub fn is_pristine(&self) -> bool {
        todo!()
    }

    /// Whether this content has already been prepared.
    pub fn is_prepared(&self) -> bool {
        todo!()
    }

    /// Mark this content as prepared.
    pub fn mark_prepared(&mut self) {
        todo!()
    }

    /// Queries the content tree for all elements that match the given selector.
    ///
    /// Elements produced in `show` rules will not be included in the results.
    pub fn query(&self, selector: Selector) -> Vec<Self> {
        todo!()
    }

    /// Queries the content tree for the first element that match the given
    /// selector.
    ///
    /// Elements produced in `show` rules will not be included in the results.
    pub fn query_first(&self, selector: Selector) -> Option<Self> {
        todo!()
    }

    /// Also auto expands sequence of sequences into flat sequence
    pub fn sequence_recursive_for_each(&self, f: &mut impl FnMut(&Self)) {
        if let Some(sequence) = self.to::<SequenceElem>() {
            for child in sequence.children() {
                child.sequence_recursive_for_each(f);
            }
        } else {
            f(self);
        }
    }

    /// Get a field by name, returning a missing field error if it does not
    /// exist.
    ///
    /// If you have access to the field IDs of the element, use [`Self::field`]
    /// instead.
    #[inline]
    pub fn field_by_name(&self, name: &str) -> StrResult<Value> {
        todo!()
    }
}

impl Value {
    /// Strongly emphasize this content.
    pub fn strong(self) -> Self {
        StrongElem::new(self).pack()
    }

    /// Emphasize this content.
    pub fn emph(self) -> Self {
        EmphElem::new(self).pack()
    }

    /// Underline this content.
    pub fn underlined(self) -> Self {
        UnderlineElem::new(self).pack()
    }

    /// Link the content somewhere.
    pub fn linked(self, dest: Destination) -> Self {
        self.styled(MetaElem::set_data(smallvec![Meta::Link(dest)]))
    }

    /// Make the content linkable by `.linked(Destination::Location(loc))`.
    ///
    /// Should be used in combination with [`Location::variant`].
    pub fn backlinked(self, loc: Location) -> Self {
        let mut backlink = Value::none();
        backlink.set_location(loc);
        self.styled(MetaElem::set_data(smallvec![Meta::Elem(backlink)]))
    }

    /// Set alignments for this content.
    pub fn aligned(self, align: Alignment) -> Self {
        self.styled(AlignElem::set_alignment(align))
    }

    /// Pad this content at the sides.
    pub fn padded(self, padding: Sides<Rel<Length>>) -> Self {
        PadElem::new(self)
            .with_left(padding.left)
            .with_top(padding.top)
            .with_right(padding.right)
            .with_bottom(padding.bottom)
            .pack()
    }

    /// Transform this content's contents without affecting layout.
    pub fn moved(self, delta: Axes<Rel<Length>>) -> Self {
        MoveElem::new(self).with_dx(delta.x).with_dy(delta.y).pack()
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl Repr for Value {
    fn repr(&self) -> EcoString {
        todo!()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        ops::equal(self, other)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        ops::compare(self, other).ok()
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

/// Visitor for value deserialization.
struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a typst value")
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_f32<E: Error>(self, v: f32) -> Result<Self::Value, E> {
        Ok((v as f64).into_value())
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(Bytes::from(v).into_value())
    }

    fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Ok(Bytes::from(v).into_value())
    }

    fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        Ok(Bytes::from(v).into_value())
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(NoneValue.into_value())
    }

    fn visit_some<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        Value::deserialize(deserializer)
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(NoneValue.into_value())
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        Ok(Array::deserialize(SeqAccessDeserializer::new(seq))?.into_value())
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let dict = Dict::deserialize(MapAccessDeserializer::new(map))?;
        Ok(match Datetime::from_toml_dict(&dict) {
            None => dict.into_value(),
            Some(datetime) => datetime.into_value(),
        })
    }
}

/// A packed value of a static type.
#[derive(Clone, PartialEq, Hash)]
#[repr(transparent)]
pub struct Packed<T>(
    /// Invariant: Must be of type `T`.
    Value,
    PhantomData<T>,
);

impl<T> Packed<T> {
    /// Pack value while retaining its static type.
    pub fn new(value: T) -> Self {
        todo!()
    }

    /// Try to cast type-erased value into a statically known packed type.
    pub fn from_ref(value: &Value) -> Option<&Self> {
        if value.is::<T>() {
            // Safety:
            // - We have checked the type.
            // - Packed<T> is repr(transparent).
            return Some(unsafe { std::mem::transmute(value) });
        }
        None
    }

    /// Try to cast type-erased value into a statically known packed type.
    pub fn from_mut(value: &mut Value) -> Option<&mut Self> {
        if value.is::<T>() {
            // Safety:
            // - We have checked the type.
            // - Packed<T> is repr(transparent).
            return Some(unsafe { std::mem::transmute(value) });
        }
        None
    }

    /// Try to cast type-erased value into a statically known packed type.
    pub fn from_owned(value: Value) -> Result<Self, Value> {
        if value.is::<T>() {
            // Safety:
            // - We have checked the type.
            // - Packed<T> is repr(transparent).
            return Ok(unsafe { std::mem::transmute(value) });
        }
        Err(value)
    }

    /// Pack back into a type-erased value.
    pub fn pack(self) -> Value {
        self.0
    }

    /// Extract the raw underlying element.
    pub fn unpack(self) -> T {
        // This function doesn't yet need owned self, but might in the future.
        todo!()
    }

    /// The element's span.
    pub fn span(&self) -> Span {
        self.0.span()
    }

    /// Set the span of the element.
    pub fn spanned(self, span: Span) -> Self {
        Self(self.0.spanned(span), PhantomData)
    }

    /// Accesses the label of the element.
    pub fn label(&self) -> Option<Label> {
        self.0.label()
    }

    /// Accesses the location of the element.
    pub fn location(&self) -> Option<Location> {
        self.0.location()
    }

    /// Sets the location of the element.
    pub fn set_location(&mut self, location: Location) {
        self.0.set_location(location);
    }
}

impl<T> AsRef<T> for Packed<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for Packed<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> Deref for Packed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> DerefMut for Packed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl<T: Debug> Debug for Packed<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
