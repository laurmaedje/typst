use std::io::Write;

use crate::collect::Test;
use crate::run::TestResult;

/// Receives status updates by individual test runs.
pub struct Logger<'a> {
    active: Vec<&'a Test>,
    suffix: usize,
    done: usize,
    count: usize,
}

impl<'a> Logger<'a> {
    /// Create a new logger.
    pub const fn new(count: usize) -> Self {
        Self { active: vec![], suffix: 0, done: 0, count }
    }

    /// Register the start of a test.
    pub fn start(&mut self, test: &'a Test) {
        self.active.push(test);
        self.print(None).unwrap();
    }

    /// Register a finished test.
    pub fn finish(&mut self, test: &'a Test, result: &TestResult) {
        self.done += 1;
        self.active.retain(|t| t.name != test.name);
        self.print(Some((test, result))).unwrap();
    }

    /// Refresh the status print.
    pub fn print(
        &mut self,
        finished: Option<(&'a Test, &TestResult)>,
    ) -> std::io::Result<()> {
        let mut out = std::io::stderr().lock();

        // Clear the stauts lines.
        for _ in 0..self.suffix {
            write!(out, "\x1B[1F\x1B[0J")?;
            self.suffix = 0;
        }

        // Print the result of a finished test.
        if let Some((test, result)) = finished {
            if !result.errors.is_empty() {
                writeln!(out, "❌ {test}")?;
                write!(out, "{}", result.errors)?;
            } else if crate::ARGS.verbose {
                writeln!(out, "✅ {test}")?;
            }
        }

        // Print the status line.
        if self.done < self.count {
            write!(out, "💨 {} / {}", self.done, self.count)?;
            // if let Some(active) = self.active.first() {
            write!(out, " ({})", self.active.len())?;
            // }
            writeln!(out)?;
            self.suffix = 1;
        }

        Ok(())
    }
}
