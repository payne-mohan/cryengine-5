//! Static build info for Cryengine 5.

pub const BUILD_VERSION: &str  = "v2.0.24";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "e9d2ccfd2988";
pub const BUILD_TIMESTAMP: u64 = 1707389889;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Cryengine 5", BUILD_VERSION, BUILD_CHANNEL)
}
