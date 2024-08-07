//! Crate to open Vdfs Files and access the different entries.
//!
//! The given example loads a wave audio file out of an archive.
//! ```rust
//! use std::{fs::File, io::Write};
//! use zen_vdfs::VdfsArchive;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//! let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Sounds.vdf")?;
//! let vdf = VdfsArchive::new(vdf_file)?;
//!
//! let register = vdf
//!     .get("CHAPTER_01.WAV")
//!     .expect("Should be there!");
//!
//! let
//!
//! let mut audio_file = File::create("/home/tom/Git/zen-loader/files/audio/chapter_01.wav")?;
//! audio_file.write(&entry.data)?;
//! # Ok(())
//! # }
//! ```

mod archive;
mod entry;
mod header;
#[cfg(feature = "bevy")]
mod plugin;

pub mod error;

pub use archive::VdfsArchive;
pub use entry::VdfsEntry;
#[cfg(feature = "bevy")]
pub use plugin::VdfsPlugin;
