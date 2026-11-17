import { REPO, ASSETS, downloadUrl } from "./config.js";
import { pickDownload } from "./detect.js";

// Same letter → emoji map as the game (web/index.html).
const KEY_EMOJI = {
  A: "🍎", B: "🐻", C: "🐱", D: "🐶", E: "🐘", F: "🐸", G: "🦒", H: "🐴", I: "🍦", J: "🪼", K: "🪁", L: "🦁", M: "🐵",
  N: "🪺", O: "🐙", P: "🐷", Q: "👑", R: "🐰", S: "☀️", T: "🐯", U: "☂️", V: "🚐", W: "🐳", X: "🎶", Y: "🪀", Z: "🦓",
};
const COLORS = ["#ff5a36", "#ffc53d", "#2ec4a0", "#4f8cff", "#b06cff", "#ff6ec4"];

// Links
const repoUrl = `https://github.com/${REPO}`;
document.querySelectorAll("[data-repo]").forEach(a => { a.href = repoUrl; });
document.querySelectorAll("[data-repo-path]").forEach(a => { a.href = repoUrl + a.dataset.repoPath; });
document.querySelectorAll("[data-asset]").forEach(a => { a.href = downloadUrl(ASSETS[a.dataset.asset]); });

// Setup tabs
const tabs = [...document.querySelectorAll(".tab")];
function showOs(os) {
  for (const t of tabs) {
    const on = t.dataset.os === os;
    t.setAttribute("aria-selected", String(on));
    t.tabIndex = on ? 0 : -1;
    document.getElementById(t.getAttribute("aria-controls")).hidden = !on;
  }
}
tabs.forEach((t, i) => {
  t.addEventListener("click", () => showOs(t.dataset.os));
  t.addEventListener("keydown", e => {
    if (e.key !== "ArrowRight" && e.key !== "ArrowLeft") return;
    const next = tabs[(i + (e.key === "ArrowRight" ? 1 : tabs.length - 1)) % tabs.length];
    showOs(next.dataset.os);
    next.focus();
  });
});

// Primary download button picks the right file for this computer
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

const primary = document.getElementById("download");
const secondary = document.getElementById("download-2");
const note = document.getElementById("platform-note");
if (choice.asset) {
  primary.href = secondary.href = downloadUrl(choice.asset);
  primary.textContent = choice.label;
  showOs(choice.os);
} else {
  primary.href = "play/";
  primary.textContent = "Play in your browser";
  document.getElementById("try").hidden = true;
  note.textContent = "The locked app runs on Mac and Windows computers. On this device you can play in the browser, but keys and gestures can't be locked there.";
}

// Hero demo: keys and taps pop the same emoji the game shows
const stage = document.getElementById("stage");
const hint = document.getElementById("stage-hint");
const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;

function boop(letter) {
  hint.style.opacity = "0";
  const r = stage.getBoundingClientRect();
  const x = 15 + Math.random() * 70, y = 25 + Math.random() * 50;
  const pop = document.createElement("div");
  pop.className = "pop";
  pop.style.left = x + "%";
  pop.style.top = y + "%";
  pop.innerHTML = `<span class="e">${KEY_EMOJI[letter]}</span><span class="l">${letter}</span>`;
  stage.append(pop);
  setTimeout(() => pop.remove(), 1600);
  if (reduceMotion) return;
  for (let i = 0; i < 14; i++) {
    const c = document.createElement("span");
    c.className = "confetti";
    c.style.left = x + "%";
    c.style.top = y + "%";
    c.style.background = COLORS[i % COLORS.length];
    const a = Math.random() * Math.PI * 2, d = 60 + Math.random() * r.width * 0.25;
    c.style.setProperty("--dx", `${Math.cos(a) * d}px`);
    c.style.setProperty("--dy", `${Math.sin(a) * d + 40}px`);
    c.style.setProperty("--rot", `${Math.random() * 720 - 360}deg`);
    stage.append(c);
    setTimeout(() => c.remove(), 1100);
  }
}

document.querySelectorAll(".key").forEach(k => k.addEventListener("click", () => boop(k.dataset.k)));

const capFor = letter => [...document.querySelectorAll(".key")].find(k => k.dataset.k === letter);
addEventListener("keydown", e => {
  if (e.metaKey || e.ctrlKey || e.altKey || e.repeat) return;
  if (e.target.closest?.("input, textarea, select, [contenteditable]")) return;
  const letter = e.key.length === 1 ? e.key.toUpperCase() : "";
  if (!KEY_EMOJI[letter]) return;
  boop(letter);
  const cap = capFor(letter);
  if (cap) { cap.classList.add("down"); setTimeout(() => cap.classList.remove("down"), 120); }
});
