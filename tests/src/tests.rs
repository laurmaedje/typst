//! Typst's test runner.

mod args;
mod collect;
mod compare;
mod logger;
mod refs;
mod run;
mod world;

use std::collections::HashMap;
use std::io::BufWriter;
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use ecow::EcoString;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::args::{CliArguments, Command};
use crate::logger::Logger;
use crate::refs::Refs;
use crate::run::TestResult;

/// The parsed command line arguments.
static ARGS: Lazy<CliArguments> = Lazy::new(CliArguments::parse);

/// The directory where the test suite is located.
const SUITE_PATH: &str = "tests/typ";

/// The directory where the full test results are stored.
const STORE_PATH: &str = "tests/store";

/// The file where the reference hashes are stored.
const REF_PATH: &str = "tests/ref.txt";

/// The file where the comparison report is stored.
const REPORT_PATH: &str = "tests/report.json";

fn main() {
    setup();

    match &ARGS.command {
        Some(Command::Compare(command)) => crate::compare::compare(command),
        Some(Command::Clean) => std::fs::remove_dir_all(STORE_PATH).unwrap(),
        None => {
            let (ok, _) = test();
            if !ok {
                std::process::exit(1);
            }
        }
    }
}

fn setup() {
    // Make all paths relative to the workspace. That's nicer for IDEs when
    // clicking on paths printed to the terminal.
    std::env::set_current_dir("..").unwrap();

    // Create the storage.
    for ext in ["png", "pdf", "svg"] {
        std::fs::create_dir_all(Path::new(STORE_PATH).join(ext)).unwrap();
    }

    // Set up the thread pool.
    if let Some(num_threads) = ARGS.num_threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    }
}

fn test() -> (bool, Refs) {
    let tree = Refs::tree().unwrap();
    let (tests, skipped) = match crate::collect::collect() {
        Ok(output) => output,
        Err(errors) => {
            eprintln!("Failed to collect tests");
            for error in errors {
                eprintln!("❌ {error}");
            }
            std::process::exit(1);
        }
    };

    let filtered = tests.len();
    if filtered == 0 {
        eprintln!("No test selected");
        return (true, tree.clone());
    }

    // Run the tests.
    let logger = Mutex::new(Logger::new(filtered));
    let output = std::thread::scope(|scope| {
        let (sender, receiver) = std::sync::mpsc::channel();
        let logger = &logger;

        scope.spawn(move || {
            while receiver.recv_timeout(Duration::from_millis(500)).is_err() {
                logger.lock().refresh();
            }
        });

        let output: HashMap<&EcoString, TestResult> = tests
            .iter()
            .filter(|t| !t.skipped)
            .par_bridge()
            .map(|test| {
                logger.lock().start(test);
                let result = run::run(test, &tree);
                logger.lock().finish(test, &result);
                (&test.name, result)
            })
            .collect();

        sender.send(()).unwrap();

        output
    });

    let mut refs = tree;
    for (&name, r) in &output {
        if let Some(hashes) = &r.update {
            refs.update(name.clone(), hashes.clone());
        }
    }

    // Update reference file.
    if ARGS.update {
        let file = std::fs::File::create(REF_PATH).unwrap();
        refs.write(BufWriter::new(file)).unwrap();
    }

    let passed = output.values().filter(|r| r.is_ok()).count();
    let failed = filtered - passed;
    let ok = failed == 0;
    eprintln!("{passed} passed, {failed} failed, {skipped} skipped");

    (ok, refs)
}
