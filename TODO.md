# Current Tasks

## Monitor UI Repaint Fix
- [x] In `src/app.rs`, add `ctx.request_repaint_after(std::time::Duration::from_millis(100))` unconditionally at the end of the `ui` function so `try_recv()` is polled regularly.