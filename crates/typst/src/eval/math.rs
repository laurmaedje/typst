use ecow::eco_format;

use crate::diag::{At, SourceResult};
use crate::eval::{Eval, Vm};
use crate::foundations::Value;
use crate::math::{AlignPointElem, AttachElem, FracElem, LrElem, PrimesElem, RootElem};
use crate::syntax::ast::{self, AstNode};
use crate::text::TextElem;

impl Eval for ast::Math<'_> {
    type Output = Value;
    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        Ok(Value::sequence(
            self.exprs()
                .map(|expr| expr.eval(vm))
                .collect::<SourceResult<Vec<_>>>()?,
        ))
    }
}

impl Eval for ast::MathIdent<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        vm.scopes.get_in_math(&self).cloned().at(self.span())
    }
}

impl Eval for ast::MathAlignPoint<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(AlignPointElem::new().pack())
    }
}

impl Eval for ast::MathDelimited<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let open = self.open().eval(vm)?;
        let body = self.body().eval(vm)?;
        let close = self.close().eval(vm)?;
        Ok(LrElem::new(Value::sequence([open, body, close])).pack())
    }
}

impl Eval for ast::MathAttach<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let base = self.base().eval(vm)?;
        let mut elem = AttachElem::new(base);

        if let Some(expr) = self.top() {
            elem.push_t(Some(expr.eval(vm)?));
        } else if let Some(primes) = self.primes() {
            elem.push_t(Some(primes.eval(vm)?));
        }

        if let Some(expr) = self.bottom() {
            elem.push_b(Some(expr.eval(vm)?));
        }

        Ok(elem.pack())
    }
}

impl Eval for ast::MathPrimes<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(PrimesElem::new(self.count()).pack())
    }
}

impl Eval for ast::MathFrac<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let num = self.num().eval(vm)?;
        let denom = self.denom().eval(vm)?;
        Ok(FracElem::new(num, denom).pack())
    }
}

impl Eval for ast::MathRoot<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let index = self.index().map(|i| TextElem::packed(eco_format!("{i}")));
        let radicand = self.radicand().eval(vm)?;
        Ok(RootElem::new(radicand).with_index(index).pack())
    }
}
