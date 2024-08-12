//! Layout flows.
//!
//! A *flow* is a collection of block-level layoutable elements.
//! This is analogous to a paragraph, which is a collection of
//! inline-level layoutable elements.

#![allow(unused_imports)]

use std::num::NonZeroUsize;
use std::ptr;

use comemo::{Track, Tracked, TrackedMut};
use typst_utils::NonZeroExt;

use crate::diag::{bail, SourceResult};
use crate::engine::{Engine, Route, Sink, Traced};
use crate::foundations::{Content, NativeElement, Packed, Resolve, Smart, StyleChain};
use crate::introspection::{
    Counter, CounterDisplayElem, CounterKey, Introspector, Locator, LocatorLink,
    ManualPageCounter, SplitLocator, Tag, TagElem,
};
use crate::layout::{
    Abs, AlignElem, Alignment, Axes, Binding, BlockElem, ColbreakElem, Dir,
    FixedAlignment, FlushElem, Fr, Fragment, Frame, FrameItem, HAlignment, Length,
    OuterVAlignment, Page, PageElem, PagebreakElem, Paper, Parity, PlaceElem, Point,
    Regions, Rel, Sides, Size, Spacing, VAlignment, VElem,
};
use crate::model::{Document, FootnoteElem, FootnoteEntry, Numbering, ParElem};
use crate::realize::{realize_doc, realize_flow, Arenas};
use crate::text::TextElem;
use crate::utils::Numeric;
use crate::World;

impl Content {
    /// Layout the content into a document.
    ///
    /// This first realizes the content into a
    /// [`DocumentElem`][crate::model::DocumentElem], which is then laid out. In
    /// contrast to [`layout`](Self::layout()), this does not take regions since
    /// the regions are defined by the page configuration in the content and
    /// style chain.
    pub fn layout_document(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Document> {
        layout_document_impl(
            self,
            engine.world,
            engine.introspector,
            engine.traced,
            TrackedMut::reborrow_mut(&mut engine.sink),
            engine.route.track(),
            styles,
        )
    }

    /// Layout the content into the given regions.
    #[typst_macros::time(span = self.span())]
    pub fn layout(
        &self,
        engine: &mut Engine,
        locator: Locator,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        layout_fragment_impl(
            self,
            engine.world,
            engine.introspector,
            engine.traced,
            TrackedMut::reborrow_mut(&mut engine.sink),
            engine.route.track(),
            locator.track(),
            styles,
            regions,
        )
    }
}

/// The raw memoized implementation of `Content::layout_document`.
#[comemo::memoize]
fn layout_document_impl(
    content: &Content,
    world: Tracked<dyn World + '_>,
    introspector: Tracked<Introspector>,
    traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    route: Tracked<Route>,
    styles: StyleChain,
) -> SourceResult<Document> {
    let mut engine = Engine {
        world,
        introspector,
        traced,
        sink,
        route: Route::extend(route).unnested(),
    };

    let mut locator = Locator::root().split();
    let arenas = Arenas::default();
    let (children, info) =
        realize_doc(&mut engine, &mut locator, &arenas, content, styles)?;

    let page_info = PageInfo::new(styles);
    let mut layouter = FlowLayouter::new(
        &children,
        &mut engine,
        locator,
        page_info.regions(),
        Some(page_info),
    );
    layouter.layout()?;

    Ok(Document {
        pages: layouter.finished2,
        info,
        introspector: Introspector::default(),
    })
}

/// The raw memoized implementation of `Content::layout`.
#[allow(clippy::too_many_arguments)]
#[comemo::memoize]
fn layout_fragment_impl(
    content: &Content,
    world: Tracked<dyn World + '_>,
    introspector: Tracked<Introspector>,
    traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    route: Tracked<Route>,
    locator: Tracked<Locator>,
    styles: StyleChain,
    regions: Regions,
) -> SourceResult<Fragment> {
    let mut engine = Engine {
        world,
        introspector,
        traced,
        sink,
        route: Route::extend(route),
    };

    if !engine.route.within(Route::MAX_LAYOUT_DEPTH) {
        bail!(
            content.span(), "maximum layout depth exceeded";
            hint: "try to reduce the amount of nesting in your layout",
        );
    }

    let link = LocatorLink::new(locator);
    let mut locator = Locator::link(&link).split();

    let arenas = Arenas::default();
    let children = realize_flow(&mut engine, &mut locator, &arenas, content, styles)?;

    let mut layouter = FlowLayouter::new(&children, &mut engine, locator, regions, None);
    layouter.layout()?;

    Ok(Fragment::frames(layouter.finished))
}

struct FlowLayouter<'a, 'e> {
    children: &'a [(&'a Content, StyleChain<'a>)],
    engine: &'a mut Engine<'e>,
    locator: SplitLocator<'a>,
    finished: Vec<Frame>,
    finished2: Vec<Page>,
    pending_tags: Vec<&'a Tag>,
    page_info: Option<PageInfo<'a>>,
    regions: Regions<'a>,
    output: Frame,
    cursor: Point,
    last_was_par: bool,
}

impl<'a, 'e> FlowLayouter<'a, 'e> {
    fn new(
        children: &'a [(&'a Content, StyleChain<'a>)],
        engine: &'a mut Engine<'e>,
        locator: SplitLocator<'a>,
        regions: Regions<'a>,
        page_info: Option<PageInfo<'a>>,
    ) -> Self {
        Self {
            children,
            engine,
            locator,
            finished: vec![],
            finished2: vec![],
            pending_tags: vec![],
            page_info,
            regions,
            output: Frame::default(),
            cursor: Point::zero(),
            last_was_par: false,
        }
    }

    fn layout(&mut self) -> SourceResult<()> {
        for &(child, styles) in self.children {
            if let Some(elem) = child.to_packed::<TagElem>() {
                self.layout_tag(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<VElem>() {
                self.layout_v(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<ParElem>() {
                self.layout_par(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<BlockElem>() {
                self.layout_block(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<PlaceElem>() {
                self.layout_place(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<FlushElem>() {
                self.layout_flush(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<ColbreakElem>() {
                self.layout_colbreak(elem, styles)?;
            } else if let Some(elem) = child.to_packed::<PagebreakElem>() {
                self.layout_pagebreak(elem, styles)?;
            } else {
                bail!(child.span(), "{} is not allowed here", child.func().name());
            }
        }

        self.drain_tags();
        self.finish_region()?;

        Ok(())
    }

    /// Layout a tag. Tags are used for introspection and always apply to the
    /// content that follows them. We want the tag position to match up with
    /// the first visible piece of content that follows. To do that we store
    /// them in a list of pending tags, which we drain whenever we layout
    /// visible content.
    fn layout_tag(
        &mut self,
        elem: &'a Packed<TagElem>,
        _: StyleChain<'a>,
    ) -> SourceResult<()> {
        self.pending_tags.push(&elem.tag);
        Ok(())
    }

    /// This is called before we layout visible content. See `layout_tag`
    /// for more details.
    fn drain_tags(&mut self) {
        for tag in self.pending_tags.drain(..) {
            self.output.push(self.cursor, FrameItem::Tag(tag.clone()));
        }
    }

    /// Layout vertical spacing.
    fn layout_v(
        &mut self,
        elem: &'a Packed<VElem>,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let amount = match elem.amount {
            Spacing::Rel(rel) => rel.resolve(styles).relative_to(self.regions.base().y),
            Spacing::Fr(_) => Abs::zero(),
        };

        let weak = elem.weakness(styles) > 0;
        let empty = self.is_currently_empty();

        if !weak {
            self.output.size_mut().y += amount;
        }

        if !weak || !empty {
            self.cursor.y += amount;
        }

        Ok(())
    }

    /// Layout a paragraph.
    fn layout_par(
        &mut self,
        par: &'a Packed<ParElem>,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let locator = self.locator.next(&par.span());
        let consecutive = self.last_was_par;
        let lines = par
            .layout(
                self.engine,
                locator,
                styles,
                consecutive,
                self.regions.base(),
                self.regions.expand.x,
            )?
            .into_frames();

        let leading = ParElem::leading_in(styles);
        let align = AlignElem::alignment_in(styles).resolve(styles);
        let costs = TextElem::costs_in(styles);
        let prevent_orphans = costs.orphan().get() > 0.0;
        let prevent_widows = costs.widow().get() > 0.0;

        // Store the heights of lines at the edges because we'll potentially
        // need these later when `lines` is already moved.
        let len = lines.len();
        let h = |i| lines.get(i).map(Frame::height).unwrap_or_default();
        let front1 = h(0);
        let front2 = h(1);
        let back2 = h(len.saturating_sub(2));
        let back1 = h(len.saturating_sub(1));

        for (i, line) in lines.into_iter().enumerate() {
            if i > 0 {
                self.cursor.y += leading;
            }

            // To prevent widows and orphans, we require enough space for
            // - all lines if it's just three
            // - the first two lines if we're at the first line
            // - the last two lines if we're at the second to last line
            let needed = if prevent_orphans && prevent_widows && len == 3 && i == 0 {
                front1 + leading + front2 + leading + back1
            } else if prevent_orphans && i == 0 {
                front1 + leading + front2
            } else if prevent_widows && i >= 2 && i + 2 == len {
                back2 + leading + back1
            } else {
                line.height()
            };

            if self.may_break() && !self.regions.size.y.fits(self.cursor.y + needed) {
                self.finish_region()?;
            }

            self.push_frame(line, align.x);
        }

        self.last_was_par = true;
        Ok(())
    }

    /// Layout a block.
    ///
    /// This includes `#block`s explicitly constructed by the user,
    /// but also many more things that show themselves as blocks internally
    /// (e.g. `heading`, `grid`, `rotate`, and many more).
    fn layout_block(
        &mut self,
        block: &'a Packed<BlockElem>,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let locator = self.locator.next(&block.span());
        let fragment = block.layout(self.engine, locator, styles, self.regions)?;

        let align = AlignElem::alignment_in(styles).resolve(styles);

        for (i, mut frame) in fragment.into_iter().enumerate() {
            if i > 0 {
                self.finish_region()?;
            }

            frame.post_process(styles);
            self.push_frame(frame, align.x);
        }

        self.last_was_par = false;
        Ok(())
    }

    /// Add a frame to the output and adjust the sizes.
    fn push_frame(&mut self, frame: Frame, x_align: FixedAlignment) {
        self.drain_tags();
        let size = frame.size();
        let mut x = x_align.position(self.regions.size.x - frame.width());
        if !x.is_finite() || !self.regions.expand.x {
            x = Abs::zero();
        }
        let delta = Point::with_x(x);
        self.output.push_frame(self.cursor + delta, frame);
        self.cursor.y += size.y;
        self.output.size_mut().x.set_max(size.x);
        self.output.size_mut().y = self.cursor.y;
    }

    /// Layout an absolutely placed element.
    fn layout_place(
        &mut self,
        placed: &'a Packed<PlaceElem>,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let locator = self.locator.next(&placed.span());
        let mut frame = placed
            .layout(self.engine, locator, styles, self.regions.base())?
            .into_frame();
        frame.post_process(styles);

        let delta = Axes::new(placed.dx(styles), placed.dy(styles))
            .resolve(styles)
            .relative_to(self.regions.size)
            .to_point();

        let alignment = placed.alignment(styles).unwrap_or_default();
        let x_align = alignment.x().unwrap_or_default().resolve(styles);
        let y_align = alignment.y().resolve(styles);

        let mut x = x_align.position(self.regions.size.x - frame.size().x);
        if !x.is_finite() || !self.regions.expand.x {
            x = Abs::zero();
        }
        let y = match y_align {
            Some(align) => align.position(self.regions.size.y - frame.size().y),
            None => self.cursor.y,
        };

        let pos = Point::new(x, y) + delta;
        self.output.push_frame(pos, frame);

        Ok(())
    }

    /// Layout a `#place.flush` element which acts as a float barrier: All
    /// floating placed elements must be placed before any following content.
    fn layout_flush(
        &mut self,
        _elem: &'a Packed<FlushElem>,
        _styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        // TODO
        Ok(())
    }

    /// Layout a column break, terminating the current column.
    fn layout_colbreak(
        &mut self,
        colbreak: &'a Packed<ColbreakElem>,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        let skippable = colbreak.weak(styles) && self.is_currently_empty();
        if !skippable && self.may_break() {
            self.finish_region()?;
        }
        Ok(())
    }

    /// Layout a column break, terminating the current page.
    fn layout_pagebreak(
        &mut self,
        pagebreak: &'a Packed<PagebreakElem>,
        styles: StyleChain<'a>,
    ) -> SourceResult<()> {
        if self.page_info.is_none() {
            bail!(pagebreak.span(), "pagebreaks are not allowed inside of containers");
        }

        let skippable = pagebreak.weak(styles) && self.is_currently_empty();
        if !skippable {
            self.finish_region()?;
        }

        if let Some(parity) = pagebreak.to(styles) {
            let nr = self.physical_counter();
            if !parity.matches(nr) {
                self.finish_region()?;
            }
        }

        let info = PageInfo::new(styles);
        self.regions = info.regions();
        self.page_info = Some(info);

        Ok(())
    }

    fn finish_region(&mut self) -> SourceResult<()> {
        let mut frame = std::mem::take(&mut self.output);
        frame.set_size(self.regions.expand.select(self.regions.size, frame.size()));

        if let Some(info) = &self.page_info {
            let nr = self.physical_counter();
            self.finished2.push(info.finalize(
                self.engine,
                &mut self.locator,
                nr,
                frame,
            )?);
        } else {
            self.finished.push(frame);
        }

        self.regions.next();
        self.cursor.y = Abs::zero();
        Ok(())
    }

    fn is_currently_empty(&self) -> bool {
        self.output.is_empty() // TODO: More robust
    }

    fn may_break(&self) -> bool {
        !self.is_currently_empty()
            && (!self.regions.backlog.is_empty() || self.regions.last.is_some())
    }

    fn physical_counter(&self) -> NonZeroUsize {
        NonZeroUsize::ONE.saturating_add(self.finished2.len())
    }
}

struct PageInfo<'a> {
    styles: StyleChain<'a>,
    size: Size,
    margin: Sides<Abs>,
    two_sided: bool,
}

impl<'a> PageInfo<'a> {
    fn new(styles: StyleChain<'a>) -> Self {
        // When one of the lengths is infinite the page fits its content along
        // that axis.
        let width = PageElem::width_in(styles).unwrap_or(Abs::inf());
        let height = PageElem::height_in(styles).unwrap_or(Abs::inf());
        let mut size = Size::new(width, height);
        if PageElem::flipped_in(styles) {
            std::mem::swap(&mut size.x, &mut size.y);
        }

        let mut min = width.min(height);
        if !min.is_finite() {
            min = Paper::A4.width();
        }

        // Determine the margins.
        let default = Rel::<Length>::from((2.5 / 21.0) * min);
        let margin = PageElem::margin_in(styles);
        let two_sided = margin.two_sided.unwrap_or(false);
        let margin = margin
            .sides
            .map(|side| side.and_then(Smart::custom).unwrap_or(default))
            .resolve(styles)
            .relative_to(size);

        Self { styles, size, margin, two_sided }
    }

    fn regions(&self) -> Regions<'static> {
        let area = self.size - self.margin.sum_by_axis();
        Regions::repeat(area, area.map(Abs::is_finite))
    }

    fn finalize(
        &self,
        engine: &mut Engine,
        locator: &mut SplitLocator,
        physical_number: NonZeroUsize,
        mut frame: Frame,
    ) -> SourceResult<Page> {
        let styles = self.styles;
        let fill = PageElem::fill_in(styles);
        let foreground = PageElem::foreground_in(styles);
        let background = PageElem::background_in(styles);
        let header_ascent = PageElem::header_ascent_in(styles);
        let footer_descent = PageElem::footer_descent_in(styles);
        let numbering = PageElem::numbering_in(styles);
        let number_align = PageElem::number_align_in(styles);
        let binding = PageElem::binding_in(styles).unwrap_or_else(|| {
            match TextElem::dir_in(styles) {
                Dir::LTR => Binding::Left,
                _ => Binding::Right,
            }
        });

        // Construct the numbering (for header or footer).
        let numbering_marginal = numbering.as_ref().map(|numbering| {
            let both = match numbering {
                Numbering::Pattern(pattern) => pattern.pieces() >= 2,
                Numbering::Func(_) => true,
            };

            let mut counter = CounterDisplayElem::new(
                Counter::new(CounterKey::Page),
                Smart::Custom(numbering.clone()),
                both,
            )
            .pack();

            // We interpret the Y alignment as selecting header or footer
            // and then ignore it for aligning the actual number.
            if let Some(x) = number_align.x() {
                counter = counter.aligned(x.into());
            }

            counter
        });

        let header = PageElem::header_in(styles);
        let footer = PageElem::footer_in(styles);
        let (header, footer) = if matches!(number_align.y(), Some(OuterVAlignment::Top)) {
            (
                header.as_ref().unwrap_or(&numbering_marginal),
                footer.as_ref().unwrap_or(&None),
            )
        } else {
            (
                header.as_ref().unwrap_or(&None),
                footer.as_ref().unwrap_or(&numbering_marginal),
            )
        };

        // The padded width of the page's content without margins.
        let pw = frame.width();

        // If two sided, left becomes inside and right becomes outside.
        // Thus, for left-bound pages, we want to swap on even pages and
        // for right-bound pages, we want to swap on odd pages.
        let mut margin = self.margin;
        if self.two_sided && binding.swap(physical_number) {
            std::mem::swap(&mut margin.left, &mut margin.right);
        }

        frame.set_size(frame.size() + margin.sum_by_axis());
        frame.translate(Point::new(margin.left, margin.top));

        // The page size with margins.
        let size = frame.size();

        // Realize overlays.
        for marginal in [header, footer, background, foreground] {
            let Some(content) = marginal.as_ref() else { continue };

            let (pos, area, align);
            if ptr::eq(marginal, header) {
                let ascent = header_ascent.relative_to(margin.top);
                pos = Point::with_x(margin.left);
                area = Size::new(pw, margin.top - ascent);
                align = Alignment::BOTTOM;
            } else if ptr::eq(marginal, footer) {
                let descent = footer_descent.relative_to(margin.bottom);
                pos = Point::new(margin.left, size.y - margin.bottom + descent);
                area = Size::new(pw, margin.bottom - descent);
                align = Alignment::TOP;
            } else {
                pos = Point::zero();
                area = size;
                align = HAlignment::Center + VAlignment::Horizon;
            };

            let pod = Regions::one(area, Axes::splat(true));
            let sub = content
                .clone()
                .styled(AlignElem::set_alignment(align))
                .layout(engine, locator.next(&content.span()), styles, pod)?
                .into_frame();

            if ptr::eq(marginal, header) || ptr::eq(marginal, background) {
                frame.prepend_frame(pos, sub);
            } else {
                frame.push_frame(pos, sub);
            }
        }

        Ok(Page {
            frame,
            fill: fill.clone(),
            numbering: numbering.clone(),
            number: 1, // TODO
        })
    }
}
