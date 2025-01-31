use hdf5::File;

/// A detector module
#[derive(Debug)]
pub struct Module {
    /// The data files written by the module
    files: Vec<File>,
}

impl Module {
    /// Sets up a new detector [`Module`] with a collection of round-robined data [`File`]s
    pub fn new(files: Vec<File>) -> Self {
        Module { files }
    }

    /// Provides access to the undereling [`File`]s
    pub fn files(&self) -> &[File] {
        &self.files
    }
}
