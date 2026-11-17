// Embeds the icon, manifest and version into BabyBoop.exe.
// The version comes from BABYBOOP_VERSION (set from the git tag by release.yml), else Cargo.toml.
fn main() {
    println!("cargo:rerun-if-env-changed=BABYBOOP_VERSION");
    println!("cargo:rerun-if-changed=app.manifest");
    println!("cargo:rerun-if-changed=assets/AppIcon.ico");
    println!("cargo:rerun-if-changed=../../web/index.html");

    let version = std::env::var("BABYBOOP_VERSION")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| std::env::var("CARGO_PKG_VERSION").unwrap());
    println!("cargo:rustc-env=BABYBOOP_VERSION={version}");

    // Resources need the Windows resource compiler, so only a Windows host embeds them.
    // `cargo check --target *-pc-windows-msvc` on macOS skips this step.
    let windows_target = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");
    if !(windows_target && cfg!(windows)) {
        return;
    }

    let numeric: Vec<u64> = version
        .split(['-', '+'])
        .next()
        .unwrap()
        .split('.')
        .map(|p| p.parse().unwrap_or(0))
        .collect();
    let packed = numeric
        .iter()
        .chain(std::iter::repeat(&0))
        .take(4)
        .fold(0u64, |acc, n| (acc << 16) | (n & 0xFFFF));

    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/AppIcon.ico")
        .set_manifest_file("app.manifest")
        .set("ProductName", "BabyBoop")
        .set("FileDescription", "BabyBoop")
        .set("ProductVersion", &version)
        .set("FileVersion", &version)
        .set_version_info(winresource::VersionInfo::PRODUCTVERSION, packed)
        .set_version_info(winresource::VersionInfo::FILEVERSION, packed);
    res.compile().expect("embedding Windows resources");
}
