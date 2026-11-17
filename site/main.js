import { REPO, ASSETS, downloadUrl } from "./config.js";
import { pickDownload } from "./detect.js";

async function architecture() {
  try {
    const v = await navigator.userAgentData?.getHighEntropyValues(["architecture"]);
    return v?.architecture;
  } catch {
    return undefined;
  }
}

const choice = pickDownload({
  platform: navigator.userAgentData?.platform,
  userAgent: navigator.userAgent,
  maxTouchPoints: navigator.maxTouchPoints,
  architecture: await architecture(),
});

document.querySelectorAll("[data-asset]").forEach(a => { a.href = downloadUrl(ASSETS[a.dataset.asset]); });
document.getElementById("source").href = `https://github.com/${REPO}`;

if (choice.asset) {
  const btn = document.getElementById("download");
  btn.href = downloadUrl(choice.asset);
  btn.textContent = choice.label;
  document.getElementById(choice.os === "mac" ? "steps-mac" : "steps-windows").hidden = false;
} else {
  document.getElementById("primary").hidden = true;
  document.getElementById("unsupported").hidden = false;
}
