#![forbid(unsafe_code)]
#![warn(
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub mod execute;
pub mod human_time;
pub mod timestamp;
