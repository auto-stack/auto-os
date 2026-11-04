//! Structured event log: one JSON line per event to stderr (and optionally a
//! file). Credential/token material is structurally excluded — callers pass
//! only non-secret labels, and no API accepts token values.

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct EventLog {
    file: Option<std::sync::Arc<std::sync::Mutex<std::fs::File>>>,
}

impl EventLog {
    pub fn stderr() -> Self {
        EventLog { file: None }
    }

    pub fn to_file(path: &std::path::Path) -> std::io::Result<Self> {
        let f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(EventLog {
            file: Some(std::sync::Arc::new(std::sync::Mutex::new(f))),
        })
    }

    pub fn event(&self, event: &str, fields: &[(&str, &str)]) {
        let mut obj = serde_json::Map::new();
        obj.insert(
            "ts_ms".into(),
            serde_json::Value::Number(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0)
                    .into(),
            ),
        );
        obj.insert("event".into(), serde_json::Value::String(event.into()));
        for (k, v) in fields {
            obj.insert((*k).into(), serde_json::Value::String((*v).into()));
        }
        let line = serde_json::Value::Object(obj).to_string();
        eprintln!("{line}");
        if let Some(f) = &self.file {
            if let Ok(mut f) = f.lock() {
                let _ = writeln!(f, "{line}");
            }
        }
    }
}
