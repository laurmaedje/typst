use std::ops::{Deref, DerefMut};

use crate::diag::SourceResult;
use crate::foundations::{Content, StyleChain};

use super::Realizer;

/// A pipeline of stages that process and transform content.
pub trait Pipeline<'a, 'v> {
    /// Ingests content into the pipeline.
    fn ingest(
        &mut self,
        realizer: &mut Realizer<'a, 'v>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()>;

    /// Flushes the pipeline.
    fn flush(&mut self, realizer: &mut Realizer<'a, 'v>) -> SourceResult<()>;
}

/// Definition of a single stage in a pipeline.
pub trait Stage<'a: 'v, 'v>: Sized {
    /// Runs the stage on the content. Must manually call
    /// `p.forward(content, styles)` to not suppress the content.
    fn run(
        p: &mut impl Pipe<'a, 'v, Self>,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()>;

    /// If the stage buffered any content because it couldn't yet make a
    /// decision, this should flush that content forward.
    #[allow(unused_variables)]
    fn flush(p: &mut impl Pipe<'a, 'v, Self>) -> SourceResult<()> {
        Ok(())
    }
}

/// A pipeline focused on a specific stage `S`.
///
/// Dereferences to `S`, so that the stage can access its state.
pub trait Pipe<'a: 'v, 'v, S: Stage<'a, 'v>>: DerefMut<Target = S> {
    /// Processes content from the start, going through the full pipeline.
    fn ingest(
        &mut self,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()>;

    /// Moves the content to the next stage in the pipeline.
    fn forward(
        &mut self,
        content: &'a Content,
        styles: StyleChain<'a>,
    ) -> SourceResult<()>;

    /// Mutably accesses the pipeline's shared context.
    fn realizer(&mut self) -> &mut Realizer<'a, 'v>;
}

/// An implementation of the `Pipe` trait using const generics to indicate the
/// pipeline's current stage.
struct PipeImpl<'p, 'a, 'v, P, const S: usize>(&'p mut P, &'p mut Realizer<'a, 'v>);

impl<'p, 'a, 'v, P, const N: usize> PipeImpl<'p, 'a, 'v, P, N> {
    /// Transitions the pipeline into a specific stage.
    fn transition(pipeline: &'p mut P, realizer: &'p mut Realizer<'a, 'v>) -> Self {
        Self(pipeline, realizer)
    }
}

macro_rules! pipeline {
    // Entry point.
    ($tuple:tt, $len:tt) => {
        pipeline!(@pipeline $tuple, $tuple);
        pipeline!(@pipes $tuple, $len, $tuple);
    };

    // Generates the implementation of `Pipeline` for the tuple.
    (@pipeline ($r:tt: $first:ident $($tts:tt)*), ($($m:tt: $stage:ident),*)) => {
        impl<'a: 'v, 'v, $($stage: Stage<'a, 'v>),*> Pipeline<'a, 'v>
            for ($($stage,)*)
        {
            fn ingest(
                &mut self,
                realizer: &mut Realizer<'a, 'v>,
                content: &'a Content,
                styles: StyleChain<'a>,
            ) -> SourceResult<()> {
                $first::run(&mut PipeImpl::transition(self, realizer), content, styles)
            }

            fn flush(&mut self, realizer: &mut Realizer<'a, 'v>) -> SourceResult<()> {
                // TODO: Might need a reflush.
                $($stage::flush(&mut PipeImpl::transition(self, realizer))?;)*
                Ok(())
            }
        }
    };

    // Generates the implementations of `Pipe<$stage>` for the stages.
    (@pipes $tuple:tt,$len:tt, ($($n:tt: $stage:ident),*)) => {
        $(pipeline!(@pipe $tuple, $len, $n, $stage);)*
    };

    // Generates a single implementations of `Pipe<$stage>` for a stage.
    (@pipe ($($m:tt: $s:ident),*), $len:tt, $n:tt, $stage:ident) => {
        impl<'a: 'v, 'v, $($s: Stage<'a, 'v>),*> Pipe<'a, 'v, $stage>
            for PipeImpl<'_, 'a, 'v, ($($s,)*), $n>
        {
            fn ingest(
                &mut self,
                content: &'a Content,
                styles: StyleChain<'a>,
            ) -> SourceResult<()> {
                self.0.ingest(&mut self.1, content, styles)
            }

            #[track_caller]
            fn forward(
                &mut self,
                content: &'a Content,
                styles: StyleChain<'a>,
            ) -> SourceResult<()> {
                if $n + 1 == $len {
                    panic!("cannot forward beyond the end of the pipeline");
                } else {
                    // For things to compile we fake forward to 0 if we reached
                    // the end. But the code will never run due to the if above.
                    const NEXT: usize = if $n + 1 == $len { 0 } else { $n + 1 };
                    Stage::run(&mut PipeImpl::<_, NEXT>::transition(self.0, self.1), content, styles)
                }
            }

            fn realizer(&mut self) -> &mut Realizer<'a, 'v> {
                self.1
            }
        }

        impl<$($s),*> Deref for PipeImpl<'_, '_, '_, ($($s,)*), $n> {
            type Target = $stage;

            fn deref(&self) -> &Self::Target {
                &(self.0) .$n
            }
        }

        impl<$($s),*> DerefMut for PipeImpl<'_, '_, '_, ($($s,)*), $n> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut (self.0) .$n
            }
        }
    };
}

// We only need one specific length, so we don't instantiate any other ones.
pipeline! { (0: T0), 1 }
pipeline! { (0: T0, 1: T1), 2 }
pipeline! { (0: T0, 1: T1, 2: T2), 3 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3), 4 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4), 5 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5), 6 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6), 7 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7), 8 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8), 9 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9), 10 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9, 10: T10), 11 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9, 10: T10, 11: T11), 12 }
pipeline! { (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8, 9: T9, 10: T10, 11: T11, 12: T12), 13 }
