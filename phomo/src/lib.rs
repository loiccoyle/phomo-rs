#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc =include_str!("../README.md")]
mod color_match;
mod distance_matrix;
mod macros;
mod master;
mod mosaic;

pub mod error;
pub mod metrics;
pub mod solvers;
pub mod utils;

#[cfg_attr(docsrs, doc(cfg(feature = "blueprint")))]
#[cfg(feature = "blueprint")]
mod blueprint;
#[cfg_attr(docsrs, doc(cfg(feature = "blueprint")))]
#[cfg(feature = "blueprint")]
pub use blueprint::Blueprint;

pub use color_match::ColorMatch;
pub use distance_matrix::DistanceMatrix;
pub use master::Master;
pub use metrics::{norm_l1, norm_l2};
pub use mosaic::Mosaic;
pub use solvers::{greedy::Greedy, hungarian::Hungarian, Solve, SolverConfig};
pub use utils::{read_images_from_dir, read_images_from_dir_cropped, read_images_from_dir_resized};
