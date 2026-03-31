use std::process::Command;

fn main() {
    // Pre-assemble the APL interpreter (COR24 assembly → machine code) at build time.
    let apl_source = std::fs::read_to_string("../sw-cor24-apl/build/apl.s")
        .expect("../sw-cor24-apl/build/apl.s");
    let mut asm = cor24_emulator::Assembler::new();
    let result = asm.assemble(&apl_source);
    if !result.errors.is_empty() {
        for e in &result.errors {
            eprintln!("apl.s: {e}");
        }
        panic!("apl.s assembly failed");
    }
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir);
    std::fs::write(out_path.join("apl.bin"), &result.bytes).unwrap();
    println!("cargo:rerun-if-changed=../sw-cor24-apl/build/apl.s");

    // Short git SHA
    let sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    // Build host
    let host = Command::new("hostname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    // ISO timestamp
    let timestamp = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    println!("cargo:rustc-env=BUILD_SHA={sha}");
    println!("cargo:rustc-env=BUILD_HOST={host}");
    println!("cargo:rustc-env=BUILD_TIMESTAMP={timestamp}");
}
