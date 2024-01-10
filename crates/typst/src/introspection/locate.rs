use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, func, Func, Packed, Show, StyleChain, Value};
use crate::introspection::Locatable;
use crate::syntax::Span;

/// Provides access to the location of content.
///
/// This is useful in combination with [queries]($query), [counters]($counter),
/// [state]($state), and [links]($link). See their documentation for more
/// details.
///
/// ```example
/// #locate(loc => [
///   My location: \
///   #loc.position()!
/// ])
/// ```
#[func]
pub fn locate(
    /// A function that receives a [`location`]($location). Its return value is
    /// displayed in the document.
    ///
    /// This function is called once for each time the content returned by
    /// `locate` appears in the document. That makes it possible to generate
    /// content that depends on its own location in the document.
    func: Func,
) -> Value {
    LocateElem::new(func).pack()
}

/// Executes a `locate` call.
#[ty(Locatable, Show)]
struct LocateElem {
    /// The function to call with the location.
    func: Func,
}

impl Show for Packed<LocateElem> {
    #[typst_macros::time(name = "locate", span = self.span())]
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Value> {
        Ok(engine.delayed(|engine| {
            let location = self.location().unwrap();
            self.func.call(engine, [location])
        }))
    }
}
