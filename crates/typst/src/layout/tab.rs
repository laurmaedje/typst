use smallvec::{smallvec, SmallVec};

use crate::diag::HintedStrResult;
use crate::foundations::{cast, elem, Array, Packed, Resolve, StyleChain, Value};
use crate::layout::{Abs, Em, HAlignment, Rel};
use crate::realize::{Behave, Behaviour};

/// Inserts spacing to snap to a tab stop.
#[elem(title = "Tab Stop", Behave)]
pub struct TabElem {
    /// The distances between snap positions.
    ///
    /// May be either a single relative length or an array of them. The
    /// length(s) are cycled and accumulated to define the available stops.
    #[default]
    pub stops: TabStops,

    /// The alignment of this tab stop.
    ///
    /// This defines how the content _following the tab stop_ is aligned with
    /// respect to the stop.
    #[default(HAlignment::Start)]
    pub align: HAlignment,
}

impl Behave for Packed<TabElem> {
    fn behaviour(&self) -> Behaviour {
        Behaviour::Destructive
    }
}

/// Tab stop definitions.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TabStops(SmallVec<[Rel; 1]>);

impl TabStops {
    /// Determines the position for the next tab stop.
    pub fn resolve(&self, styles: StyleChain, base: Abs) -> ResolvedTabStops {
        ResolvedTabStops::new(
            self.0.iter().map(|rel| rel.resolve(styles).relative_to(base)),
        )
    }
}

cast! {
    TabStops,
    self => self.0.into_value(),
    v: Rel => Self(smallvec![v]),
    values: Array => Self(
        values
            .into_iter()
            .map(Value::cast)
            .collect::<HintedStrResult<_>>()?
    ),
}

impl Default for TabStops {
    fn default() -> Self {
        Self(smallvec![Em::new(2.0).into()])
    }
}

/// Tab stop positions that have been resolved with styles and a base width.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ResolvedTabStops {
    stops: SmallVec<[Abs; 1]>,
    period: Abs,
}

impl ResolvedTabStops {
    /// Create resolved stops from an iterator.
    pub fn new(stops: impl IntoIterator<Item = Abs>) -> Self {
        let stops: SmallVec<[Abs; 1]> = stops.into_iter().collect();
        let period = stops.iter().sum();
        ResolvedTabStops { stops, period }
    }

    /// Returns the next snap position for the given line cursor.
    pub fn snap(&self, cursor: Abs) -> Abs {
        // Snap to the nearest smaller multiple of the period.
        let mut snapped = cursor - (cursor % self.period);

        // Find the concrete stop to snap to.
        for &stop in &self.stops {
            if snapped > cursor {
                break;
            }
            snapped += stop;
        }

        snapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::assert_approx_eq;

    #[track_caller]
    fn test(stops: &[f64], cursor: f64, expected: f64) {
        let stops = ResolvedTabStops::new(stops.iter().map(|&s| Abs::pt(s)));
        assert_approx_eq!(stops.snap(Abs::pt(cursor)), Abs::pt(expected));
    }

    #[test]
    fn test_tab_stop_snap() {
        test(&[2.0], 1.6, 2.0);
        test(&[2.0], 2.1, 4.0);
        test(&[2.0, 0.7], 2.1, 2.7);
        test(&[2.0, 0.7], 2.9, 4.7);
        test(&[2.0, 0.7], 5.3, 5.4);
        test(&[2.0, 0.7], -3.0, -2.7);
        test(&[2.0, 0.7], 0.0, 2.0);
        test(&[2.0, 0.7], 15.0, 15.5);
    }
}
