export const REPO = "alok722/babyboop";
export const ASSETS = {
  mac: "BabyBoop-macOS.dmg",
  winX64: "BabyBoop-Windows-x64.exe",
  winArm64: "BabyBoop-Windows-arm64.exe",
};
export const downloadUrl = asset => `https://github.com/${REPO}/releases/latest/download/${asset}`;
