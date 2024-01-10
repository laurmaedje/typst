use std::fmt::{self, Debug, Formatter};

use ecow::EcoString;
use serde::{Serialize, Serializer};

use crate::diag::StrResult;
use crate::foundations::{
    cast, ty, CastInfo, FromValue, IntoValue, Reflect, Repr, Value,
};

/// A value that indicates the absence of any other value.
///
/// The none type has exactly one value: `{none}`.
///
/// When inserted into the document, it is not visible. This is also the value
/// that is produced by empty code blocks. It can be
/// [joined]($scripting/#blocks) with any value, yielding the other value.
///
/// # Example
/// ```example
/// Not visible: #none
/// ```
#[ty(cast, name = "none")]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NoneValue;

impl Debug for NoneValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("None")
    }
}

impl Repr for NoneValue {
    fn repr(&self) -> EcoString {
        "none".into()
    }
}

impl Serialize for NoneValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

cast! {
    (),
    self => NoneValue.into_value(),
    _: NoneValue => (),
}

impl<T: Reflect> Reflect for Option<T> {
    fn input() -> CastInfo {
        T::input() + <NoneValue as Reflect>::input()
    }

    fn output() -> CastInfo {
        T::output() + <NoneValue as Reflect>::output()
    }

    fn castable(value: &Value) -> bool {
        <NoneValue as Reflect>::castable(value) || T::castable(value)
    }
}

impl<T: IntoValue> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into_value(),
            None => NoneValue.into_value(),
        }
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> StrResult<Self> {
        if value.is::<NoneValue>() {
            Ok(None)
        } else if T::castable(&value) {
            Ok(Some(T::from_value(value)?))
        } else {
            Err(Self::error(&value))
        }
    }
}
