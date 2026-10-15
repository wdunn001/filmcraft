//! Parallel path for SharedWorker `craft-runtime` messages (pilot; does not replace OPFS/recovery yet).

use craft_runtime::{HostMessage, Runtime, WorkerMessage};
use serde_json;

pub struct CraftRuntimeBridge {
    runtime: Runtime,
}

impl Default for CraftRuntimeBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl CraftRuntimeBridge {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::builder().with_default_providers().null_context().build(),
        }
    }

    /// JSON host message in, JSON worker replies out (one round-trip).
    pub fn handle_worker_json(&mut self, msg: &str) -> String {
        let host = match serde_json::from_str::<HostMessage>(msg) {
            Ok(h) => h,
            Err(e) => {
                return json_replies(&[WorkerMessage::Error { message: e.to_string() }]);
            }
        };
        let replies = self.runtime.handle_host_message(host);
        json_replies(&replies)
    }
}

fn json_replies(replies: &[WorkerMessage]) -> String {
    serde_json::to_string(replies).unwrap_or_else(|_| "[]".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_storage_flush_and_noop_gpu_roundtrip() {
        let mut bridge = CraftRuntimeBridge::new();
        let ready = bridge.handle_worker_json(r#"{"type":"init","app_id":"filmcraft","capabilities":["gpu"]}"#);
        assert!(ready.contains("ready"));
        bridge.handle_worker_json(r#"{"type":"storage_put","key":"config/session.json","bytes":[123,125]}"#);
        let flushed = bridge.handle_worker_json(r#"{"type":"storage_flush"}"#);
        assert!(flushed.contains("storage_flushed"));
        let gpu = bridge.handle_worker_json(
            r#"{"type":"gpu_jobs","batch_id":1,"jobs":[{"provider_id":"craft.noop","params":[],"inputs":[],"outputs":[]}]}"#,
        );
        assert!(gpu.contains("gpu_batch_done"));
    }
}
