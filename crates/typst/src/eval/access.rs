use ecow::eco_format;

use crate::diag::{bail, At, Hint, SourceResult, Trace, Tracepoint};
use crate::eval::{Eval, Vm};
use crate::foundations::{
    call_method_access, is_accessor_method, Content, Dict, Func, Module, Value,
};
use crate::symbols::Symbol;
use crate::syntax::ast::{self, AstNode};

/// Access an expression mutably.
pub(crate) trait Access {
    /// Access the value.
    fn access<'a>(self, vm: &'a mut Vm) -> SourceResult<&'a mut Value>;
}

impl Access for ast::Expr<'_> {
    fn access<'a>(self, vm: &'a mut Vm) -> SourceResult<&'a mut Value> {
        match self {
            Self::Ident(v) => v.access(vm),
            Self::Parenthesized(v) => v.access(vm),
            Self::FieldAccess(v) => v.access(vm),
            Self::FuncCall(v) => v.access(vm),
            _ => {
                let _ = self.eval(vm)?;
                bail!(self.span(), "cannot mutate a temporary value");
            }
        }
    }
}

impl Access for ast::Ident<'_> {
    fn access<'a>(self, vm: &'a mut Vm) -> SourceResult<&'a mut Value> {
        let span = self.span();
        let value = vm.scopes.get_mut(&self).at(span)?;
        if vm.inspected == Some(span) {
            vm.engine.tracer.value(value.clone());
        }
        Ok(value)
    }
}

impl Access for ast::Parenthesized<'_> {
    fn access<'a>(self, vm: &'a mut Vm) -> SourceResult<&'a mut Value> {
        self.expr().access(vm)
    }
}

impl Access for ast::FieldAccess<'_> {
    fn access<'a>(self, vm: &'a mut Vm) -> SourceResult<&'a mut Value> {
        access_dict(vm, self)?.at_mut(self.field().get()).at(self.span())
    }
}

impl Access for ast::FuncCall<'_> {
    fn access<'a>(self, vm: &'a mut Vm) -> SourceResult<&'a mut Value> {
        if let ast::Expr::FieldAccess(access) = self.callee() {
            let method = access.field();
            if is_accessor_method(&method) {
                let span = self.span();
                let world = vm.world();
                let args = self.args().eval(vm)?;
                let value = access.target().access(vm)?;
                let result = call_method_access(value, &method, args, span);
                let point = || Tracepoint::Call(Some(method.get().clone()));
                return result.trace(world, point, span);
            }
        }

        let _ = self.eval(vm)?;
        bail!(self.span(), "cannot mutate a temporary value");
    }
}

pub(crate) fn access_dict<'a>(
    vm: &'a mut Vm,
    access: ast::FieldAccess,
) -> SourceResult<&'a mut Dict> {
    let value = access.target().access(vm)?;
    if value.is::<Dict>() {
        return Ok(value.to_mut::<Dict>().unwrap());
    }

    let ty = value.ty();
    let span = access.target().span();

    // Those types have their own field getters
    if value.is::<Symbol>()
        || value.is::<Content>()
        || value.is::<Module>()
        || value.is::<Func>()
    {
        bail!(span, "cannot mutate fields on {ty}");
    } else if crate::foundations::fields_on(ty).is_empty() {
        bail!(span, "{ty} does not have accessible fields");
    } else {
        // type supports static fields, which don't yet have
        // setters
        Err(eco_format!("fields on {ty} are not yet mutable"))
            .hint(eco_format!(
                "try creating a new {ty} with the updated field value instead"
            ))
            .at(span)
    }
}
