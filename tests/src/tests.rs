//! Typst's test runner.

mod args;
mod collect;
mod compare;
mod logger;
mod run;
mod world;

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use ecow::EcoString;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::args::{CliArguments, Command};
use crate::compare::Refs;
use crate::logger::Logger;
use crate::run::TestResult;

/// The parsed command line arguments.
static ARGS: Lazy<CliArguments> = Lazy::new(CliArguments::parse);

/// The directory where the test suite is located.
const SUITE_PATH: &str = "typ";

/// The directory where the full test results are stored.
const STORE_PATH: &str = "store";

/// The file where the reference hashes are stored.
const REF_PATH: &str = "ref.txt";

/// The file where the comparison report is stored.
const REPORT_PATH: &str = "report.html";

fn main() {
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

fn test() -> (bool, Refs) {
    let tree = Refs::tree();
    let tests = match crate::collect::collect() {
        Ok(output) => output,
        Err(errors) => {
            eprintln!("Failed to collect tests");
            for error in errors {
                eprintln!("❌ {error}");
            }
            std::process::exit(1);
        }
    };

    let filtered = tests.iter().filter(|t| !t.skipped).count();
    if filtered == 0 {
        eprintln!("No test selected");
        return (true, tree.clone());
    }

    eprintln!("Running {} / {} tests", filtered, tests.len());

    for ext in ["png", "pdf", "svg"] {
        std::fs::create_dir_all(Path::new(STORE_PATH).join(ext)).unwrap();
    }

    if let Some(num_threads) = ARGS.num_threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    }

    let logger = Mutex::new(Logger::new(filtered));
    let results: HashMap<&EcoString, TestResult> = tests
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

    let passed = results.values().filter(|r| r.is_ok()).count();
    let ok = passed == filtered;
    if ok {
        eprintln!("All {filtered} tests passed");
    } else {
        eprintln!("{passed} / {filtered} tests passed");
    }

    if ARGS.update {
        let mut file = std::fs::File::create(REF_PATH).unwrap();
        let mut first = true;
        for test in &tests {
            let update = results.get(&test.name).and_then(|r| r.update.as_ref());
            if update.is_some() {
                eprintln!("🌊 Updating reference for {test}");
            }

            if let Some(hashes) = update.or(tree.get(&test.name)) {
                if !first {
                    writeln!(file).unwrap();
                }
                writeln!(
                    file,
                    "{}\n{}\n{}\n{}",
                    test.name, hashes.render, hashes.pdf, hashes.svg
                )
                .unwrap();
                first = false;
            }
        }
    }

    (ok, Refs::from_results(&results, &tree))
}
