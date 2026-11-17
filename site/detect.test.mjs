import { test } from "node:test";
import assert from "node:assert/strict";
import { pickDownload } from "./detect.js";

const MAC_UA = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15";
const WIN_UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0 Safari/537.36";

test("macDesktop", () => {
  const r = pickDownload({ platform: "macOS", userAgent: MAC_UA, maxTouchPoints: 0 });
  assert.equal(r.os, "mac");
  assert.equal(r.asset, "BabyBoop-macOS.dmg");
});

test("macFromUAOnly", () => {
  const r = pickDownload({ platform: undefined, userAgent: MAC_UA, maxTouchPoints: 0 });
  assert.equal(r.os, "mac");
  assert.equal(r.asset, "BabyBoop-macOS.dmg");
});

test("ipadIsUnsupported", () => {
  const r = pickDownload({ platform: undefined, userAgent: MAC_UA, maxTouchPoints: 5 });
  assert.equal(r.os, "unsupported");
  assert.equal(r.asset, null);
});

test("windowsArm", () => {
  const r = pickDownload({ platform: "Windows", userAgent: WIN_UA, maxTouchPoints: 0, architecture: "arm" });
  assert.equal(r.os, "windows");
  assert.equal(r.asset, "BabyBoop-Windows-arm64.exe");
});

test("windowsDefaultsToX64", () => {
  const r = pickDownload({ platform: "Windows", userAgent: WIN_UA, maxTouchPoints: 0, architecture: undefined });
  assert.equal(r.os, "windows");
  assert.equal(r.asset, "BabyBoop-Windows-x64.exe");
});

test("linuxUnsupported", () => {
  const r = pickDownload({ platform: "Linux", userAgent: "Mozilla/5.0 (X11; Linux x86_64)", maxTouchPoints: 0 });
  assert.equal(r.os, "unsupported");
});

test("androidUnsupported", () => {
  const r = pickDownload({ platform: undefined, userAgent: "Mozilla/5.0 (Linux; Android 14; Pixel 8)", maxTouchPoints: 5 });
  assert.equal(r.os, "unsupported");
});

test("chromeOSUnsupported", () => {
  const r = pickDownload({ platform: undefined, userAgent: "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0)", maxTouchPoints: 0 });
  assert.equal(r.os, "unsupported");
});
