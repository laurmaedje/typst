use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use ecow::EcoString;
use once_cell::unsync::Lazy;

/// A collection of reference hashes for tests.
#[derive(Clone)]
pub struct Refs(pub BTreeMap<EcoString, TestHashes>);

impl Refs {
    /// Get the `TREE` refs.
    pub fn tree() -> io::Result<Self> {
        let file = std::fs::File::open(crate::REF_PATH)?;
        Self::read(BufReader::new(file))
    }

    /// Retrieve the contents of a file at a specified revision.
    pub fn at_rev(rev: &str, live: &Lazy<Self>) -> io::Result<Refs> {
        match rev {
            "TREE" => return Self::tree(),
            "LIVE" => return Ok((**live).clone()),
            _ => {}
        }

        let output = Command::new("git")
            .arg("show")
            .arg(format!("{rev}:{}", crate::REF_PATH))
            .output()
            .unwrap();
        if !output.stderr.is_empty() {
            io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(1);
        }
        Refs::read(output.stdout.as_slice())
    }

    /// Parse refs from the reference file.
    pub fn read(reader: impl BufRead) -> io::Result<Self> {
        let mut iter = reader.lines();
        let mut hashes = BTreeMap::new();
        while let Some(name) = iter.next() {
            let mut read = || {
                iter.next()
                    .ok_or_else(|| io::Error::from(io::ErrorKind::UnexpectedEof))?
            };
            hashes.insert(
                name?.into(),
                TestHashes {
                    render: TestHash(read()?),
                    // pdf: TestHash(read()?),
                    // svg: TestHash(read()?),
                },
            );
            if !read()?.is_empty() {
                return Err(io::ErrorKind::InvalidData.into());
            }
        }
        Ok(Self(hashes))
    }

    /// Encode refs into a reference file.
    pub fn write(&self, mut writer: impl Write) -> io::Result<()> {
        for (name, hashes) in &self.0 {
            writeln!(writer, "{name}")?;
            writeln!(writer, "{}", hashes.render)?;
            // writeln!(writer, "{}", hashes.pdf)?;
            // writeln!(writer, "{}", hashes.svg)?;
            writeln!(writer)?;
        }
        Ok(())
    }

    /// Update or insert hashes.
    pub fn update(&mut self, name: EcoString, hashes: TestHashes) {
        self.0.insert(name, hashes);
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
    pub render: TestHash<'r'>,
    // /// The hash for the `typst-pdf` output.
    // pub pdf: TestHash<'p'>,
    // /// The hash for the `typst-svg` output.
    // pub svg: TestHash<'s'>,
}

/// A hash of a test output artifact.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestHash<const KIND: char>(String);

impl<const KIND: char> TestHash<KIND> {
    /// Compute a test hash from bytes.
    pub fn compute(bytes: impl AsRef<[u8]>) -> Self {
        use sha2::Digest;
        use std::fmt::Write;

        let hash = {
            let mut digest = sha2::Sha256::new();
            digest.update(bytes);
            digest.finalize()
        };

        // We truncate the hash to 64-bit. That's enough for our purposes since
        // it is very unlikely for a bad test result to accidentally collide
        // with the correct one even with just this.
        let prefix = &hash.as_slice()[..8];

        // Encode as hex.
        let mut string = String::new();
        for byte in prefix {
            write!(string, "{:02x}", byte).unwrap();
        }

        Self(string)
    }

    /// Get the raw hash.
    pub fn get(&self) -> &str {
        &self.0
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
            'r' => "png",
            'p' => "pdf",
            's' => "svg",
            _ => panic!("invalid test hash kind"),
        };
        let stem = format!("{}-{}", name, self.0);
        store.join(ext).join(stem).with_extension(ext)
    }
}

impl<const KIND: char> Display for TestHash<KIND> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
