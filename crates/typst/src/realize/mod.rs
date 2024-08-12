//! Realization of content.
//!
//! *Realization* is the process of applying show rules to produce
//! something that can be laid out directly.
//!
//! Currently, there are issues with the realization process, and
//! it is subject to changes in the future.

mod arenas;
mod behaviour;
mod pipeline;
mod process;

use once_cell::unsync::Lazy;

pub use self::arenas::Arenas;
pub use self::behaviour::{Behave, BehavedBuilder, Behaviour, StyleVec};
pub use self::process::process;

use std::mem;

use self::pipeline::{Pipe, Pipeline, Stage};
use crate::diag::{bail, SourceResult};
use crate::engine::{Engine, Route};
use crate::foundations::{
    Content, NativeElement, Packed, SequenceElem, Smart, StyleChain, StyledElem,
};
use crate::introspection::{SplitLocator, TagElem};
use crate::layout::{
    AlignElem, BlockElem, BoxElem, HElem, InlineElem, PageElem, PagebreakElem, VElem,
};
use crate::math::{EquationElem, LayoutMath};
use crate::model::{
    CiteElem, CiteGroup, DocumentElem, DocumentInfo, EnumElem, ListElem, ListLike,
    ParElem, ParbreakElem, TermsElem,
};
use crate::syntax::Span;
use crate::text::{LinebreakElem, SmartQuoteElem, SpaceElem, TextElem};

/// Realize into a `DocumentElem`, an element that is capable of root-level
/// layout.
#[typst_macros::time(name = "realize doc")]
pub fn realize_doc<'a>(
    engine: &mut Engine<'a>,
    locator: &mut SplitLocator<'a>,
    arenas: &'a Arenas<'a>,
    content: &'a Content,
    styles: StyleChain<'a>,
) -> SourceResult<(Vec<(&'a Content, StyleChain<'a>)>, DocumentInfo)> {
    let mut realizer = Realizer { engine, locator, arenas };
    let mut pipeline = ContentPipeline::<'a>::default();
    let (_, styler, ..) = &mut pipeline;
    styler.info = Some(DocumentInfo::default());

    pipeline.ingest(&mut realizer, content, styles)?;
    pipeline.flush(&mut realizer)?;

    let (_, styler, .., sink) = pipeline;
    Ok((sink.0.finish(), styler.info.unwrap()))
}

/// Realize into a `FlowElem`, an element that is capable of block-level layout.
#[typst_macros::time(name = "realize flow")]
pub fn realize_flow<'a>(
    engine: &mut Engine<'a>,
    locator: &mut SplitLocator<'a>,
    arenas: &'a Arenas<'a>,
    content: &'a Content,
    styles: StyleChain<'a>,
) -> SourceResult<Vec<(&'a Content, StyleChain<'a>)>> {
    let mut realizer = Realizer { engine, locator, arenas };
    let mut pipeline = ContentPipeline::<'a>::default();

    pipeline.ingest(&mut realizer, content, styles)?;
    pipeline.flush(&mut realizer)?;

    let (.., sink) = pipeline;
    Ok(sink.0.finish())
}

/// A sequence of steps that are taken to realize content.
#[rustfmt::skip]
type ContentPipeline<'a> = (
    RuleMatcher,
    Styler,
    Flattener,
    MathWrapper,
    CiteGroupBuilder<'a>,
    ListBuilder<'a, ListElem>,
    ListBuilder<'a, EnumElem>,
    ListBuilder<'a, TermsElem>,
    ParBuilder<'a>,
    BlockSpacer,
    Sink<'a>,
);

/// Builds a document or a flow element from content.
struct Realizer<'a, 'v> {
    /// The engine.
    engine: &'v mut Engine<'a>,
    /// Assigns unique locations to elements.
    locator: &'v mut SplitLocator<'a>,
    /// Scratch arenas for building.
    arenas: &'a Arenas<'a>,
}

/// Handles preparation and show rules.
#[derive(Default)]
struct RuleMatcher;

impl<'a: 'v, 'v> Stage<'a, 'v> for RuleMatcher {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let realizer = p.realizer();
        let Some(realized) =
            process(realizer.engine, &mut realizer.locator, content, styles)?
        else {
            return p.forward(content, styles);
        };

        realizer.engine.route.increase();

        if !realizer.engine.route.within(Route::MAX_SHOW_RULE_DEPTH) {
            bail!(
                content.span(), "maximum show rule depth exceeded";
                hint: "check whether the show rule matches its own output"
            );
        }

        let stored = realizer.arenas.store(realized);
        let result = p.ingest(stored, styles);

        p.realizer().engine.route.decrease();

        result
    }
}

/// Handles styles.
#[derive(Default)]
struct Styler {
    info: Option<DocumentInfo>,
}

impl<'a: 'v, 'v> Stage<'a, 'v> for Styler {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let arenas = p.realizer().arenas;
        let Some(styled) = content.to_packed::<StyledElem>() else {
            return p.forward(content, styles);
        };

        let local = &styled.styles;
        let styles = arenas.store(styles).chain(local);

        if let Some(span) = local.interruption::<DocumentElem>() {
            let Some(info) = &mut p.info else {
                bail!(span, "document set rules are not allowed inside of containers");
            };
            info.populate(local);
        }

        if let Some(span) = local.interruption::<PageElem>() {
            let parbreak = PagebreakElem::new().with_weak(true).pack().spanned(span);
            p.ingest(arenas.store(parbreak), styles)?;
        } else if let Some(span) = local
            .interruption::<ParElem>()
            .or_else(|| local.interruption::<AlignElem>())
        {
            let parbreak = ParbreakElem::new().pack().spanned(span);
            p.ingest(arenas.store(parbreak), styles)?;
        }

        p.ingest(&styled.child, styles)
    }
}

/// Flattens sequences.
#[derive(Default)]
struct Flattener;

impl<'a: 'v, 'v> Stage<'a, 'v> for Flattener {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let Some(sequence) = content.to_packed::<SequenceElem>() else {
            return p.forward(content, styles);
        };

        for elem in &sequence.children {
            p.ingest(elem, styles)?;
        }

        Ok(())
    }
}

/// Wraps free-floating math content in an equation if it's not already in one.
#[derive(Default)]
struct MathWrapper;

impl<'a: 'v, 'v> Stage<'a, 'v> for MathWrapper {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        if !content.can::<dyn LayoutMath>() || content.is::<EquationElem>() {
            return p.forward(content, styles);
        }

        let eq = EquationElem::new(content.clone()).pack().spanned(content.span());
        let stored = p.realizer().arenas.store(eq);
        p.ingest(stored, styles)
    }
}

/// Builds a [citation group](CiteGroup) from citations.
#[derive(Default)]
struct CiteGroupBuilder<'a> {
    /// The citations.
    items: Vec<Packed<CiteElem>>,
    /// The styles.
    styles: StyleChain<'a>,
    /// Trailing content for which it is unclear whether it is part of the list.
    staged: Vec<(&'a Content, StyleChain<'a>)>,
}

impl<'a: 'v, 'v> Stage<'a, 'v> for CiteGroupBuilder<'a> {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let Some(citation) = content.to_packed::<CiteElem>() else {
            // If it's not a citation, but we have some citations, either
            // - we're unsure whether the group ends here: stage it
            // - we know it ends: flush it
            if !p.items.is_empty() {
                if content.is::<SpaceElem>() || content.is::<TagElem>() {
                    p.staged.push((content, styles));
                    return Ok(());
                }
                Self::flush(p)?;
            }
            return p.forward(content, styles);
        };

        // If it's a citation, add it to the group.
        if p.items.is_empty() {
            p.styles = styles;
        }
        p.staged.retain(|(elem, _)| !elem.is::<SpaceElem>());
        p.items.push(citation.clone());

        Ok(())
    }

    fn flush(p: &mut impl Pipe<'a, 'v, Self>) -> SourceResult<()> {
        if p.items.is_empty() {
            return Ok(());
        }

        let Self { styles, items, staged } = mem::take(&mut **p);

        let span = items.first().map(|elem| elem.span()).unwrap_or(Span::detached());
        let group = CiteGroup::new(items).pack().spanned(span);
        let stored = p.realizer().arenas.store(group);
        p.ingest(stored, styles)?;

        for (content, styles) in staged {
            p.ingest(content, styles)?;
        }

        Ok(())
    }
}

/// Builds a list (either [`ListElem`], [`EnumElem`], or [`TermsElem`])
/// from list or enum items, spaces, and paragraph breaks.
struct ListBuilder<'a, T: ListLike> {
    /// The list items collected so far.
    items: Vec<Packed<T::Item>>,
    /// Trailing content for which it is unclear whether it is part of the list.
    styles: StyleChain<'a>,
    /// Trailing content for which it is unclear whether it is part of the list.
    staged: Vec<(&'a Content, StyleChain<'a>)>,
    /// Whether the list contains no paragraph breaks.
    tight: bool,
}

impl<'a: 'v, 'v, T: ListLike> Stage<'a, 'v> for ListBuilder<'a, T> {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let Some(item) = content.to_packed::<T::Item>() else {
            if !p.items.is_empty() {
                if content.is::<SpaceElem>() || content.is::<ParbreakElem>() {
                    p.staged.push((content, styles));
                    return Ok(());
                }
                Self::flush(p)?;
            }
            return p.forward(content, styles);
        };

        // If the the new list item has different styles, flush the previous
        // list before starting a new one.
        if !p.items.is_empty() && p.styles != styles {
            Self::flush(p)?;
        }

        // If it's a list item, add it to the group.
        let has_break = p.staged.drain(..).all(|(t, _)| !t.is::<ParbreakElem>());
        p.items.push(item.clone());
        p.tight &= has_break;
        p.styles = styles;

        Ok(())
    }

    fn flush(p: &mut impl Pipe<'a, 'v, Self>) -> SourceResult<()> {
        if p.items.is_empty() {
            return Ok(());
        }

        let Self { items, styles, staged, tight } = mem::take(&mut **p);

        let span = items.first().map(|elem| elem.span()).unwrap_or(Span::detached());
        let list = T::create(items, tight).pack().spanned(span);
        let stored = p.realizer().arenas.store(list);
        p.ingest(stored, styles)?;

        for (content, styles) in staged {
            p.ingest(content, styles)?;
        }

        Ok(())
    }
}

impl<T: ListLike> Default for ListBuilder<'_, T> {
    fn default() -> Self {
        Self {
            items: vec![],
            styles: StyleChain::default(),
            staged: vec![],
            tight: true,
        }
    }
}

/// Builds a [paragraph][ParElem] from paragraph content.
#[derive(Default)]
struct ParBuilder<'a>(BehavedBuilder<'a>);

impl<'a: 'v, 'v> Stage<'a, 'v> for ParBuilder<'a> {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        if content.is::<TagElem>() {
            if !p.0.is_empty() {
                p.0.push(content, styles);
                return Ok(());
            }
        } else if content.is::<SpaceElem>()
            || content.is::<TextElem>()
            || content.is::<HElem>()
            || content.is::<LinebreakElem>()
            || content.is::<SmartQuoteElem>()
            || content.is::<InlineElem>()
            || content.is::<BoxElem>()
        {
            p.0.push(content, styles);
            return Ok(());
        }

        if !p.0.is_empty() {
            Self::flush(p)?;
        }

        p.forward(content, styles)
    }

    fn flush(p: &mut impl Pipe<'a, 'v, Self>) -> SourceResult<()> {
        if p.0.is_empty() {
            return Ok(());
        }

        let state = mem::take(&mut **p);
        let buf = state.0.finish();
        let span = buf.first().map(|(c, _)| c.span()).unwrap_or(Span::detached());
        let (children, trunk) = StyleVec::create(buf);
        let par = ParElem::new(children).pack().spanned(span);
        let stored = p.realizer().arenas.store(par);
        p.ingest(stored, trunk)?;

        Ok(())
    }
}

/// Adds block spacing to blocks and paragraphs and filters attached spacing.
#[derive(Default)]
struct BlockSpacer {
    last_was_par: bool,
}

impl<'a: 'v, 'v> Stage<'a, 'v> for BlockSpacer {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let arenas = p.realizer().arenas;
        let last_was_par = p.last_was_par;
        p.last_was_par = false;

        let par_spacing = Lazy::new(|| {
            let amount = ParElem::spacing_in(styles).into();
            arenas.store(VElem::par_spacing(amount).pack())
        });

        if let Some(elem) = content.to_packed::<VElem>() {
            if !elem.attach(styles) || last_was_par {
                p.forward(content, styles)?;
            }
            return Ok(());
        }

        if let Some(elem) = content.to_packed::<BlockElem>() {
            let above = match elem.above(styles) {
                Smart::Auto => *par_spacing,
                Smart::Custom(above) => arenas.store(VElem::block_spacing(above).pack()),
            };

            let below = match elem.below(styles) {
                Smart::Auto => *par_spacing,
                Smart::Custom(below) => arenas.store(VElem::block_spacing(below).pack()),
            };

            p.forward(above, styles)?;
            p.forward(content, styles)?;
            p.forward(below, styles)?;
            return Ok(());
        }

        if content.is::<ParElem>() {
            p.forward(*par_spacing, styles)?;
            p.forward(content, styles)?;
            p.forward(*par_spacing, styles)?;
            p.last_was_par = true;
            return Ok(());
        }

        p.forward(content, styles)
    }
}

/// Builds a document or a flow element from incoming content.
#[derive(Default)]
struct Sink<'a>(BehavedBuilder<'a>);

impl<'a: 'v, 'v> Stage<'a, 'v> for Sink<'a> {
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        if content.is::<ParbreakElem>() {
            return Ok(());
        }

        p.0.push(content, styles);
        Ok(())
    }
}
