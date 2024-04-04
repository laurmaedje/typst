use std::fmt::Write;
use std::ops::Range;

use tiny_skia as sk;
use typst::diag::SourceDiagnostic;
use typst::eval::Tracer;
use typst::foundations::Smart;
use typst::introspection::Meta;
use typst::layout::{Abs, Frame, FrameItem, Transform};
use typst::model::Document;
use typst::visualize::Color;
use typst::WorldExt;

use crate::collect::{NoteKind, Test};
use crate::compare::{Refs, TestHash, TestHashes};
use crate::world::TestWorld;

/// Runs a single test.
///
/// Returns whether the test passed.
pub fn run(test: &Test, tree: &Refs) -> TestResult {
    Runner::new(test, tree).run()
}

/// The result of running a single test.
pub struct TestResult {
    /// The error log for this test. If empty, the test passed.
    pub errors: String,
    /// The new hashes, if the hashes were mismatched.
    pub update: Option<TestHashes>,
}

impl TestResult {
    /// Whether the test passed.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Write a line to the test's error log.
macro_rules! errln {
    ($runner:expr, $($tts:tt)*) => {
        writeln!(&mut $runner.result.errors, $($tts)*).unwrap();
    }
}

/// Runs a single test.
pub struct Runner<'a> {
    test: &'a Test,
    tree: &'a Refs,
    world: TestWorld,
    seen: Vec<bool>,
    result: TestResult,
}

impl<'a> Runner<'a> {
    /// Create a new test runner.
    fn new(test: &'a Test, tree: &'a Refs) -> Self {
        Self {
            test,
            tree,
            world: TestWorld::new(test.source.clone()),
            seen: vec![false; test.notes.len()],
            result: TestResult { errors: String::new(), update: None },
        }
    }

    /// Run the test.
    fn run(mut self) -> TestResult {
        let _guard = PanicGuard(self.test);

        let mut tracer = Tracer::new();
        match typst::compile(&self.world, &mut tracer) {
            Ok(doc) => self.check_document(&doc),
            Err(errors) => {
                for error in &errors {
                    self.check_diagnostic(NoteKind::Error, error);
                }
            }
        }

        for warning in tracer.warnings() {
            self.check_diagnostic(NoteKind::Warning, &warning);
        }

        for (note, &seen) in self.test.notes.iter().zip(&self.seen) {
            // TODO: Handle hints.
            if note.kind == NoteKind::Hint || seen {
                continue;
            }
            let base = note.pos.line - self.test.pos.line + 1;
            let note_range = self.format_range(&note.range, base);
            errln!(
                self,
                // TODO: Looks bad.
                "  Not emitted ({}): {}: {:<10} {}",
                note.pos,
                note.kind,
                note_range,
                note.message
            );
        }

        self.result
    }

    /// Check that the document output is correct.
    fn check_document(&mut self, document: &Document) {
        if let Some(reference) = self.tree.get(&self.test.name) {
            let hashes = self.compute_hashes(document);

            if hashes.render != reference.render {
                errln!(self, "  Mismatched rendering");
            }

            // TODO: Add back in once it's reproducible.
            // if hashes.pdf != reference.pdf {
            //     errln!(self, "  Mismatched PDF");
            // }

            // TODO: Add back in once it's reproducible.
            // if hashes.svg != reference.svg {
            //     errln!(self, "  Mismatched SVG");
            // }

            if hashes != *reference {
                self.result.update = Some(hashes);
            }
        } else {
            let skippable = match document.pages.as_slice() {
                [page] => skippable(&page.frame),
                _ => false,
            };
            if !skippable {
                let hashes = self.compute_hashes(document);
                self.result.update = Some(hashes);
                errln!(self, "  Missing reference hashes");
            }
        }
    }

    /// Compute the test's hashes.
    fn compute_hashes(&self, document: &Document) -> TestHashes {
        let pixmap = render(document);
        let png = pixmap.encode_png().unwrap();
        let pdf = typst_pdf::pdf(document, Smart::Auto, None);
        let svg = typst_svg::svg_merged(document, Abs::pt(5.0));

        let hashes = TestHashes {
            render: TestHash::compute(pixmap.data()),
            pdf: TestHash::compute(&pdf),
            svg: TestHash::compute(&svg),
        };

        let name = &self.test.name;
        hashes.render.write(name, png.as_slice());
        hashes.pdf.write(name, pdf.as_slice());
        hashes.svg.write(name, svg.as_bytes());

        hashes
    }

    /// Compare a subset of notes with a given kind against diagnostics of
    /// that same kind.
    fn check_diagnostic(&mut self, kind: NoteKind, diag: &SourceDiagnostic) {
        // Ignore diagnostics from other files.
        if diag.span.id().is_some_and(|id| id != self.test.source.id()) {
            return;
        }

        let range = self.world.range(diag.span);
        let message = diag.message.replace("\\", "/");

        // Try to find perfect match.
        if let Some((i, _)) = self.test.notes.iter().enumerate().find(|&(i, note)| {
            !self.seen[i]
                && note.kind == kind
                && note.range == range
                && note.message == message
        }) {
            self.seen[i] = true;
            return;
        }

        // Try to find closely matching annotation. If the note has the same
        // range or message, it's most likely the one we're interested in.
        let Some((i, note)) = self.test.notes.iter().enumerate().find(|&(i, note)| {
            !self.seen[i]
                && note.kind == kind
                && (note.range == range || note.message == message)
        }) else {
            // Not even a close match, diagnostic is not annotated.
            let diag_range = self.format_range(&range, 0);
            errln!(self, "  Not annotated: {kind}: {:<10} {}", diag_range, diag.message);
            return;
        };

        // Mark this annotation as visited.
        self.seen[i] = true;

        // Range is wrong.
        if range != note.range {
            let base = note.pos.line - self.test.pos.line + 1;
            let note_range = self.format_range(&note.range, base);
            let note_text = self.text_for_range(&note.range);
            let diag_range = self.format_range(&range, base);
            let diag_text = self.text_for_range(&range);
            errln!(self, "  Mismatched range ({}):", note.pos);
            errln!(self, "    Message   | {}", note.message);
            errln!(self, "    Annotated | {:<9} | {}", note_range, note_text);
            errln!(self, "    Emitted   | {:<9} | {}", diag_range, diag_text);
        }

        // Message is wrong.
        if message != note.message {
            errln!(self, "  Mismatched message ({}):", note.pos);
            errln!(self, "    Annotated | {}", note.message);
            errln!(self, "    Emitted   | {}", message);
        }
    }

    /// Display the text for a range.
    fn text_for_range(&self, range: &Option<Range<usize>>) -> String {
        let Some(range) = range else { return "No text".into() };
        if range.is_empty() {
            "(empty)".into()
        } else {
            format!("`{}`", self.test.source.text()[range.clone()].replace('\n', "\\n"))
        }
    }

    /// Display a byte range as a line:column range.
    fn format_range(&self, range: &Option<Range<usize>>, base: usize) -> String {
        let Some(range) = range else { return "No range".into() };
        if range.start == range.end {
            self.format_pos(range.start, base)
        } else {
            format!(
                "{}-{}",
                self.format_pos(range.start, base),
                self.format_pos(range.end, base)
            )
        }
    }

    /// Display a position as a line:column pair.
    fn format_pos(&self, pos: usize, base: usize) -> String {
        if let (Some(line_idx), Some(column_idx)) =
            (self.test.source.byte_to_line(pos), self.test.source.byte_to_column(pos))
        {
            let line = line_idx + 1 - base;
            let column = column_idx + 1;
            if line == 1 {
                format!("{column}")
            } else {
                format!("{line}:{column}")
            }
        } else {
            "oob".into()
        }
    }
}

/// Draw all frames into one image with padding in between.
fn render(document: &Document) -> sk::Pixmap {
    let pixel_per_pt = 2.0;
    let padding = Abs::pt(5.0);

    for page in &document.pages {
        let limit = Abs::cm(100.0);
        if page.frame.width() > limit || page.frame.height() > limit {
            panic!("overlarge frame: {:?}", page.frame.size());
        }
    }

    let mut pixmap = typst_render::render_merged(
        document,
        pixel_per_pt,
        Color::WHITE,
        padding,
        Color::BLACK,
    );

    let padding = (pixel_per_pt * padding.to_pt() as f32).round();
    let [x, mut y] = [padding; 2];
    for page in &document.pages {
        let ts =
            sk::Transform::from_scale(pixel_per_pt, pixel_per_pt).post_translate(x, y);
        render_links(&mut pixmap, ts, &page.frame);
        y += (pixel_per_pt * page.frame.height().to_pt() as f32).round().max(1.0)
            + padding;
    }

    pixmap
}

/// Draw extra boxes for links so we can see whether they are there.
fn render_links(canvas: &mut sk::Pixmap, ts: sk::Transform, frame: &Frame) {
    for (pos, item) in frame.items() {
        let ts = ts.pre_translate(pos.x.to_pt() as f32, pos.y.to_pt() as f32);
        match *item {
            FrameItem::Group(ref group) => {
                let ts = ts.pre_concat(to_sk_transform(&group.transform));
                render_links(canvas, ts, &group.frame);
            }
            FrameItem::Meta(Meta::Link(_), size) => {
                let w = size.x.to_pt() as f32;
                let h = size.y.to_pt() as f32;
                let rect = sk::Rect::from_xywh(0.0, 0.0, w, h).unwrap();
                let mut paint = sk::Paint::default();
                paint.set_color_rgba8(40, 54, 99, 40);
                canvas.fill_rect(rect, &paint, ts, None);
            }
            _ => {}
        }
    }
}

/// Whether rendering of a frame can be skipped.
fn skippable(frame: &Frame) -> bool {
    frame.items().all(|(_, item)| match item {
        FrameItem::Group(group) => skippable(&group.frame),
        FrameItem::Meta(..) => true,
        _ => false,
    })
}

/// Convert a Typst transform to a tiny-skia transform.
fn to_sk_transform(transform: &Transform) -> sk::Transform {
    let Transform { sx, ky, kx, sy, tx, ty } = *transform;
    sk::Transform::from_row(
        sx.get() as _,
        ky.get() as _,
        kx.get() as _,
        sy.get() as _,
        tx.to_pt() as f32,
        ty.to_pt() as f32,
    )
}

/// Prints which test failed when there is a panic.
struct PanicGuard<'a>(&'a Test);

impl Drop for PanicGuard<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!("note: panicked in test {}", self.0);
        }
    }
}
