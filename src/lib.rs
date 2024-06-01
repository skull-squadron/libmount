//! # libmount
//!
//! [Documentation](https://docs.rs/libmount) |
//! [Github](https://github.com/tailhook/libmount) |
//! [Crate](https://crates.io/crates/libmount)
//!
//! This library has two major goals:
//!
//! 1. Add type-safe interface for mount() system call
//! 2. Add very good explanation of what's wrong when the call fails
//!
//! So we have two error types:
//!
//! 1. `OSError` holds mount info and errno
//! 2. `Error` is returned by `OSError::explain()`
//!
//! The first one is returned by `bare_mount()` the second by `mount()`, and
//! using latter is preffered for most situations. Unless performance is
//! too critical (i.e. you are doing thousands of *failing* mounts per second).
//! On the success path there is no overhead.
//!
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

extern crate libc;
extern crate nix;
#[macro_use] extern crate quick_error;

mod util;
mod error;
mod explain;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
mod bind;
mod overlay;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
mod tmpfs;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
mod modify;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
mod remount;
pub mod mountinfo;

use std::io;

use explain::Explainable;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
use remount::RemountError;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
pub use bind::BindMount;
pub use overlay::Overlay;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
pub use tmpfs::Tmpfs;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
pub use modify::Move;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
pub use remount::Remount;

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
quick_error! {
    #[derive(Debug)]
    enum MountError {
        Io(err: io::Error) {
            cause(err)
            from()
        }
        Remount(err: RemountError) {
            cause(err)
            from()
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos"))]
quick_error! {
    #[derive(Debug)]
    enum MountError {
        Io(err: io::Error) {
            cause(err)
            from()
        }
    }
}

/// The raw os error
///
/// This is a wrapper around `io::Error` providing `explain()` method
///
/// Note: you need to explain as fast as possible, because during explain
/// library makes some probes for different things in filesystem, and if
/// anything changes it may give incorrect results.
///
/// You should always `explain()` the errors, unless you are trying lots of
/// mounts for bruteforcing or other similar thing and you are concerned of
/// performance. Usually library does `stat()` and similar things which are
/// much faster than mount anyway. Also explaining is zero-cost in the success
/// path.
///
#[derive(Debug)]
pub struct OSError(MountError, Box<dyn Explainable>);

impl OSError {
    #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
    fn from_remount(err: RemountError, explain: Box<dyn Explainable>) -> OSError {
        OSError(MountError::Remount(err), explain)
    }

    fn from_nix(err: nix::Error, explain: Box<dyn Explainable>) -> OSError {
        OSError(
            MountError::Io(
                io::Error::from(err),
            ),
            explain,
        )
    }
}

/// The error holder which contains as much information about why failure
/// happens as the library implementors could gain
///
/// This type only provides `Display` for now, but some programmatic interface
/// is expected in future.
#[derive(Debug)]
pub struct Error(Box<dyn Explainable>, io::Error, String);
