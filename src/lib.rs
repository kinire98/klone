mod app;

/// Error struct and enum for the different error messages the app shows
pub mod error;

mod sys;

/// Establishes different functions that tell if a file should be excluded from the backup
/// or the defaults in case no path was specified
pub mod config;

mod output;

/// The function that starts the backup of the code
pub use app::backup;

/// The function that states if an element shall be copied
pub use sys::should_be_backed;
