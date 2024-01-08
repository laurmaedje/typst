use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

use ecow::EcoString;
use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::diag::StrResult;
use crate::eval::ops;
use crate::foundations::{
    Args, Array, AutoValue, Bytes, CastInfo, Content, Datetime, Dict, Duration,
    FromValue, Func, IntoValue, Label, Module, NoneValue, Plugin, Reflect, Repr, Scope,
    Str, Styles, Type, Version,
};
use crate::layout::{Abs, Angle, Em, Fr, Length, Ratio, Rel};
use crate::symbols::Symbol;
use crate::syntax::{ast, Span};
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
    pub fn is<T: 'static>(&self) -> bool {
        todo!()
    }

    /// Cast the value to a concrete type.
    pub fn to<T: 'static>(&self) -> Option<&T> {
        todo!()
    }

    /// Cast the value to a concrete type.
    pub fn to_mut<T: 'static>(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Cast the value to a concrete type.
    pub fn unpack<T: 'static>(self) -> Result<T, Self> {
        todo!()
    }

    /// The type of this value.
    pub fn ty(&self) -> Type {
        todo!()
    }

    pub fn display(self) -> Content {
        todo!()
    }

    /// Try to cast the value into a specific type.
    pub fn cast<T: FromValue>(self) -> StrResult<T> {
        T::from_value(self)
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

/// Implements traits for primitives (Value enum variants).
macro_rules! primitive {
    (
        $ty:ty: $name:literal, $variant:ident
        $(, $other:ident$(($binding:ident))? => $out:expr)*
    ) => {
        impl Reflect for $ty {
            fn input() -> CastInfo {
                todo!()
            }

            fn output() -> CastInfo {
                todo!()
            }

            fn castable(value: &Value) -> bool {
                todo!()
            }
        }

        impl IntoValue for $ty {
            fn into_value(self) -> Value {
                todo!()
            }
        }

        impl FromValue for $ty {
            fn from_value(value: Value) -> StrResult<Self> {
                todo!()
            }
        }
    };

    (@$other:ident($binding:ident)) => {
        Value::$other(_)
    };
    (@$other:ident) => {
        Value::$other
    };
}

primitive! { NoneValue: "none", None }
primitive! { AutoValue: "auto", Auto }
primitive! { bool: "boolean", Bool }
primitive! { i64: "integer", Int }
primitive! { f64: "float", Float, Int(v) => v as f64 }
primitive! { Length: "length", Length }
primitive! { Angle: "angle", Angle }
primitive! { Ratio: "ratio", Ratio }
primitive! { Rel<Length>:  "relative length",
    Relative,
    Length(v) => v.into(),
    Ratio(v) => v.into()
}
primitive! { Fr: "fraction", Fraction }
primitive! { Color: "color", Color }
primitive! { Gradient: "gradient", Gradient }
primitive! { Pattern: "pattern", Pattern }
primitive! { Symbol: "symbol", Symbol }
primitive! { Version: "version", Version }
primitive! {
    Str: "string",
    Str,
    Symbol(symbol) => symbol.get().into()
}
primitive! { Bytes: "bytes", Bytes }
primitive! { Label: "label", Label }
primitive! { Datetime: "datetime", Datetime }
primitive! { Duration: "duration", Duration }
primitive! { Content: "content",
    Content,
    None => Content::empty(),
    Symbol(v) => TextElem::packed(v.get()),
    Str(v) => TextElem::packed(v)
}
primitive! { Styles: "styles", Styles }
primitive! { Array: "array", Array }
primitive! { Dict: "dictionary", Dict }
primitive! {
    Func: "function",
    Func,
    Type(ty) => ty.constructor()?.clone()
}
primitive! { Args: "arguments", Args }
primitive! { Type: "type", Type }
primitive! { Module: "module", Module }
primitive! { Plugin: "plugin", Plugin }
