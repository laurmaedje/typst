#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};

use once_cell::unsync::Lazy;

use crate::args::CompareCommand;
use crate::refs::{Refs, TestHash, TestHashes};

/// Produces a comparison report between test results of two revisions.
pub fn compare(command: &CompareCommand) {
    let old_rev = command.old.to_uppercase();
    let new_rev = command.new.to_uppercase();

    let live = Lazy::<Refs, fn() -> Refs>::new(|| crate::test().1);
    let old_refs = Refs::at_rev(&old_rev, &live).unwrap();
    let new_refs = Refs::at_rev(&new_rev, &live).unwrap();

    let report = Report {
        old: &old_rev,
        new: &new_rev,
        changes: diff(&old_refs, &new_refs),
    };

    let data = serde_json::to_vec_pretty(&report).unwrap();
    std::fs::write(crate::REPORT_PATH, &data).unwrap();
    eprintln!("Wrote {}", crate::REPORT_PATH);
}

#[derive(Serialize)]
struct Report<'a> {
    old: &'a str,
    new: &'a str,
    changes: Vec<Change<'a>>,
}

#[derive(Serialize)]
struct Change<'a> {
    name: &'a str,
    added: &'a str,
    removed: &'a str,
}

/// Diffs two `Refs` instances.
fn diff<'a>(old_refs: &'a Refs, new_refs: &'a Refs) -> Vec<Change<'a>> {
    let mut changes = vec![];

    for (name, old) in &old_refs.0 {
        if let Some(new) = new_refs.get(name) {
            if new != old {
                changes.push(Change {
                    name,
                    added: new.render.get(),
                    removed: old.render.get(),
                });
            }
        } else {
            // changes.push(Change { name, new: None, old: Some(old) });
        }
    }

    // for (name, new) in &new_refs.0 {
    //     if old_refs.get(name).is_none() {
    //         changes.push(Change { name, new: Some(new), old: None });
    //     }
    // }

    changes.sort_by_key(|change| change.name);
    changes
}
