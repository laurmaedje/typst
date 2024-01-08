//! Operations on values.

use std::cmp::Ordering;

use ecow::eco_format;

use crate::diag::{At, SourceResult, StrResult};
use crate::eval::{access_dict, Access, Eval, Vm};
use crate::foundations::{
    format_str, Datetime, IntoValue, NoneValue, Regex, Repr, Value,
};
use crate::layout::{Length, Rel};
use crate::syntax::ast::{self, AstNode};

impl Eval for ast::Unary<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let value = self.expr().eval(vm)?;
        let result = match self.op() {
            ast::UnOp::Pos => pos(value),
            ast::UnOp::Neg => neg(value),
            ast::UnOp::Not => not(value),
        };
        result.at(self.span())
    }
}

impl Eval for ast::Binary<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        match self.op() {
            ast::BinOp::Add => apply_binary(self, vm, add),
            ast::BinOp::Sub => apply_binary(self, vm, sub),
            ast::BinOp::Mul => apply_binary(self, vm, mul),
            ast::BinOp::Div => apply_binary(self, vm, div),
            ast::BinOp::And => apply_binary(self, vm, and),
            ast::BinOp::Or => apply_binary(self, vm, or),
            ast::BinOp::Eq => apply_binary(self, vm, eq),
            ast::BinOp::Neq => apply_binary(self, vm, neq),
            ast::BinOp::Lt => apply_binary(self, vm, lt),
            ast::BinOp::Leq => apply_binary(self, vm, leq),
            ast::BinOp::Gt => apply_binary(self, vm, gt),
            ast::BinOp::Geq => apply_binary(self, vm, geq),
            ast::BinOp::In => apply_binary(self, vm, in_),
            ast::BinOp::NotIn => apply_binary(self, vm, not_in),
            ast::BinOp::Assign => apply_assignment(self, vm, |_, b| Ok(b)),
            ast::BinOp::AddAssign => apply_assignment(self, vm, add),
            ast::BinOp::SubAssign => apply_assignment(self, vm, sub),
            ast::BinOp::MulAssign => apply_assignment(self, vm, mul),
            ast::BinOp::DivAssign => apply_assignment(self, vm, div),
        }
    }
}

/// Apply a basic binary operation.
fn apply_binary(
    binary: ast::Binary,
    vm: &mut Vm,
    op: fn(Value, Value) -> StrResult<Value>,
) -> SourceResult<Value> {
    let lhs = binary.lhs().eval(vm)?;

    // Short-circuit boolean operations.
    if (binary.op() == ast::BinOp::And && lhs == false.into_value())
        || (binary.op() == ast::BinOp::Or && lhs == true.into_value())
    {
        return Ok(lhs);
    }

    let rhs = binary.rhs().eval(vm)?;
    op(lhs, rhs).at(binary.span())
}

/// Apply an assignment operation.
fn apply_assignment(
    binary: ast::Binary,
    vm: &mut Vm,
    op: fn(Value, Value) -> StrResult<Value>,
) -> SourceResult<Value> {
    let rhs = binary.rhs().eval(vm)?;
    let lhs = binary.lhs();

    // An assignment to a dictionary field is different from a normal access
    // since it can create the field instead of just modifying it.
    if binary.op() == ast::BinOp::Assign {
        if let ast::Expr::FieldAccess(access) = lhs {
            let dict = access_dict(vm, access)?;
            dict.insert(access.field().get().clone().into(), rhs);
            return Ok(NoneValue.into_value());
        }
    }

    let location = binary.lhs().access(vm)?;
    let lhs = std::mem::take(&mut *location);
    *location = op(lhs, rhs).at(binary.span())?;
    Ok(NoneValue.into_value())
}

/// Bail with a type mismatch error.
macro_rules! mismatch {
    ($fmt:expr, $($value:expr),* $(,)?) => {
        return Err(eco_format!($fmt, $($value.ty()),*))
    };
}

/// Join a value with another value.
pub fn join(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Apply the unary plus operator to a value.
pub fn pos(value: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the negation of a value.
pub fn neg(value: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the sum of two values.
pub fn add(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the difference of two values.
pub fn sub(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the product of two values.
pub fn mul(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the quotient of two values.
pub fn div(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Whether a value is a numeric zero.
fn is_zero(v: &Value) -> bool {
    todo!()
}

/// Try to divide two lengths.
fn try_div_length(a: Length, b: Length) -> StrResult<f64> {
    a.try_div(b).ok_or_else(|| "cannot divide these two lengths".into())
}

/// Try to divide two relative lengths.
fn try_div_relative(a: Rel<Length>, b: Rel<Length>) -> StrResult<f64> {
    a.try_div(b)
        .ok_or_else(|| "cannot divide these two relative lengths".into())
}

/// Compute the logical "not" of a value.
pub fn not(value: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the logical "and" of two values.
pub fn and(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Compute the logical "or" of two values.
pub fn or(lhs: Value, rhs: Value) -> StrResult<Value> {
    todo!()
}

/// Compute whether two values are equal.
pub fn eq(lhs: Value, rhs: Value) -> StrResult<Value> {
    Ok(equal(&lhs, &rhs).into_value())
}

/// Compute whether two values are unequal.
pub fn neq(lhs: Value, rhs: Value) -> StrResult<Value> {
    Ok((!equal(&lhs, &rhs)).into_value())
}

macro_rules! comparison {
    ($name:ident, $op:tt, $($pat:tt)*) => {
        /// Compute how a value compares with another value.
        pub fn $name(lhs: Value, rhs: Value) -> StrResult<Value> {
            todo!()
        }
    };
}

comparison!(lt, "<", Ordering::Less);
comparison!(leq, "<=", Ordering::Less | Ordering::Equal);
comparison!(gt, ">", Ordering::Greater);
comparison!(geq, ">=", Ordering::Greater | Ordering::Equal);

/// Determine whether two values are equal.
pub fn equal(lhs: &Value, rhs: &Value) -> bool {
    todo!()
}

/// Compare two values.
pub fn compare(lhs: &Value, rhs: &Value) -> StrResult<Ordering> {
    todo!()
}

/// Try to compare two values.
fn try_cmp_values<T: PartialOrd + Repr>(a: &T, b: &T) -> StrResult<Ordering> {
    a.partial_cmp(b)
        .ok_or_else(|| eco_format!("cannot compare {} with {}", a.repr(), b.repr()))
}

/// Try to compare two datetimes.
fn try_cmp_datetimes(a: &Datetime, b: &Datetime) -> StrResult<Ordering> {
    a.partial_cmp(b)
        .ok_or_else(|| eco_format!("cannot compare {} and {}", a.kind(), b.kind()))
}

/// Try to compare arrays of values lexicographically.
fn try_cmp_arrays(a: &[Value], b: &[Value]) -> StrResult<Ordering> {
    a.iter()
        .zip(b.iter())
        .find_map(|(first, second)| {
            match compare(first, second) {
                // Keep searching for a pair of elements that isn't equal.
                Ok(Ordering::Equal) => None,
                // Found a pair which either is not equal or not comparable, so
                // we stop searching.
                result => Some(result),
            }
        })
        .unwrap_or_else(|| {
            // The two arrays are equal up to the shortest array's extent,
            // so compare their lengths instead.
            Ok(a.len().cmp(&b.len()))
        })
}

/// Test whether one value is "in" another one.
pub fn in_(lhs: Value, rhs: Value) -> StrResult<Value> {
    if let Some(b) = contains(&lhs, &rhs) {
        Ok(b.into_value())
    } else {
        mismatch!("cannot apply 'in' to {} and {}", lhs, rhs)
    }
}

/// Test whether one value is "not in" another one.
pub fn not_in(lhs: Value, rhs: Value) -> StrResult<Value> {
    if let Some(b) = contains(&lhs, &rhs) {
        Ok((!b).into_value())
    } else {
        mismatch!("cannot apply 'not in' to {} and {}", lhs, rhs)
    }
}

/// Test for containment.
pub fn contains(lhs: &Value, rhs: &Value) -> Option<bool> {
    todo!()
}

#[cold]
fn too_large() -> &'static str {
    "value is too large"
}
