#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Executable {
    pub contents: Vec<u8>,
    pub name: String,
}

impl fmt::Debug for Executable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("File")
            .field("name", &self.name)
            .field("size", &self.contents.len())
            .finish()
    }
}

impl Executable {
    pub fn open<P>(path: P) -> Self
        where P: AsRef<Path>
    {
        Self::open_(path.as_ref())
    }

    fn open_(path: &Path) -> Self {
        let mut contents = Vec::new();
        File::open(path).unwrap().read_to_end(&mut contents).unwrap();

        Executable {
            contents: contents,
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Output {
    pub status: ExitStatus,
    pub stderr: Vec<u8>,
    pub stdout: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExitStatus {
    pub code: Option<i32>,
    pub success: bool,
}
