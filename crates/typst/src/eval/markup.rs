use crate::diag::{warning, SourceResult};
use crate::eval::{Eval, Vm};
use crate::foundations::{Label, Smart, Unlabellable, Value};
use crate::math::EquationElem;
use crate::model::{
    EmphElem, EnumItem, HeadingElem, LinkElem, ListItem, ParbreakElem, RefElem,
    StrongElem, Supplement, TermItem,
};
use crate::symbols::Symbol;
use crate::syntax::ast::{self, AstNode};
use crate::text::{LinebreakElem, RawElem, SmartQuoteElem, SpaceElem, TextElem};

impl Eval for ast::Markup<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        eval_markup(vm, &mut self.exprs())
    }
}

/// Evaluate a stream of markup.
fn eval_markup<'a>(
    vm: &mut Vm,
    exprs: &mut impl Iterator<Item = ast::Expr<'a>>,
) -> SourceResult<Value> {
    let flow = vm.flow.take();
    let mut seq = Vec::with_capacity(exprs.size_hint().1.unwrap_or_default());

    while let Some(expr) = exprs.next() {
        match expr {
            ast::Expr::Set(set) => {
                let styles = set.eval(vm)?;
                if vm.flow.is_some() {
                    break;
                }

                seq.push(eval_markup(vm, exprs)?.styled_with_map(styles))
            }
            ast::Expr::Show(show) => {
                let recipe = show.eval(vm)?;
                if vm.flow.is_some() {
                    break;
                }

                let tail = eval_markup(vm, exprs)?;
                seq.push(tail.styled_with_recipe(&mut vm.engine, recipe)?)
            }
            expr => {
                let value = expr.eval(vm)?;
                if let Some(label) = value.to::<Label>() {
                    if let Some(elem) =
                        seq.iter_mut().rev().find(|node| !node.can::<dyn Unlabellable>())
                    {
                        *elem = std::mem::take(elem).labelled(**label);
                    }
                } else {
                    seq.push(value.spanned(expr.span()))
                }
            }
        }

        if vm.flow.is_some() {
            break;
        }
    }

    if flow.is_some() {
        vm.flow = flow;
    }

    Ok(Value::sequence(seq))
}

impl Eval for ast::Text<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(TextElem::packed(self.get().clone()))
    }
}

impl Eval for ast::Space<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(SpaceElem::new().pack())
    }
}

impl Eval for ast::Linebreak<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(LinebreakElem::new().pack())
    }
}

impl Eval for ast::Parbreak<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(ParbreakElem::new().pack())
    }
}

impl Eval for ast::Escape<'_> {
    type Output = Symbol;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(Symbol::single(self.get()))
    }
}

impl Eval for ast::Shorthand<'_> {
    type Output = Symbol;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(Symbol::single(self.get()))
    }
}

impl Eval for ast::SmartQuote<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(SmartQuoteElem::new().with_double(self.double()).pack())
    }
}

impl Eval for ast::Strong<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let body = self.body();
        if body.exprs().next().is_none() {
            vm.engine
                .tracer
                .warn(warning!(
                    self.span(), "no text within stars";
                    hint: "using multiple consecutive stars (e.g. **) has no additional effect",
                ));
        }

        Ok(StrongElem::new(body.eval(vm)?).pack())
    }
}

impl Eval for ast::Emph<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let body = self.body();
        if body.exprs().next().is_none() {
            vm.engine
                .tracer
                .warn(warning!(
                    self.span(), "no text within underscores";
                    hint: "using multiple consecutive underscores (e.g. __) has no additional effect"
                ));
        }

        Ok(EmphElem::new(body.eval(vm)?).pack())
    }
}

impl Eval for ast::Raw<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        let mut elem = RawElem::new(self.text()).with_block(self.block());
        if let Some(lang) = self.lang() {
            elem.push_lang(Some(lang.into()));
        }
        Ok(elem.pack())
    }
}

impl Eval for ast::Link<'_> {
    type Output = Value;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(LinkElem::from_url(self.get().clone()).pack())
    }
}

impl Eval for ast::Label<'_> {
    type Output = Label;

    fn eval(self, _: &mut Vm) -> SourceResult<Self::Output> {
        Ok(Label::new(self.get()))
    }
}

impl Eval for ast::Ref<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let target = Label::new(self.target());
        let mut elem = RefElem::new(target);
        if let Some(supplement) = self.supplement() {
            elem.push_supplement(Smart::Custom(Some(Supplement::Content(
                supplement.eval(vm)?,
            ))));
        }
        Ok(elem.pack())
    }
}

impl Eval for ast::Heading<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let level = self.level();
        let body = self.body().eval(vm)?;
        Ok(HeadingElem::new(body).with_level(level).pack())
    }
}

impl Eval for ast::ListItem<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        Ok(ListItem::new(self.body().eval(vm)?).pack())
    }
}

impl Eval for ast::EnumItem<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let body = self.body().eval(vm)?;
        let mut elem = EnumItem::new(body);
        if let Some(number) = self.number() {
            elem.push_number(Some(number));
        }
        Ok(elem.pack())
    }
}

impl Eval for ast::TermItem<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let term = self.term().eval(vm)?;
        let description = self.description().eval(vm)?;
        Ok(TermItem::new(term, description).pack())
    }
}

impl Eval for ast::Equation<'_> {
    type Output = Value;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let body = self.body().eval(vm)?;
        let block = self.block();
        Ok(EquationElem::new(body).with_block(block).pack())
    }
}
