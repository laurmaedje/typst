use std::io::Write;
use std::time::{Duration, Instant};

use crate::collect::Test;
use crate::run::TestResult;

/// Receives status updates by individual test runs.
pub struct Logger<'a> {
    filtered: usize,
    done: usize,
    active: Vec<(&'a Test, Instant)>,
    temp_lines: usize,
}

impl<'a> Logger<'a> {
    /// Create a new logger.
    pub const fn new(filtered: usize) -> Self {
        Self { filtered, done: 0, active: vec![], temp_lines: 0 }
    }

    /// Register the start of a test.
    pub fn start(&mut self, test: &'a Test) {
        self.active.push((test, Instant::now()));
        self.print(None).unwrap();
    }

    /// Refresh the status.
    pub fn refresh(&mut self) {
        self.print(None).unwrap();
    }

    /// Register a finished test.
    pub fn finish(&mut self, test: &'a Test, result: &TestResult) {
        self.active.retain(|(t, _)| t.name != test.name);
        self.done += 1;
        self.print(Some((test, result))).unwrap();
    }

    /// Refresh the status print.
    pub fn print(
        &mut self,
        finished: Option<(&'a Test, &TestResult)>,
    ) -> std::io::Result<()> {
        let mut out = std::io::stderr().lock();

        // Clear the status lines.
        for _ in 0..self.temp_lines {
            write!(out, "\x1B[1F\x1B[0J")?;
            self.temp_lines = 0;
        }

        // Print the result of a finished test.
        if let Some((test, result)) = finished {
            if !result.errors.is_empty() {
                writeln!(out, "❌ {test}")?;
                write!(out, "{}", result.errors)?;

                if crate::ARGS.update && result.update.is_some() {
                    writeln!(out, "  Updating reference for {test}")?;
                }
            } else if crate::ARGS.verbose {
                writeln!(out, "✅ {test}")?;
            }
        }

        // Print the status line.
        if self.done < self.filtered {
            for (test, started) in &self.active {
                if started.elapsed() > Duration::from_secs(2) {
                    writeln!(out, "⏰ {test} is taking a long time ...")?;
                    self.temp_lines += 1;
                }
            }
            writeln!(out, "💨 {} / {}", self.done, self.filtered)?;
            self.temp_lines += 1;
        }

        Ok(())
    }
}
