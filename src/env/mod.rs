use std::{ffi::OsStr, fmt::Debug, str::FromStr};

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

pub fn get_or<K: AsRef<OsStr> + Debug>(key: K, default: String) -> String {
    std::env::var(&key)
        .inspect_err(|e| tracing::info!("{e}: {key:?}, defaulting to {default:?}"))
        .unwrap_or(default)
}

pub fn get<K: AsRef<OsStr> + Debug>(key: K) -> String {
    std::env::var(&key)
        .inspect_err(|e| tracing::error!("{e}: {key:?}"))
        .expect(&format!("env variable {key:?} must be set"))
}
