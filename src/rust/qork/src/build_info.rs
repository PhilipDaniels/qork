use target_info;
use chrono::*;

// Information about the build.
// This information is derived at compile time.

// The file has been placed there by the build.rs script and the 'built' crate.
// Import it privately so we can rename the constants to avoid too deep a nesting.
// TODO: Is there a better way of doing this?
mod bi {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[doc="The name of the program."]
pub const PKG_NAME: &'static str = bi::PKG_NAME;

#[doc="The description."]
pub const PKG_DESCRIPTION: &'static str = bi::PKG_DESCRIPTION;

#[doc="The full version."]
pub const PKG_VERSION: &'static str = bi::PKG_VERSION;

#[doc="The major version."]
pub const PKG_VERSION_MAJOR: &'static str = bi::PKG_VERSION_MAJOR;

#[doc="The minor version."]
pub const PKG_VERSION_MINOR: &'static str = bi::PKG_VERSION_MINOR;

#[doc="The patch version."]
pub const PKG_VERSION_PATCH: &'static str = bi::PKG_VERSION_PATCH;

#[doc="The pre-release version."]
pub const PKG_VERSION_PRE: &'static str = bi::PKG_VERSION_PRE;

#[doc="A colon-separated list of authors."]
pub const PKG_AUTHORS: &'static str = bi::PKG_AUTHORS;

#[doc="The homepage."]
pub const PKG_HOMEPAGE: &'static str = bi::PKG_HOMEPAGE;

#[doc="`release` for release builds, `debug` for other builds."]
pub const PROFILE: &'static str = bi::PROFILE;

#[doc="Value of DEBUG for the profile used during compilation."]
pub const DEBUG: bool = bi::DEBUG;

#[doc="Value of OPT_LEVEL for the profile used during compilation."]
pub const OPT_LEVEL: u8 = bi::OPT_LEVEL;

#[doc="The compiler that cargo resolved to use."]
pub const RUSTC: &'static str = bi::RUSTC;

#[doc="The output of `rustc -V`"]
pub const RUSTC_VERSION: &'static str = bi::RUSTC_VERSION;

#[doc="The output of `rustdoc -V`"]
pub const RUSTDOC_VERSION: &'static str = bi::RUSTDOC_VERSION;

#[doc="The documentation generator that cargo resolved to use."]
pub const RUSTDOC: &'static str = bi::RUSTDOC;

#[doc="The features that were enabled during compilation."]
pub const FEATURES: [&'static str; 0] = bi::FEATURES;

#[doc="The features as a comma-separated string."]
pub const FEATURES_STR: &'static str = bi::FEATURES_STR;

#[doc="The built-time in RFC822, UTC."]
pub const BUILD_TIME_UTC: &'static str = bi::BUILT_TIME_UTC;

#[doc="Return BUILD_TIME_UTC as a chrono DateTime<FixedOffset>."]
pub fn build_time() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc2822(BUILD_TIME_UTC).unwrap()
}

// TODO: These should be compile-time constants too. However, the target_info crate
// only exposes them as functions.
pub fn arch() -> &'static str {
    target_info::Target::arch()
}

pub fn endian() -> &'static str {
    target_info::Target::endian()
}

pub fn env() -> &'static str {
    target_info::Target::env()
}

pub fn family() -> &'static str {
    target_info::Target::family()
}

pub fn os() -> &'static str {
    target_info::Target::os()
}


pub fn log_build_info() {
    info!(r#"BuildInfo {{ PKG_VERSION: "{}", PROFILE: "{}", DEBUG: "{}", OPT_LEVEL: "{}", RUSTC: "{}", RUSTC_VERSION: "{}", FEATURES_STR: "{}", BUILD_TIME_UTC: "{}", arch: "{}", endian: "{}", env: "{}", family: "{}", os: "{}" }}"#,
        PKG_VERSION, PROFILE, DEBUG, OPT_LEVEL, RUSTC, RUSTC_VERSION, FEATURES_STR, BUILD_TIME_UTC,
        arch(), endian(), env(), family(), os()
        );
}


macro_rules! return_cfg {
	($i:ident : $s:expr) => ( if cfg!($i = $s) { return $s; } );
	($i:ident : $s:expr, $($t:expr),+) => ( return_cfg!($i: $s); return_cfg!($i: $($t),+) );
}

/*
pub const ARCH: &'static str = {
		return_cfg!(target_arch: "x86", "x86_64", "mips", "powerpc", "arm", "aarch64");
		"unknown"
	};

    // e.g. x86, x86_64, mips...
    pub arch: &'static str,
    // little or big.
    pub endian: &'static str,
    // e.g. gnu, msvc, musl.
    pub env: &'static str,
    // e.g. unix, windows.
    pub family: &'static str,
    // e.g. linux, windows, macos, ios, android.
    pub os: &'static str

*/
