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

// ---------------------------------------------------------------------------
// OPFS warm-tier sink (wasm32 only)
// ---------------------------------------------------------------------------

/// OPFS path root for the craft-runtime warm tier, matching `opfs-flush.js` in craft-libs.
#[cfg(target_arch = "wasm32")]
const OPFS_WARM_ROOT: &str = "effectcraft-store/v1";

/// [`craft_runtime::FlushSink`] that queues pending mirror writes to OPFS via `crate::opfs`.
///
/// Keys are mapped to `effectcraft-store/v1/<key>` (leading `/` stripped), mirroring the
/// `keyToSegments` logic in `craft-libs` `opfs-flush.js` (craft-libs#10).
#[cfg(target_arch = "wasm32")]
struct OpfsFlushSink;

#[cfg(target_arch = "wasm32")]
impl craft_runtime::FlushSink for OpfsFlushSink {
    fn write_all(&mut self, pending: &[craft_runtime::PendingWrite]) -> usize {
        let count = pending.len();
        for write in pending {
            let key = write.key.0.trim_start_matches('/');
            crate::opfs::write(format!("{OPFS_WARM_ROOT}/{key}"), write.bytes.clone());
        }
        count
    }
}

#[cfg(target_arch = "wasm32")]
impl CraftRuntimeBridge {
    /// Build a bridge whose `storage_flush` messages drain into OPFS (`effectcraft-store/v1/`).
    pub fn with_opfs_sink() -> Self {
        Self {
            runtime: Runtime::builder()
                .with_default_providers()
                .null_context()
                .flush_sink(Box::new(OpfsFlushSink))
                .build(),
        }
    }
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
