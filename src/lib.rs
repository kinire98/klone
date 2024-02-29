mod app;

pub mod error;

mod sys;

mod config_files;

pub mod config;

mod output;

pub use app::backup;

pub use sys::should_be_backed;

pub use config_files::generate_file_structure;
