#[cfg(target_os = "linux")]
pub const IS_LINUX: bool = true;

#[cfg(not(target_os = "linux"))]
pub const IS_LINUX: bool = false;