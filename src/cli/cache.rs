use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub struct Cache {
    dir: Option<PathBuf>,
    refresh: bool,
}

fn safe_name(key: &str) -> String {
    key.chars()
        .map(|c| match c {
            '/' => '~',
            c if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') => c,
            _ => '_',
        })
        .collect()
}

impl Cache {
    pub fn new(refresh: bool) -> Self {
        let dir = dirs::cache_dir().map(|d| d.join("trustpkg"));
        Self::with_dir(dir, refresh)
    }

    pub fn with_dir(dir: Option<PathBuf>, refresh: bool) -> Self {
        Self { dir, refresh }
    }

    pub fn get(&self, key: &str, ttl: Duration) -> Option<String> {
        if self.refresh {
            return None;
        }
        let path = self.dir.as_ref()?.join(safe_name(key));
        let age = fs::metadata(&path).ok()?.modified().ok()?.elapsed().ok()?;
        if age >= ttl {
            return None;
        }
        fs::read_to_string(path).ok()
    }

    pub fn put(&self, key: &str, body: &str) {
        let Some(dir) = self.dir.as_ref() else { return };
        if fs::create_dir_all(dir).is_err() {
            return;
        }
        let name = safe_name(key);
        let path = dir.join(&name);

        let tmp = dir.join(format!("{name}.tmp{}", std::process::id()));
        if fs::write(&tmp, body).is_err() {
            let _ = fs::remove_file(&tmp);
            return;
        }
        if fs::rename(&tmp, &path).is_err() {
            let _ = fs::remove_file(&tmp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_cache(tag: &str, refresh: bool) -> (Cache, PathBuf) {
        let dir = std::env::temp_dir().join(format!("trustpkg-test-{}-{tag}", std::process::id()));
        (Cache::with_dir(Some(dir.clone()), refresh), dir)
    }

    #[test]
    fn put_then_get_returns_body() {
        let (cache, dir) = temp_cache("roundtrip", false);
        cache.put("crates.io/serde", "{\"a\":1}");
        assert_eq!(cache.get("crates.io/serde", Duration::from_secs(60)), Some("{\"a\":1}".to_string()));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn zero_ttl_is_always_expired() {
        let (cache, dir) = temp_cache("ttl", false);
        cache.put("k", "x");
        assert_eq!(cache.get("k", Duration::ZERO), None);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn refresh_skips_reads_but_still_writes() {
        let (cache, dir) = temp_cache("refresh", true);
        cache.put("k", "x");
        assert_eq!(cache.get("k", Duration::from_secs(60)), None);
        let normal = Cache::with_dir(Some(dir.clone()), false);
        assert_eq!(normal.get("k", Duration::from_secs(60)), Some("x".to_string()));
        let _ = fs::remove_dir_all(dir);
    }
}