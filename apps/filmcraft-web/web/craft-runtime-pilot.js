// Optional smoke: ?craftRuntime=1 connects SharedWorker at /craft-worker-host.js
import { connect } from "./craft-runtime-loader.js";

function banner(text, ok) {
  const el = document.getElementById("loading");
  if (!el) return;
  const span = el.querySelector(".err") || el;
  if (span.classList?.contains("err")) {
    span.textContent = text;
    span.className = ok ? "ok" : "err";
  } else {
    el.insertAdjacentHTML(
      "beforeend",
      `<p class="${ok ? "ok" : "err"}" id="craft-runtime-status">${text}</p>`,
    );
  }
}

export async function runCraftRuntimePilot() {
  banner("craft-runtime: connecting SharedWorker…", true);
  const rt = connect();
  const ready = await rt.init({
    app_id: "filmcraft",
    capabilities: ["gpu", "tier_store"],
  });
  await rt.storageFlush();
  banner(`craft-runtime ready (worker /craft-worker-host.js)`, true);
  return ready;
}

if (new URLSearchParams(location.search).has("craftRuntime")) {
  runCraftRuntimePilot().catch((e) => {
    banner(`craft-runtime failed: ${e?.message || e}`, false);
    console.error(e);
  });
}
