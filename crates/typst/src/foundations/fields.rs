//! Fields on values.

use ecow::{eco_format, EcoString};

use crate::diag::StrResult;
use crate::foundations::{IntoValue, Type, Value, Version};
use crate::layout::{Alignment, Length, Rel};
use crate::visualize::Stroke;

/// Try to access a field on a value.
///
/// This function is exclusively for types which have predefined fields, such as
/// stroke and length.
pub(crate) fn field(value: &Value, field: &str) -> StrResult<Value> {
    let ty = value.ty();
    let nope = || Err(no_fields(ty));
    let missing = || Err(missing_field(ty, field));

    // Special cases, such as module and dict, are handled by Value itself
    let result = if let Some(version) = value.to::<Version>() {
        match version.component(field) {
            Ok(i) => i.into_value(),
            Err(_) => return missing(),
        }
    } else if let Some(length) = value.to::<Length>() {
        match field {
            "em" => length.em.get().into_value(),
            "abs" => length.abs.into_value(),
            _ => return missing(),
        }
    } else if let Some(rel) = value.to::<Rel>() {
        match field {
            "ratio" => rel.rel.into_value(),
            "length" => rel.abs.into_value(),
            _ => return missing(),
        }
    } else if let Some(stroke) = value.to::<Stroke>() {
        match field {
            "paint" => stroke.paint.clone().into_value(),
            "thickness" => stroke.thickness.into_value(),
            "cap" => stroke.cap.into_value(),
            "join" => stroke.join.into_value(),
            "dash" => stroke.dash.clone().into_value(),
            "miter-limit" => stroke.miter_limit.map(|limit| limit.get()).into_value(),
            _ => return missing(),
        }
    } else if let Some(align) = value.to::<Alignment>() {
        match field {
            "x" => align.x().into_value(),
            "y" => align.y().into_value(),
            _ => return missing(),
        }
    } else {
        return nope();
    };

    Ok(result)
}

/// The error message for a type not supporting field access.
#[cold]
fn no_fields(ty: Type) -> EcoString {
    eco_format!("cannot access fields on type {ty}")
}

/// The missing field error message.
#[cold]
fn missing_field(ty: Type, field: &str) -> EcoString {
    eco_format!("{ty} does not contain field \"{field}\"")
}

/// List the available fields for a type.
pub fn fields_on(ty: Type) -> &'static [&'static str] {
    if ty == Type::of::<Version>() {
        &Version::COMPONENTS
    } else if ty == Type::of::<Length>() {
        &["em", "abs"]
    } else if ty == Type::of::<Rel>() {
        &["ratio", "length"]
    } else if ty == Type::of::<Stroke>() {
        &["paint", "thickness", "cap", "join", "dash", "miter-limit"]
    } else if ty == Type::of::<Alignment>() {
        &["x", "y"]
    } else {
        &[]
    }
}
