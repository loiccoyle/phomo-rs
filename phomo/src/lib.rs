#![doc =include_str!("../README.md")]
#[cfg(feature = "blueprint")]
mod blueprint;
mod color_match;
mod distance_matrix;
mod error;
mod macros;
mod master;
pub mod metrics;
mod mosaic;
pub mod utils;

#[cfg(feature = "blueprint")]
pub use blueprint::Blueprint;
pub use color_match::ColorMatch;
pub use distance_matrix::DistanceMatrix;
pub use master::Master;
pub use metrics::{norm_l1, norm_l2};
pub use mosaic::Mosaic;
pub use utils::{read_images_from_dir, read_images_from_dir_cropped, read_images_from_dir_resized};
