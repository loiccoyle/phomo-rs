#[cfg(feature = "progress_bar")]
extern crate indicatif;

#[cfg(feature = "progress_bar")]
pub(crate) fn progress_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::with_template("{msg}: {wide_bar} {elapsed_precise} {pos}/{len}")
        .unwrap()
}

macro_rules! iter_or_par_iter {
    ($iter:expr) => {{
        #[cfg(feature = "parallel")]
        {
            $iter.par_iter()
        }
        #[cfg(not(feature = "parallel"))]
        {
            $iter.iter()
        }
    }};
}

macro_rules! maybe_progress_bar {
    ($iter:expr, $msg:expr, par) => {{
        #[cfg(feature = "progress_bar")]
        {
            indicatif::ParallelProgressIterator::progress($iter)
                .with_message($msg)
                .with_style(crate::macros::progress_style())
        }
        #[cfg(not(feature = "progress_bar"))]
        {
            $iter
        }
    }};
    ($iter:expr, $msg:expr) => {{
        #[cfg(feature = "progress_bar")]
        {
            indicatif::ProgressIterator::progress($iter)
                .with_message($msg)
                .with_style(crate::macros::progress_style())
        }
        #[cfg(not(feature = "progress_bar"))]
        {
            $iter
        }
    }};
}

pub(crate) use iter_or_par_iter;
pub(crate) use maybe_progress_bar;
