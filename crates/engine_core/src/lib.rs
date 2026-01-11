mod app;
mod config;
mod fs;
mod time; // filesystem helper

// Re-export what games should see:
pub use app::run;
pub use config::EngineConfig;
pub use time::Time;

pub mod prelude {
    pub use crate::Time;
    pub use crate::run;
}
