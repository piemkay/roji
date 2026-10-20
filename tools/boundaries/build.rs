// Records the target triple so the check can ask `cargo metadata` for this platform only.
fn main() {
    let target = std::env::var("TARGET").expect("cargo sets TARGET for build scripts");
    println!("cargo::rustc-env=BOUNDARIES_TARGET={target}");
}
