# N0

N0 is an experimental minimal server built from two components:

- **Node.js** — raw TCP networking through the built-in `node:net` module.
- **Rust** — native byte-processing core exposed to Node.js as a shared library.

## Initial Goal

Build the smallest working data path:

```text
TCP client
    → Node.js Buffer
    → Rust shared library
    → processed bytes
    → Node.js
    → TCP client
