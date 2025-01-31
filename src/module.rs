use hdf5::{Dataset, File};

/// A detector module
#[derive(Debug)]
pub struct Module {
    /// The data files written by the module
    files: Vec<File>,
    /// The event IDs, containing position information
    event_ids: Vec<Dataset>,
    /// The event energies
    event_energies: Vec<Dataset>,
    /// The event times, measured since collection start
    event_times: Vec<Dataset>,
    /// The cue IDs containing cue type and possible metadata
    cue_ids: Vec<Dataset>,
    /// The cue times, measured since collection start
    cue_times: Vec<Dataset>,
}

impl Module {
    /// Sets up a new detector [`Module`] with a collection of round-robined data [`File`]s
    pub fn new(files: Vec<File>) -> Result<Self, hdf5::Error> {
        let event_ids = files
            .iter()
            .map(|file| file.dataset("event_id"))
            .collect::<Result<Vec<_>, _>>()?;
        let event_energies = files
            .iter()
            .map(|file| file.dataset("event_energy"))
            .collect::<Result<Vec<_>, _>>()?;
        let event_times = files
            .iter()
            .map(|file| file.dataset("event_time_offset"))
            .collect::<Result<Vec<_>, _>>()?;
        let cue_ids = files
            .iter()
            .map(|file| file.dataset("cue_id"))
            .collect::<Result<Vec<_>, _>>()?;
        let cue_times = files
            .iter()
            .map(|file| file.dataset("cue_timestamp_zero"))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Module {
            files,
            event_ids,
            event_energies,
            event_times,
            cue_ids,
            cue_times,
        })
    }

    /// Provides access to the undereling [`File`]s
    pub fn files(&self) -> &[File] {
        &self.files
    }
}
