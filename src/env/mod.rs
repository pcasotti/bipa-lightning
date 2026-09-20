use std::{ffi::OsStr, fmt::Debug, str::FromStr};

/// Reads environment variables with a unified logging behaviour.
///
/// Missing or invalid variables are logged with [`tracing`]. Helpers with an
/// `_or` suffix fall back to a provided default, while `get` variants panic
/// when the variable is missing or cannot be parsed.
///
/// # Panics
///
/// [`get`] and [`get_from_str`] panic if the requested environment variable is
/// not set or cannot be parsed.

/// Reads an environment variable and parses it to `T`.
///
/// Falls back to `default` when the variable is missing or cannot be parsed.
/// Both cases are logged.
pub fn get_from_str_or<K, T>(key: K, default: T) -> T
where
    K: AsRef<OsStr> + Debug,
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    std::env::var(&key)
        .inspect_err(|e| tracing::info!("{e}: {key:?}, defaulting to {default:?}"))
        .ok()
        .and_then(|s| {
            s.parse::<T>()
                .inspect_err(|e| tracing::warn!("{e:?}: {key:?}: {s}, defaulting to {default:?}"))
                .ok()
        })
        .unwrap_or(default)
}

/// Reads an environment variable and parses it to `T`.
///
/// # Panics
///
/// Panics if the environment variable is not set or cannot be parsed as `T`.
/// Both cases are logged before panicking.
pub fn get_from_str<K, T>(key: K) -> T
where
    K: AsRef<OsStr> + Debug,
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let val = std::env::var(&key)
        .inspect_err(|e| tracing::error!("{e}: {key:?}"))
        .expect(&format!("env variable {key:?} must be set"));

    val.parse::<T>()
        .inspect_err(|e| tracing::error!("{e:?}: {key:?}: {val}"))
        .expect(&format!("{key:?}"))
}

/// Reads an environment variable as a [`String`].
///
/// Falls back to `default` when the variable is not set. The fallback is
/// logged.
pub fn get_or<K: AsRef<OsStr> + Debug>(key: K, default: String) -> String {
    std::env::var(&key)
        .inspect_err(|e| tracing::info!("{e}: {key:?}, defaulting to {default:?}"))
        .unwrap_or(default)
}

/// Reads an environment variable as a [`String`].
///
/// # Panics
///
/// Panics if the environment variable is not set. The failure is logged
/// before panicking.
pub fn get<K: AsRef<OsStr> + Debug>(key: K) -> String {
    std::env::var(&key)
        .inspect_err(|e| tracing::error!("{e}: {key:?}"))
        .expect(&format!("env variable {key:?} must be set"))
}
