import { ASSETS } from "./config.js";

const UNSUPPORTED = { os: "unsupported", asset: null, label: "BabyBoop runs on Mac and Windows computers" };

export function pickDownload({ platform, userAgent = "", maxTouchPoints = 0, architecture }) {
  const p = (platform || "").toLowerCase();
  const ua = userAgent.toLowerCase();
  if (/android|iphone|ipad|cros/.test(ua)) return UNSUPPORTED;
  if (p === "macos" || (!p && ua.includes("macintosh"))) {
    if (maxTouchPoints > 1) return UNSUPPORTED;
    return { os: "mac", asset: ASSETS.mac, label: "Download for Mac (Intel & Apple silicon)" };
  }
  if (p === "windows" || (!p && ua.includes("windows"))) {
    return architecture === "arm"
      ? { os: "windows", asset: ASSETS.winArm64, label: "Download for Windows (ARM)" }
      : { os: "windows", asset: ASSETS.winX64, label: "Download for Windows" };
  }
  return UNSUPPORTED;
}
