Phase 12: Build, test, and deploy all Phase 7-11 features to GitHub Pages.

1. Run cargo clippy --all-targets --all-features -- -D warnings and fix any warnings.
2. Run cargo fmt --all to ensure formatting.
3. Test the dev build with ./scripts/serve.sh -- verify all new demos run, prettification works, history works, scroll lock works, help overlay opens/closes.
4. Run ./scripts/build-pages.sh to create the release build.
5. Commit all changes and push to deploy to GitHub Pages.