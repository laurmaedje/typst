use std::collections::HashSet;

use crate::diag::{bail, At, SourceResult};
use crate::eval::{Access, Eval, Vm};
use crate::foundations::{Array, Dict, IntoValue, NoneValue, Value};
use crate::syntax::ast::{self, AstNode};

impl Eval for ast::LetBinding<'_> {
    type Output = NoneValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let value = match self.init() {
            Some(expr) => expr.eval(vm)?,
            None => NoneValue.into_value(),
        };

        if vm.flow.is_some() {
            return Ok(NoneValue);
        }

        match self.kind() {
            ast::LetBindingKind::Normal(pattern) => destructure(vm, pattern, value)?,
            ast::LetBindingKind::Closure(ident) => vm.define(ident, value),
        }

        Ok(NoneValue)
    }
}

impl Eval for ast::DestructAssignment<'_> {
    type Output = NoneValue;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let value = self.value().eval(vm)?;
        destructure_impl(vm, self.pattern(), value, |vm, expr, value| {
            let location = expr.access(vm)?;
            *location = value;
            Ok(())
        })?;
        Ok(NoneValue)
    }
}

/// Destructures a value into a pattern.
pub(crate) fn destructure(
    vm: &mut Vm,
    pattern: ast::Pattern,
    value: Value,
) -> SourceResult<()> {
    destructure_impl(vm, pattern, value, |vm, expr, value| match expr {
        ast::Expr::Ident(ident) => {
            vm.define(ident, value);
            Ok(())
        }
        _ => bail!(expr.span(), "nested patterns are currently not supported"),
    })
}

/// Destruct the given value into the pattern and apply the function to each binding.
fn destructure_impl<T>(
    vm: &mut Vm,
    pattern: ast::Pattern,
    value: Value,
    f: T,
) -> SourceResult<()>
where
    T: Fn(&mut Vm, ast::Expr, Value) -> SourceResult<()>,
{
    match pattern {
        ast::Pattern::Normal(expr) => {
            f(vm, expr, value)?;
        }
        ast::Pattern::Placeholder(_) => {}
        ast::Pattern::Destructuring(destruct) => {
            if value.is::<Array>() {
                let array = value.to_packed::<Array>().unwrap().unpack();
                destructure_array(vm, pattern, array, f, destruct)?;
            } else if value.is::<Dict>() {
                let dict = value.to_packed::<Dict>().unwrap().unpack();
                destructure_dict(vm, dict, f, destruct)?;
            } else {
                bail!(pattern.span(), "cannot destructure {}", value.ty());
            }
        }
    }
    Ok(())
}

fn destructure_array<F>(
    vm: &mut Vm,
    pattern: ast::Pattern,
    value: Array,
    f: F,
    destruct: ast::Destructuring,
) -> SourceResult<()>
where
    F: Fn(&mut Vm, ast::Expr, Value) -> SourceResult<()>,
{
    let mut i = 0;
    let len = value.as_slice().len();
    for p in destruct.bindings() {
        match p {
            ast::DestructuringKind::Normal(expr) => {
                let Ok(v) = value.at(i as i64, None) else {
                    bail!(expr.span(), "not enough elements to destructure");
                };
                f(vm, expr, v)?;
                i += 1;
            }
            ast::DestructuringKind::Sink(spread) => {
                let sink_size = (1 + len).checked_sub(destruct.bindings().count());
                let sink = sink_size.and_then(|s| value.as_slice().get(i..i + s));
                if let (Some(sink_size), Some(sink)) = (sink_size, sink) {
                    if let Some(expr) = spread.expr() {
                        f(vm, expr, Array::from(sink).into_value())?;
                    }
                    i += sink_size;
                } else {
                    bail!(pattern.span(), "not enough elements to destructure")
                }
            }
            ast::DestructuringKind::Named(named) => {
                bail!(named.span(), "cannot destructure named elements from an array")
            }
            ast::DestructuringKind::Placeholder(underscore) => {
                if i < len {
                    i += 1
                } else {
                    bail!(underscore.span(), "not enough elements to destructure")
                }
            }
        }
    }
    if i < len {
        bail!(pattern.span(), "too many elements to destructure");
    }

    Ok(())
}

fn destructure_dict<F>(
    vm: &mut Vm,
    dict: Dict,
    f: F,
    destruct: ast::Destructuring,
) -> SourceResult<()>
where
    F: Fn(&mut Vm, ast::Expr, Value) -> SourceResult<()>,
{
    let mut sink = None;
    let mut used = HashSet::new();
    for p in destruct.bindings() {
        match p {
            ast::DestructuringKind::Normal(ast::Expr::Ident(ident)) => {
                let v = dict.get(&ident).at(ident.span())?;
                f(vm, ast::Expr::Ident(ident), v.clone())?;
                used.insert(ident.as_str());
            }
            ast::DestructuringKind::Sink(spread) => sink = spread.expr(),
            ast::DestructuringKind::Named(named) => {
                let name = named.name();
                let v = dict.get(&name).at(name.span())?;
                f(vm, named.expr(), v.clone())?;
                used.insert(name.as_str());
            }
            ast::DestructuringKind::Placeholder(_) => {}
            ast::DestructuringKind::Normal(expr) => {
                bail!(expr.span(), "expected key, found expression");
            }
        }
    }

    if let Some(expr) = sink {
        let mut sink = Dict::new();
        for (key, value) in dict {
            if !used.contains(key.as_str()) {
                sink.insert(key, value);
            }
        }
        f(vm, expr, sink.into_value())?;
    }

    Ok(())
}
