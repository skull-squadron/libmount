#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
extern crate libmount;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
extern crate argparse;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
extern crate env_logger;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
#[macro_use] extern crate log;

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
use std::path::PathBuf;
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
use std::process::exit;

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
use argparse::{ArgumentParser, Parse, StoreFalse, StoreTrue};

#[cfg(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos"))]
fn main() {
}

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "watchos", target_os = "tvos", target_os = "visionos")))]
fn main() {
    env_logger::init();
    let mut source = PathBuf::new();
    let mut target = PathBuf::new();
    let mut recursive = true;
    let mut readonly = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Bind mounting utility. Similar to `mount --bind`");
        ap.refer(&mut source).add_argument("source", Parse,
            "Source directory for bind mount").required();
        ap.refer(&mut target).add_argument("target", Parse,
            "Target directory for bind mount").required();
        ap.refer(&mut recursive).add_option(&["--non-recursive"], StoreFalse,
            "Disable recursive mount (only a real superuser can do this)");
        ap.refer(&mut readonly).add_option(&["--readonly"], StoreTrue,
            "Readonly mount");
        ap.parse_args_or_exit();
    }
    match libmount::BindMount::new(source, target)
        .recursive(recursive)
        .readonly(readonly)
        .mount()
    {
        Ok(()) => {}
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    }
}
