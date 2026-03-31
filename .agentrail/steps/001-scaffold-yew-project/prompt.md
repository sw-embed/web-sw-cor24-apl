Scaffold the Yew/Rust/WASM project for web-sw-cor24-apl. Follow the exact patterns from web-sw-cor24-pcode.

1. Create Cargo.toml:
   - edition = "2024", name = "web-sw-cor24-apl"
   - [lib] crate-type = ["cdylib", "rlib"]
   - Dependencies: yew 0.21 (features=["csr"]), wasm-bindgen, web-sys (features: HtmlInputElement, HtmlDivElement, KeyboardEvent, Element, Window, Document), gloo (features: timers, utils), console_error_panic_hook
   - Path dep: cor24-emulator = { path = "../sw-cor24-emulator" }, cor24-isa = { path = "../sw-cor24-emulator/isa" }
   - [build-dependencies]: cor24-emulator = { path = "../sw-cor24-emulator" }

2. Create index.html with Trunk directives, Catppuccin Mocha theme CSS, layout with header ("APL" title, "COR24 Environment" subtitle), main content area, footer with build metadata (git SHA, hostname, timestamp from env! macros), GitHub corner link to https://github.com/sw-embed/web-sw-cor24-apl

3. Create src/main.rs: entry point with Yew renderer::Renderer + console_error_panic_hook::set_once()

4. Create src/lib.rs: App function_component with basic html! rendering header, main placeholder, footer with build metadata

5. Create src/app.css: Catppuccin Mocha theme variables and base styles (same pattern as web-sw-cor24-pcode)

6. Create build.rs: generate git SHA, hostname, timestamp as cargo env vars

7. Create scripts/serve.sh: trunk serve --port 9957 --open

8. Create scripts/build-pages.sh: trunk build --release --public-url /web-sw-cor24-apl/ && rsync to pages/

9. Create pages/ directory with .nojekyll file

10. Verify trunk build compiles successfully.

Reference: Study ~/github/sw-embed/web-sw-cor24-pcode/ for exact patterns (Cargo.toml, index.html, src/lib.rs, src/main.rs, build.rs, scripts/, app.css).