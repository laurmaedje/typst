//! Evaluation of markup into modules.

#[macro_use]
mod array;
#[macro_use]
mod dict;
#[macro_use]
mod value;

mod args;
mod capture;
mod control;
mod func;
pub mod methods;
mod module;
pub mod ops;
mod raw;
mod scope;
mod str;

pub use self::str::*;
pub use args::*;
pub use array::*;
pub use capture::*;
pub use control::*;
pub use dict::*;
pub use func::*;
pub use module::*;
pub use raw::*;
pub use scope::*;
pub use value::*;

use std::collections::BTreeMap;

use parking_lot::{MappedRwLockWriteGuard, RwLockWriteGuard};
use unicode_segmentation::UnicodeSegmentation;

use crate::diag::{At, StrResult, Trace, Tracepoint, TypResult};
use crate::geom::{Angle, Em, Fraction, Length, Ratio};
use crate::library;
use crate::model::{Content, StyleMap};
use crate::syntax::ast::*;
use crate::syntax::{Span, Spanned};
use crate::util::EcoString;
use crate::Context;

/// Evaluate an expression.
pub trait Eval {
    /// The output of evaluating the expression.
    type Output;

    /// Evaluate the expression to the output value.
    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output>;
}

/// The result type for evaluating a syntactic construct.
pub type EvalResult<T> = Result<T, Control>;

impl Eval for Markup {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        eval_markup(ctx, scp, &mut self.nodes())
    }
}

/// Evaluate a stream of markup nodes.
fn eval_markup(
    ctx: &mut Context,
    scp: &mut Scopes,
    nodes: &mut impl Iterator<Item = MarkupNode>,
) -> EvalResult<Content> {
    let mut seq = Vec::with_capacity(nodes.size_hint().1.unwrap_or_default());

    while let Some(node) = nodes.next() {
        seq.push(match node {
            MarkupNode::Expr(Expr::Set(set)) => {
                let styles = set.eval(ctx, scp)?;
                eval_markup(ctx, scp, nodes)?.styled_with_map(styles)
            }
            MarkupNode::Expr(Expr::Show(show)) => {
                let styles = show.eval(ctx, scp)?;
                eval_markup(ctx, scp, nodes)?.styled_with_map(styles)
            }
            MarkupNode::Expr(Expr::Wrap(wrap)) => {
                let tail = eval_markup(ctx, scp, nodes)?;
                scp.top.def_mut(wrap.binding().take(), tail);
                wrap.body().eval(ctx, scp)?.display()
            }
            _ => node.eval(ctx, scp)?,
        });
    }

    Ok(Content::sequence(seq))
}

impl Eval for MarkupNode {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(match self {
            Self::Space => Content::Space,
            Self::Parbreak => Content::Parbreak,
            Self::Linebreak(soft) => Content::Linebreak(*soft),
            Self::Text(text) => Content::Text(text.clone()),
            Self::Quote(double) => Content::Quote(*double),
            Self::Strong(strong) => strong.eval(ctx, scp)?,
            Self::Emph(emph) => emph.eval(ctx, scp)?,
            Self::Raw(raw) => raw.eval(ctx, scp)?,
            Self::Math(math) => math.eval(ctx, scp)?,
            Self::Heading(heading) => heading.eval(ctx, scp)?,
            Self::List(list) => list.eval(ctx, scp)?,
            Self::Enum(enum_) => enum_.eval(ctx, scp)?,
            Self::Expr(expr) => expr.eval(ctx, scp)?.display(),
        })
    }
}

impl Eval for StrongNode {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(Content::show(library::text::StrongNode(
            self.body().eval(ctx, scp)?,
        )))
    }
}

impl Eval for EmphNode {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(Content::show(library::text::EmphNode(
            self.body().eval(ctx, scp)?,
        )))
    }
}

impl Eval for RawNode {
    type Output = Content;

    fn eval(&self, _: &mut Context, _: &mut Scopes) -> EvalResult<Self::Output> {
        let content = Content::show(library::text::RawNode {
            text: self.text.clone(),
            block: self.block,
        });
        Ok(match self.lang {
            Some(_) => content.styled(library::text::RawNode::LANG, self.lang.clone()),
            None => content,
        })
    }
}

impl Eval for MathNode {
    type Output = Content;

    fn eval(&self, _: &mut Context, _: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(Content::show(library::math::MathNode {
            formula: self.formula.clone(),
            display: self.display,
        }))
    }
}

impl Eval for HeadingNode {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(Content::show(library::structure::HeadingNode {
            body: self.body().eval(ctx, scp)?,
            level: self.level(),
        }))
    }
}

impl Eval for ListNode {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(Content::List(library::structure::ListItem {
            number: None,
            body: Box::new(self.body().eval(ctx, scp)?),
        }))
    }
}

impl Eval for EnumNode {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(Content::Enum(library::structure::ListItem {
            number: self.number(),
            body: Box::new(self.body().eval(ctx, scp)?),
        }))
    }
}

impl Eval for Expr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        match self {
            Self::Lit(v) => v.eval(ctx, scp),
            Self::Ident(v) => v.eval(ctx, scp),
            Self::Code(v) => v.eval(ctx, scp),
            Self::Content(v) => v.eval(ctx, scp).map(Value::Content),
            Self::Array(v) => v.eval(ctx, scp).map(Value::Array),
            Self::Dict(v) => v.eval(ctx, scp).map(Value::Dict),
            Self::Group(v) => v.eval(ctx, scp),
            Self::FieldAccess(v) => v.eval(ctx, scp),
            Self::FuncCall(v) => v.eval(ctx, scp),
            Self::MethodCall(v) => v.eval(ctx, scp),
            Self::Closure(v) => v.eval(ctx, scp),
            Self::Unary(v) => v.eval(ctx, scp),
            Self::Binary(v) => v.eval(ctx, scp),
            Self::Let(v) => v.eval(ctx, scp),
            Self::Set(_) | Self::Show(_) | Self::Wrap(_) => {
                Err("set, show and wrap are only allowed directly in markup")
                    .at(self.span())
                    .map_err(Into::into)
            }
            Self::If(v) => v.eval(ctx, scp),
            Self::While(v) => v.eval(ctx, scp),
            Self::For(v) => v.eval(ctx, scp),
            Self::Import(v) => v.eval(ctx, scp),
            Self::Include(v) => v.eval(ctx, scp).map(Value::Content),
            Self::Break(v) => v.eval(ctx, scp),
            Self::Continue(v) => v.eval(ctx, scp),
            Self::Return(v) => v.eval(ctx, scp),
        }
    }
}

impl Eval for Lit {
    type Output = Value;

    fn eval(&self, _: &mut Context, _: &mut Scopes) -> EvalResult<Self::Output> {
        Ok(match self.kind() {
            LitKind::None => Value::None,
            LitKind::Auto => Value::Auto,
            LitKind::Bool(v) => Value::Bool(v),
            LitKind::Int(v) => Value::Int(v),
            LitKind::Float(v) => Value::Float(v),
            LitKind::Numeric(v, unit) => match unit {
                Unit::Length(unit) => Length::with_unit(v, unit).into(),
                Unit::Angle(unit) => Angle::with_unit(v, unit).into(),
                Unit::Em => Em::new(v).into(),
                Unit::Fr => Fraction::new(v).into(),
                Unit::Percent => Ratio::new(v / 100.0).into(),
            },
            LitKind::Str(ref v) => Value::Str(v.clone()),
        })
    }
}

impl Eval for Ident {
    type Output = Value;

    fn eval(&self, _: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        match scp.get(self) {
            Some(slot) => Ok(slot.read().clone()),
            None => bail!(self.span(), "unknown variable"),
        }
    }
}

impl Eval for CodeBlock {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        scp.enter();

        let mut output = Value::None;
        for expr in self.exprs() {
            output = join_result(output, expr.eval(ctx, scp), expr.span())?;
        }

        scp.exit();
        Ok(output)
    }
}

impl Eval for ContentBlock {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        scp.enter();
        let content = self.body().eval(ctx, scp)?;
        scp.exit();
        Ok(content)
    }
}

impl Eval for GroupExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        self.expr().eval(ctx, scp)
    }
}

impl Eval for ArrayExpr {
    type Output = Array;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let items = self.items();

        let mut vec = Vec::with_capacity(items.size_hint().0);
        for item in items {
            match item {
                ArrayItem::Pos(expr) => vec.push(expr.eval(ctx, scp)?),
                ArrayItem::Spread(expr) => match expr.eval(ctx, scp)? {
                    Value::None => {}
                    Value::Array(array) => vec.extend(array.into_iter()),
                    v => bail!(expr.span(), "cannot spread {} into array", v.type_name()),
                },
            }
        }

        Ok(Array::from_vec(vec))
    }
}

impl Eval for DictExpr {
    type Output = Dict;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let mut map = BTreeMap::new();

        for item in self.items() {
            match item {
                DictItem::Named(named) => {
                    map.insert(named.name().take(), named.expr().eval(ctx, scp)?);
                }
                DictItem::Spread(expr) => match expr.eval(ctx, scp)? {
                    Value::None => {}
                    Value::Dict(dict) => map.extend(dict.into_iter()),
                    v => bail!(
                        expr.span(),
                        "cannot spread {} into dictionary",
                        v.type_name()
                    ),
                },
            }
        }

        Ok(Dict::from_map(map))
    }
}

impl Eval for UnaryExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let value = self.expr().eval(ctx, scp)?;
        let result = match self.op() {
            UnOp::Pos => ops::pos(value),
            UnOp::Neg => ops::neg(value),
            UnOp::Not => ops::not(value),
        };
        Ok(result.at(self.span())?)
    }
}

impl Eval for BinaryExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        match self.op() {
            BinOp::Add => self.apply(ctx, scp, ops::add),
            BinOp::Sub => self.apply(ctx, scp, ops::sub),
            BinOp::Mul => self.apply(ctx, scp, ops::mul),
            BinOp::Div => self.apply(ctx, scp, ops::div),
            BinOp::And => self.apply(ctx, scp, ops::and),
            BinOp::Or => self.apply(ctx, scp, ops::or),
            BinOp::Eq => self.apply(ctx, scp, ops::eq),
            BinOp::Neq => self.apply(ctx, scp, ops::neq),
            BinOp::Lt => self.apply(ctx, scp, ops::lt),
            BinOp::Leq => self.apply(ctx, scp, ops::leq),
            BinOp::Gt => self.apply(ctx, scp, ops::gt),
            BinOp::Geq => self.apply(ctx, scp, ops::geq),
            BinOp::In => self.apply(ctx, scp, ops::in_),
            BinOp::NotIn => self.apply(ctx, scp, ops::not_in),
            BinOp::Assign => self.assign(ctx, scp, |_, b| Ok(b)),
            BinOp::AddAssign => self.assign(ctx, scp, ops::add),
            BinOp::SubAssign => self.assign(ctx, scp, ops::sub),
            BinOp::MulAssign => self.assign(ctx, scp, ops::mul),
            BinOp::DivAssign => self.assign(ctx, scp, ops::div),
        }
    }
}

impl BinaryExpr {
    /// Apply a basic binary operation.
    fn apply(
        &self,
        ctx: &mut Context,
        scp: &mut Scopes,
        op: fn(Value, Value) -> StrResult<Value>,
    ) -> EvalResult<Value> {
        let lhs = self.lhs().eval(ctx, scp)?;

        // Short-circuit boolean operations.
        if (self.op() == BinOp::And && lhs == Value::Bool(false))
            || (self.op() == BinOp::Or && lhs == Value::Bool(true))
        {
            return Ok(lhs);
        }

        let rhs = self.rhs().eval(ctx, scp)?;
        Ok(op(lhs, rhs).at(self.span())?)
    }

    /// Apply an assignment operation.
    fn assign(
        &self,
        ctx: &mut Context,
        scp: &mut Scopes,
        op: fn(Value, Value) -> StrResult<Value>,
    ) -> EvalResult<Value> {
        let rhs = self.rhs().eval(ctx, scp)?;
        let lhs = self.lhs();
        let mut location = lhs.access(ctx, scp)?;
        let lhs = std::mem::take(&mut *location);
        *location = op(lhs, rhs).at(self.span())?;
        Ok(Value::None)
    }
}

impl Eval for FieldAccess {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let object = self.object().eval(ctx, scp)?;
        Ok(match object {
            Value::Dict(dict) => dict.get(self.field().take()).at(self.span())?.clone(),

            v => bail!(
                self.object().span(),
                "cannot access field on {}",
                v.type_name()
            ),
        })
    }
}

impl Eval for FuncCall {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let callee = self.callee().eval(ctx, scp)?;
        let args = self.args().eval(ctx, scp)?;

        Ok(match callee {
            Value::Array(array) => array.get(args.into_index()?).at(self.span())?.clone(),
            Value::Dict(dict) => dict.get(args.into_key()?).at(self.span())?.clone(),
            Value::Func(func) => {
                let point = || Tracepoint::Call(func.name().map(ToString::to_string));
                func.call(ctx, args).trace(point, self.span())?
            }

            v => bail!(
                self.callee().span(),
                "expected callable or collection, found {}",
                v.type_name(),
            ),
        })
    }
}

impl Eval for MethodCall {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let span = self.span();
        let method = self.method();
        let point = || Tracepoint::Call(Some(method.to_string()));

        Ok(if methods::is_mutating(&method) {
            let args = self.args().eval(ctx, scp)?;
            let mut value = self.receiver().access(ctx, scp)?;
            methods::call_mut(ctx, &mut value, &method, args, span).trace(point, span)?;
            Value::None
        } else {
            let value = self.receiver().eval(ctx, scp)?;
            let args = self.args().eval(ctx, scp)?;
            methods::call(ctx, value, &method, args, span).trace(point, span)?
        })
    }
}

impl Eval for CallArgs {
    type Output = Args;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let mut items = Vec::new();

        for arg in self.items() {
            let span = arg.span();
            match arg {
                CallArg::Pos(expr) => {
                    items.push(Arg {
                        span,
                        name: None,
                        value: Spanned::new(expr.eval(ctx, scp)?, expr.span()),
                    });
                }
                CallArg::Named(named) => {
                    items.push(Arg {
                        span,
                        name: Some(named.name().take()),
                        value: Spanned::new(
                            named.expr().eval(ctx, scp)?,
                            named.expr().span(),
                        ),
                    });
                }
                CallArg::Spread(expr) => match expr.eval(ctx, scp)? {
                    Value::None => {}
                    Value::Array(array) => {
                        items.extend(array.into_iter().map(|value| Arg {
                            span,
                            name: None,
                            value: Spanned::new(value, span),
                        }));
                    }
                    Value::Dict(dict) => {
                        items.extend(dict.into_iter().map(|(key, value)| Arg {
                            span,
                            name: Some(key),
                            value: Spanned::new(value, span),
                        }));
                    }
                    Value::Args(args) => items.extend(args.items),
                    v => bail!(expr.span(), "cannot spread {}", v.type_name()),
                },
            }
        }

        Ok(Args { span: self.span(), items })
    }
}

impl Eval for ClosureExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        // The closure's name is defined by its let binding if there's one.
        let name = self.name().map(Ident::take);

        // Collect captured variables.
        let captured = {
            let mut visitor = CapturesVisitor::new(scp);
            visitor.visit(self.as_red());
            visitor.finish()
        };

        let mut params = Vec::new();
        let mut sink = None;

        // Collect parameters and an optional sink parameter.
        for param in self.params() {
            match param {
                ClosureParam::Pos(name) => {
                    params.push((name.take(), None));
                }
                ClosureParam::Named(named) => {
                    params
                        .push((named.name().take(), Some(named.expr().eval(ctx, scp)?)));
                }
                ClosureParam::Sink(name) => {
                    if sink.is_some() {
                        bail!(name.span(), "only one argument sink is allowed");
                    }
                    sink = Some(name.take());
                }
            }
        }

        // Define the actual function.
        Ok(Value::Func(Func::from_closure(Closure {
            name,
            captured,
            params,
            sink,
            body: self.body(),
        })))
    }
}

impl Eval for LetExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let value = match self.init() {
            Some(expr) => expr.eval(ctx, scp)?,
            None => Value::None,
        };
        scp.top.def_mut(self.binding().take(), value);
        Ok(Value::None)
    }
}

impl Eval for SetExpr {
    type Output = StyleMap;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let target = self.target();
        let target = target.eval(ctx, scp)?.cast::<Func>().at(target.span())?;
        let args = self.args().eval(ctx, scp)?;
        Ok(target.set(args)?)
    }
}

impl Eval for ShowExpr {
    type Output = StyleMap;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        // Evaluate the target function.
        let target = self.target();
        let target_span = target.span();
        let target = target.eval(ctx, scp)?.cast::<Func>().at(target_span)?;

        // Collect captured variables.
        let captured = {
            let mut visitor = CapturesVisitor::new(scp);
            visitor.visit(self.as_red());
            visitor.finish()
        };

        // Define the recipe function.
        let body = self.body();
        let body_span = body.span();
        let recipe = Func::from_closure(Closure {
            name: None,
            captured,
            params: vec![(self.binding().take(), None)],
            sink: None,
            body,
        });

        Ok(target.show(recipe, body_span).at(target_span)?)
    }
}

impl Eval for IfExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let condition = self.condition();
        if condition.eval(ctx, scp)?.cast::<bool>().at(condition.span())? {
            self.if_body().eval(ctx, scp)
        } else if let Some(else_body) = self.else_body() {
            else_body.eval(ctx, scp)
        } else {
            Ok(Value::None)
        }
    }
}

impl Eval for WhileExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let mut output = Value::None;

        let condition = self.condition();
        while condition.eval(ctx, scp)?.cast::<bool>().at(condition.span())? {
            let body = self.body();
            match join_result(output, body.eval(ctx, scp), body.span()) {
                Err(Control::Break(value, _)) => {
                    output = value;
                    break;
                }
                Err(Control::Continue(value, _)) => output = value,
                other => output = other?,
            }
        }

        Ok(output)
    }
}

impl Eval for ForExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        macro_rules! iter {
            (for ($($binding:ident => $value:ident),*) in $iter:expr) => {{
                let mut output = Value::None;
                scp.enter();

                #[allow(unused_parens)]
                for ($($value),*) in $iter {
                    $(scp.top.def_mut(&$binding, $value);)*

                    let body = self.body();
                    match join_result(output, body.eval(ctx, scp), body.span()) {
                        Err(Control::Break(value, _)) => {
                            output = value;
                            break;
                        }
                        Err(Control::Continue(value, _)) => output = value,
                        other => output = other?,
                    }
                }

                scp.exit();
                return Ok(output);
            }};
        }

        let iter = self.iter().eval(ctx, scp)?;
        let pattern = self.pattern();
        let key = pattern.key().map(Ident::take);
        let value = pattern.value().take();

        match (key, value, iter) {
            (None, v, Value::Str(string)) => {
                iter!(for (v => value) in string.graphemes(true));
            }
            (None, v, Value::Array(array)) => {
                iter!(for (v => value) in array.into_iter());
            }
            (Some(i), v, Value::Array(array)) => {
                iter!(for (i => idx, v => value) in array.into_iter().enumerate());
            }
            (None, v, Value::Dict(dict)) => {
                iter!(for (v => value) in dict.into_iter().map(|p| p.1));
            }
            (Some(k), v, Value::Dict(dict)) => {
                iter!(for (k => key, v => value) in dict.into_iter());
            }
            (None, v, Value::Args(args)) => {
                iter!(for (v => value) in args.items.into_iter()
                    .filter(|arg| arg.name.is_none())
                    .map(|arg| arg.value.v));
            }
            (Some(k), v, Value::Args(args)) => {
                iter!(for (k => key, v => value) in args.items.into_iter()
                    .map(|arg| (arg.name.map_or(Value::None, Value::Str), arg.value.v)));
            }
            (_, _, Value::Str(_)) => {
                bail!(pattern.span(), "mismatched pattern");
            }
            (_, _, iter) => {
                bail!(self.iter().span(), "cannot loop over {}", iter.type_name());
            }
        }
    }
}

impl Eval for ImportExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let span = self.path().span();
        let path = self.path().eval(ctx, scp)?.cast::<EcoString>().at(span)?;
        let module = import(ctx, &path, span)?;

        match self.imports() {
            Imports::Wildcard => {
                for (var, slot) in module.scope.iter() {
                    scp.top.def_mut(var, slot.read().clone());
                }
            }
            Imports::Items(idents) => {
                for ident in idents {
                    if let Some(slot) = module.scope.get(&ident) {
                        scp.top.def_mut(ident.take(), slot.read().clone());
                    } else {
                        bail!(ident.span(), "unresolved import");
                    }
                }
            }
        }

        Ok(Value::None)
    }
}

impl Eval for IncludeExpr {
    type Output = Content;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let span = self.path().span();
        let path = self.path().eval(ctx, scp)?.cast::<EcoString>().at(span)?;
        let module = import(ctx, &path, span)?;
        Ok(module.content.clone())
    }
}

/// Process an import of a module relative to the current location.
fn import(ctx: &mut Context, path: &str, span: Span) -> TypResult<Module> {
    // Load the source file.
    let full = ctx.complete_path(path);
    let id = ctx.sources.load(&full).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => error!(span, "file not found"),
        _ => error!(span, "failed to load source file ({})", err),
    })?;

    // Prevent cyclic importing.
    if ctx.route.contains(&id) {
        bail!(span, "cyclic import");
    }

    // Evaluate the file.
    let module = ctx.evaluate(id).trace(|| Tracepoint::Import, span)?;
    ctx.deps.extend(module.deps.iter().cloned());
    Ok(module)
}

impl Eval for BreakExpr {
    type Output = Value;

    fn eval(&self, _: &mut Context, _: &mut Scopes) -> EvalResult<Self::Output> {
        Err(Control::Break(Value::default(), self.span()))
    }
}

impl Eval for ContinueExpr {
    type Output = Value;

    fn eval(&self, _: &mut Context, _: &mut Scopes) -> EvalResult<Self::Output> {
        Err(Control::Continue(Value::default(), self.span()))
    }
}

impl Eval for ReturnExpr {
    type Output = Value;

    fn eval(&self, ctx: &mut Context, scp: &mut Scopes) -> EvalResult<Self::Output> {
        let value = self.body().map(|body| body.eval(ctx, scp)).transpose()?;
        let explicit = value.is_some();
        Err(Control::Return(
            value.unwrap_or_default(),
            explicit,
            self.span(),
        ))
    }
}

/// Access an expression mutably.
pub trait Access {
    /// Access the value.
    fn access<'a>(
        &self,
        ctx: &mut Context,
        scp: &'a mut Scopes,
    ) -> EvalResult<Location<'a>>;
}

impl Access for Expr {
    fn access<'a>(
        &self,
        ctx: &mut Context,
        scp: &'a mut Scopes,
    ) -> EvalResult<Location<'a>> {
        match self {
            Expr::Ident(ident) => ident.access(ctx, scp),
            Expr::FuncCall(call) => call.access(ctx, scp),
            _ => bail!(self.span(), "cannot mutate a temporary value"),
        }
    }
}

impl Access for Ident {
    fn access<'a>(
        &self,
        _: &mut Context,
        scp: &'a mut Scopes,
    ) -> EvalResult<Location<'a>> {
        match scp.get(self) {
            Some(slot) => match slot.try_write() {
                Some(guard) => Ok(RwLockWriteGuard::map(guard, |v| v)),
                None => bail!(self.span(), "cannot mutate a constant"),
            },
            None => bail!(self.span(), "unknown variable"),
        }
    }
}

impl Access for FuncCall {
    fn access<'a>(
        &self,
        ctx: &mut Context,
        scp: &'a mut Scopes,
    ) -> EvalResult<Location<'a>> {
        let args = self.args().eval(ctx, scp)?;
        let guard = self.callee().access(ctx, scp)?;
        try_map(guard, |value| {
            Ok(match value {
                Value::Array(array) => {
                    array.get_mut(args.into_index()?).at(self.span())?
                }
                Value::Dict(dict) => dict.get_mut(args.into_key()?),
                v => bail!(
                    self.callee().span(),
                    "expected collection, found {}",
                    v.type_name(),
                ),
            })
        })
    }
}

/// A mutable location.
type Location<'a> = MappedRwLockWriteGuard<'a, Value>;

/// Map a reader-writer lock with a function.
fn try_map<F>(location: Location, f: F) -> EvalResult<Location>
where
    F: FnOnce(&mut Value) -> EvalResult<&mut Value>,
{
    let mut error = None;
    MappedRwLockWriteGuard::try_map(location, |value| match f(value) {
        Ok(value) => Some(value),
        Err(err) => {
            error = Some(err);
            None
        }
    })
    .map_err(|_| error.unwrap())
}
