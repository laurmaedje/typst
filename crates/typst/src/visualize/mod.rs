//! Drawing and visualization.

mod color;
mod gradient;
mod image;
mod line;
mod paint;
mod path;
mod pattern;
mod polygon;
mod shape;
mod stroke;

pub use self::color::*;
pub use self::gradient::*;
pub use self::image::*;
pub use self::line::*;
pub use self::paint::*;
pub use self::path::*;
pub use self::pattern::*;
pub use self::polygon::*;
pub use self::shape::*;
pub use self::stroke::*;

use crate::foundations::{category, Category, Scope};

/// Drawing and data visualization.
///
/// If you want to create more advanced drawings or plots, also have a look at
/// the [CetZ](https://github.com/johannes-wolf/cetz) package as well as more
/// specialized [packages]($packages) for your use case.
#[category]
pub static VISUALIZE: Category;

/// Hook up all visualize definitions.
pub(super) fn define(global: &mut Scope) {
    global.category(VISUALIZE);
    global.define_type::<Color>();
    global.define_type::<Gradient>();
    global.define_type::<Pattern>();
    global.define_type::<Stroke>();
    global.define_type::<ImageElem>();
    global.define_type::<LineElem>();
    global.define_type::<RectElem>();
    global.define_type::<SquareElem>();
    global.define_type::<EllipseElem>();
    global.define_type::<CircleElem>();
    global.define_type::<PolygonElem>();
    global.define_type::<PathElem>();
}
