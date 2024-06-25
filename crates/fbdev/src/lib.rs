#[cfg(not(target_os = "linux"))]
compile_error!("fbdev is a Linux API");

mod sys;

pub mod prelude {}
