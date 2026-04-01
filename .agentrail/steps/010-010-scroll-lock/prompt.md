Phase 10: Implement scroll-lock behavior for the REPL output area.

Currently the output area always auto-scrolls to the bottom on new output. This makes it impossible to read earlier output while the emulator is running.

In src/repl.rs, implement:

1. Add a boolean flag (e.g. user_scrolled_up) to the component state, default false.
2. On the output container's 'scroll' event, detect if the user has scrolled away from the bottom. If scrollTop + clientHeight < scrollHeight - threshold (e.g. 20px), set user_scrolled_up = true.
3. If user is near the bottom (within threshold), set user_scrolled_up = false.
4. In the auto-scroll logic (after new output), only scroll to bottom if user_scrolled_up is false.
5. Optionally add a small 'scroll to bottom' indicator/button that appears when user_scrolled_up is true, clicking it scrolls to bottom and resets the flag.

Use web-sys scroll APIs. Keep it simple -- no smooth scrolling animation needed for the indicator jump.