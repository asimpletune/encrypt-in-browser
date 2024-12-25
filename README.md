# Encrypt Messages in Browser

This README for https://github.com/asimpletune/encrypt-in-browser

- To see example, run a static server, e.g. `python3 -m http.server 8000`
- To clean run `npm run clean` (Do `cargo clean` if you want to remove rust dependencies)
- To build run `npm run build`
- To verify output run `gpg --decrypt <output-saved-to-file>`
- To prepare for importing into another project run `npm run build && npm pack`
- To verify contents of packaged file run `tar -tf <tarball>`