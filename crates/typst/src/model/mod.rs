//! Structuring elements that define the document model.

mod bibliography;
mod cite;
mod document;
mod emph;
#[path = "enum.rs"]
mod enum_;
mod figure;
mod footnote;
mod heading;
mod link;
mod list;
#[path = "numbering.rs"]
mod numbering_;
mod outline;
mod par;
mod quote;
mod reference;
mod strong;
mod table;
mod terms;

pub use self::bibliography::*;
pub use self::cite::*;
pub use self::document::*;
pub use self::emph::*;
pub use self::enum_::*;
pub use self::figure::*;
pub use self::footnote::*;
pub use self::heading::*;
pub use self::link::*;
pub use self::list::*;
pub use self::numbering_::*;
pub use self::outline::*;
pub use self::par::*;
pub use self::quote::*;
pub use self::reference::*;
pub use self::strong::*;
pub use self::table::*;
pub use self::terms::*;

use crate::foundations::{category, Category, Scope};

/// Document structuring.
///
/// Here, you can find functions to structure your document and interact with
/// that structure. This includes section headings, figures, bibliography
/// management, cross-referencing and more.
#[category]
pub static MODEL: Category;

/// Hook up all `model` definitions.
pub fn define(global: &mut Scope) {
    global.category(MODEL);
    global.define_type::<DocumentElem>();
    global.define_type::<RefElem>();
    global.define_type::<LinkElem>();
    global.define_type::<OutlineElem>();
    global.define_type::<HeadingElem>();
    global.define_type::<FigureElem>();
    global.define_type::<FootnoteElem>();
    global.define_type::<QuoteElem>();
    global.define_type::<CiteElem>();
    global.define_type::<BibliographyElem>();
    global.define_type::<EnumElem>();
    global.define_type::<ListElem>();
    global.define_type::<ParbreakElem>();
    global.define_type::<ParElem>();
    global.define_type::<TableElem>();
    global.define_type::<TermsElem>();
    global.define_type::<EmphElem>();
    global.define_type::<StrongElem>();
    global.define_func::<numbering>();
}
