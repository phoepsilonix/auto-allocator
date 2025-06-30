fn main() {
    // Only apply embedded-specific linking for no_std targets
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "none" {
        println!("cargo:rerun-if-changed=memory.x");
        println!("cargo:rustc-link-arg=-Tmemory.x");
    }
}