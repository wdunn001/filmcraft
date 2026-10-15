// Pilot: SharedWorker at CollabCraft web root (same protocol as effectcraft-web).
const CRAFT_HOST_TYPES = new Set([
  "init",
  "storage_put",
  "storage_flush",
  "gpu_jobs",
  "cancel",
]);

function isCraftHostMessage(data) {
  return data && typeof data.type === "string" && CRAFT_HOST_TYPES.has(data.type);
}

export function connect(opts = {}) {
  const metaUrl =
    typeof document !== "undefined"
      ? document.querySelector('meta[name="craft-runtime-worker"]')?.content
      : undefined;
  const workerUrl = opts.workerUrl ?? metaUrl ?? "/craft-worker-host.js";
  const worker = new SharedWorker(workerUrl, { name: "craft-runtime" });
  const port = worker.port;
  port.start();

  return {
    port,
    init(payload) {
      return new Promise((resolve, reject) => {
        const onReply = (ev) => {
          const msg = ev.data;
          if (msg?.type === "ready") {
            port.removeEventListener("message", onReply);
            resolve(msg);
          } else if (msg?.type === "error") {
            port.removeEventListener("message", onReply);
            reject(new Error(msg.message));
          }
        };
        port.addEventListener("message", onReply);
        port.postMessage({ type: "init", ...payload });
      });
    },
    storageFlush() {
      return new Promise((resolve, reject) => {
        const onReply = (ev) => {
          const msg = ev.data;
          if (msg?.type === "storage_flushed") {
            port.removeEventListener("message", onReply);
            resolve(msg);
          } else if (msg?.type === "error") {
            port.removeEventListener("message", onReply);
            reject(new Error(msg.message));
          }
        };
        port.addEventListener("message", onReply);
        port.postMessage({ type: "storage_flush" });
      });
    },
  };
}
