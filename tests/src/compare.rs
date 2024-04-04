use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use base64::Engine;
use ecow::EcoString;
use once_cell::unsync::Lazy;
use sha2::Digest;

use crate::args::CompareCommand;
use crate::run::TestResult;

/// Produces a comparison report between test results of two revisions.
pub fn compare(command: &CompareCommand) {
    let old_rev = command.old.to_uppercase();
    let new_rev = command.new.to_uppercase();

    let live = Lazy::<Refs, fn() -> Refs>::new(|| crate::test().1);
    let old_refs = Refs::at_rev(&old_rev, &live);
    let new_refs = Refs::at_rev(&new_rev, &live);

    let changes = diff(&old_refs, &new_refs);

    let mut file = File::create(crate::REPORT_PATH).unwrap();
    report(&mut file, &old_rev, &new_rev, &changes).unwrap();
    drop(file);

    if command.open {
        open::that(crate::REPORT_PATH).unwrap();
    }
}

/// Write the formatted report.
pub fn report(
    out: &mut File,
    old_rev: &str,
    new_rev: &str,
    changes: &[Change],
) -> std::io::Result<()> {
    let title = format!("Comparing {old_rev} with {new_rev}");

    writeln!(
        out,
        "
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset=\"UTF-8\">
            <title>{title}</title>
        </head>
        <body>
        <h1>{title}</h1>
    "
    )?;

    for change in changes {
        let name = change.name;
        writeln!(out, "<section><h2>{name}</h2>")?;
        match (change.old, change.new) {
            (None, None) => {}
            (Some(old), None) => {
                writeln!(out, "<img src=\"{}\" />", old.render.path(name).display())?;
            }
            (None, Some(new)) => {
                writeln!(out, "<img src=\"{}\" />", new.render.path(name).display())?;
            }
            (Some(old), Some(new)) => {
                // let old_path = old.render.path(change.name);
                // if !old_path.exists() {
                //     eprintln!("Missing comparison artifact {}", old_path.display());
                //     eprintln!("Please run tests at revision {}", command.old);
                //     std::process::exit(1);
                // }
                writeln!(out, "<img src=\"{}\" />", old.render.path(name).display())?;
                writeln!(out, "<img src=\"{}\" />", new.render.path(name).display())?;
            }
        }
        writeln!(out, "</section>")?;
    }

    writeln!(out, "</body></html>")
}

/// A collection of reference hashes for tests.
#[derive(Clone)]
pub struct Refs(HashMap<EcoString, TestHashes>);

impl Refs {
    /// Get the `TREE` refs.
    pub fn tree() -> Self {
        let text = std::fs::read_to_string(crate::REF_PATH).unwrap();
        Self::parse(&text)
    }

    /// Retrieve the contents of a file at a specified revision.
    pub fn at_rev(rev: &str, live: &Lazy<Self>) -> Refs {
        match rev {
            "TREE" => return Self::tree(),
            "LIVE" => return (**live).clone(),
            _ => {}
        }

        let path = format!("tests/{}", crate::REF_PATH);
        let output = Command::new("git")
            .arg("show")
            .arg(format!("{rev}:{path}"))
            .output()
            .unwrap();
        if !output.stderr.is_empty() {
            std::io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(1);
        }
        let string = String::from_utf8(output.stdout).unwrap();
        Refs::parse(&string)
    }

    /// Parse refs from the reference file.
    pub fn parse(text: &str) -> Self {
        let mut iter = text.lines();
        let mut hashes = HashMap::new();
        while let Some(name) = iter.next() {
            let render = TestHash(iter.next().unwrap().into());
            let pdf = TestHash(iter.next().unwrap().into());
            let svg = TestHash(iter.next().unwrap().into());
            hashes.insert(name.into(), TestHashes { render, pdf, svg });
            iter.next();
        }
        Self(hashes)
    }

    pub fn from_results(results: &HashMap<&EcoString, TestResult>, tree: &Self) -> Self {
        let mut map = HashMap::new();
        for (&name, r) in results {
            if let Some(hashes) = r.update.as_ref().or(tree.get(name)) {
                map.insert(name.clone(), hashes.clone());
            }
        }
        Self(map)
    }

    /// Get the hashes for a particular test.
    pub fn get(&self, name: &str) -> Option<&TestHashes> {
        self.0.get(name)
    }
}

/// Reference hashes for a test.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestHashes {
    /// The hash for the `typst-render` output.
    pub render: TestHash<0>,
    /// The hash for the `typst-pdf` output.
    pub pdf: TestHash<1>,
    /// The hash for the `typst-svg` output.
    pub svg: TestHash<2>,
}

/// A hash of a test output artifact.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestHash<const KIND: usize>(String);

impl<const KIND: usize> TestHash<KIND> {
    /// Compute a test hash from bytes.
    pub fn compute(bytes: impl AsRef<[u8]>) -> Self {
        let mut digest = sha2::Sha256::new();
        digest.update(bytes);
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;

        // We only take the first 16 bytes. That's enough for our purposes.
        Self(engine.encode(&digest.finalize().as_slice()[..16]))
    }

    /// Write a artifact for this hash to the store.
    pub fn write(&self, name: &str, buf: impl AsRef<[u8]>) {
        std::fs::write(self.path(name), buf).unwrap()
    }

    /// Read the artifact for this hash from the store.
    #[allow(unused)]
    pub fn read(&self, name: &str) -> io::Result<Vec<u8>> {
        std::fs::read(self.path(name))
    }

    /// The path to the store file for this test's artifact.
    pub fn path(&self, name: &str) -> PathBuf {
        let store = Path::new(crate::STORE_PATH);
        let ext = match KIND {
            0 => "png",
            1 => "pdf",
            2 => "svg",
            _ => panic!("invalid test hash kind"),
        };
        let stem = format!("{}-{}", name, self.0);
        store.join(ext).join(stem).with_extension(ext)
    }
}

impl<const KIND: usize> Display for TestHash<KIND> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Diffs two `Refs` instances.
pub fn diff<'a>(old_refs: &'a Refs, new_refs: &'a Refs) -> Vec<Change<'a>> {
    let mut changes = vec![];

    for (name, old) in &old_refs.0 {
        if let Some(new) = new_refs.get(name) {
            if new != old {
                changes.push(Change { name, new: Some(new), old: Some(old) });
            }
        } else {
            changes.push(Change { name, new: None, old: Some(old) });
        }
    }

    for (name, new) in &new_refs.0 {
        if old_refs.get(name).is_none() {
            changes.push(Change { name, new: Some(new), old: None });
        }
    }

    changes.sort_by_key(|change| change.name);
    changes
}

/// A change to the references.
pub struct Change<'a> {
    pub name: &'a str,
    pub new: Option<&'a TestHashes>,
    pub old: Option<&'a TestHashes>,
}
