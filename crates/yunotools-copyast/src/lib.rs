mod binary;
mod command;
mod config;
mod detector;
mod domain;
mod duplicate;
mod error;
mod framework;
mod ignore;
mod ignore_generator;
mod ignore_template;
mod incremental;
mod path;
mod scanner;
mod service;
mod templates;
mod token;
mod writer;

pub use command::CopyastCommand;

pub use config::{CopyastConfig, DEFAULT_MAX_FILE_SIZE, PathMode, TokenModel};

pub use detector::{DetectedLanguage, Language, LanguageDetector};

pub use domain::{CopyastResult, DuplicateGroup, IncrementalStats, ScanStats, TextFile};

pub use duplicate::DuplicateDetector;

pub use error::CopyastError;

pub use framework::{DetectedFramework, Framework, FrameworkDetector};

pub use ignore_generator::IgnoreGenerator;

pub use ignore_template::IgnoreTemplate;

pub use incremental::IncrementalTracker;

pub use service::CopyastService;

pub use token::{TokenEstimate, TokenEstimator};
