//! Composable layouts.

mod abs;
mod align;
mod angle;
mod axes;
mod columns;
mod container;
mod corners;
mod dir;
mod em;
mod flow;
mod fr;
mod fragment;
mod frame;
mod grid;
mod hide;
mod inline;
#[path = "layout.rs"]
mod layout_;
mod length;
#[path = "measure.rs"]
mod measure_;
mod pad;
mod page;
mod place;
mod point;
mod ratio;
mod regions;
mod rel;
mod repeat;
mod sides;
mod size;
mod spacing;
mod stack;
mod transform;

pub use self::abs::*;
pub use self::align::*;
pub use self::angle::*;
pub use self::axes::*;
pub use self::columns::*;
pub use self::container::*;
pub use self::corners::*;
pub use self::dir::*;
pub use self::em::*;
pub use self::flow::*;
pub use self::fr::*;
pub use self::fragment::*;
pub use self::frame::*;
pub use self::grid::*;
pub use self::hide::*;
pub use self::layout_::*;
pub use self::length::*;
pub use self::measure_::*;
pub use self::pad::*;
pub use self::page::*;
pub use self::place::*;
pub use self::point::*;
pub use self::ratio::*;
pub use self::regions::Regions;
pub use self::rel::*;
pub use self::repeat::*;
pub use self::sides::*;
pub use self::size::*;
pub use self::spacing::*;
pub use self::stack::*;
pub use self::transform::*;

pub(crate) use self::inline::*;

use comemo::{Tracked, TrackedMut};

use crate::diag::{bail, SourceResult};
use crate::engine::{Engine, Route};
use crate::eval::Tracer;
use crate::foundations::{category, Category, Scope, StyleChain, Value};
use crate::introspection::{Introspector, Locator};
use crate::model::Document;
use crate::realize::{realize_block, realize_root, Scratch};
use crate::World;

/// Arranging elements on the page in different ways.
///
/// By combining layout functions, you can create complex and automatic layouts.
#[category]
pub static LAYOUT: Category;

/// Hook up all `layout` definitions.
pub fn define(global: &mut Scope) {
    global.category(LAYOUT);
    global.define_type::<Length>();
    global.define_type::<Angle>();
    global.define_type::<Ratio>();
    global.define_type::<Rel<Length>>();
    global.define_type::<Fr>();
    global.define_type::<Dir>();
    global.define_type::<Alignment>();
    global.define_type::<PageElem>();
    global.define_type::<PagebreakElem>();
    global.define_type::<VElem>();
    global.define_type::<HElem>();
    global.define_type::<BoxElem>();
    global.define_type::<BlockElem>();
    global.define_type::<StackElem>();
    global.define_type::<GridElem>();
    global.define_type::<ColumnsElem>();
    global.define_type::<ColbreakElem>();
    global.define_type::<PlaceElem>();
    global.define_type::<AlignElem>();
    global.define_type::<PadElem>();
    global.define_type::<RepeatElem>();
    global.define_type::<MoveElem>();
    global.define_type::<ScaleElem>();
    global.define_type::<RotateElem>();
    global.define_type::<HideElem>();
    global.define_func::<measure>();
    global.define_func::<layout>();
}

/// Root-level layout.
pub trait LayoutRoot {
    /// Layout into one frame per page.
    fn layout_root(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Document>;
}

/// Layout into regions.
pub trait Layout {
    /// Layout into one frame per region.
    fn layout(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment>;

    /// Layout without side effects.
    ///
    /// This element must be layouted again in the same order for the results to
    /// be valid.
    fn measure(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        let mut locator = Locator::chained(engine.locator.track());
        let mut engine = Engine {
            world: engine.world,
            route: engine.route.clone(),
            introspector: engine.introspector,
            locator: &mut locator,
            tracer: TrackedMut::reborrow_mut(&mut engine.tracer),
        };
        self.layout(&mut engine, styles, regions)
    }
}

impl LayoutRoot for Value {
    #[typst_macros::time(name = "layout root", span = self.span())]
    fn layout_root(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Document> {
        #[comemo::memoize]
        fn cached(
            content: &Value,
            world: Tracked<dyn World + '_>,
            introspector: Tracked<Introspector>,
            route: Tracked<Route>,
            locator: Tracked<Locator>,
            tracer: TrackedMut<Tracer>,
            styles: StyleChain,
        ) -> SourceResult<Document> {
            let mut locator = Locator::chained(locator);
            let mut engine = Engine {
                world,
                introspector,
                route: Route::extend(route).unnested(),
                locator: &mut locator,
                tracer,
            };
            let scratch = Scratch::default();
            let (realized, styles) =
                realize_root(&mut engine, &scratch, content, styles)?;
            realized
                .with::<dyn LayoutRoot>()
                .unwrap()
                .layout_root(&mut engine, styles)
        }

        cached(
            self,
            engine.world,
            engine.introspector,
            engine.route.track(),
            engine.locator.track(),
            TrackedMut::reborrow_mut(&mut engine.tracer),
            styles,
        )
    }
}

impl Layout for Value {
    fn layout(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        #[allow(clippy::too_many_arguments)]
        #[comemo::memoize]
        fn cached(
            content: &Value,
            world: Tracked<dyn World + '_>,
            introspector: Tracked<Introspector>,
            route: Tracked<Route>,
            locator: Tracked<Locator>,
            tracer: TrackedMut<Tracer>,
            styles: StyleChain,
            regions: Regions,
        ) -> SourceResult<Fragment> {
            let mut locator = Locator::chained(locator);
            let mut engine = Engine {
                world,
                introspector,
                route: Route::extend(route),
                locator: &mut locator,
                tracer,
            };

            if !engine.route.within(Route::MAX_LAYOUT_DEPTH) {
                bail!(
                    content.span(), "maximum layout depth exceeded";
                    hint: "try to reduce the amount of nesting in your layout",
                );
            }

            let scratch = Scratch::default();
            let (realized, styles) =
                realize_block(&mut engine, &scratch, content, styles)?;
            realized
                .with::<dyn Layout>()
                .unwrap()
                .layout(&mut engine, styles, regions)
        }

        let fragment = cached(
            self,
            engine.world,
            engine.introspector,
            engine.route.track(),
            engine.locator.track(),
            TrackedMut::reborrow_mut(&mut engine.tracer),
            styles,
            regions,
        )?;

        engine.locator.visit_frames(&fragment);
        Ok(fragment)
    }
}
