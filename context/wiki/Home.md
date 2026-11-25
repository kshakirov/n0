# N0

[Русский](Home-RU.md)

N0 is my experimental byte-processing server: Node.js owns TCP networking and event dispatch; Rust parses HTTP through N-API. I develop it in small, testable steps and measure changes before drawing performance conclusions.

## Architecture

The implementation has a pure Rust core, a Node addon, and a standalone Rust runner. Input buffers are read-only. Fixed-length and chunked bodies are processed incrementally, with bulk payload copying into a fresh output arena for each call. The application decides whether to retain body portions.

Request phase and body-automaton state are separate and preserved across calls. The parser is experimental; the current tests do not establish complete HTTP compliance.

## Engineering Decisions

I keep the authoritative ADRs in the repository, with matching English and Russian versions and stable numbers.

| ADR | Decision | Status | Languages |
|---|---|---|---|
| 001 | Streaming HTTP parser optimization | Accepted | [EN](https://github.com/kshakirov/n0/blob/dev-optimized/context/decisions/ADR-001-http-parser-optimization.en.md) · [RU](https://github.com/kshakirov/n0/blob/dev-optimized/context/decisions/ADR-001-http-parser-optimization.ru.md) |
| 002 | Streaming residual-language router | Proposed | [EN](https://github.com/kshakirov/n0/blob/dev-optimized/context/decisions/ADR-002-residual-language-router.en.md) · [RU](https://github.com/kshakirov/n0/blob/dev-optimized/context/decisions/ADR-002-residual-language-router.ru.md) |

## Research and current work

I am prototyping route compilation: preserving literal bytes, replacing parameter placeholders with reserved capture regions, and preparing byte-level metadata. Matching and candidate elimination are not implemented yet. The intended matcher narrows independent working route copies as URI bytes arrive and returns compatible full matches with their original RouteIDs.

Further directions include header scanning with `memchr` and routing during URI parsing. These remain research directions, not implemented capabilities.

## Experiments and history

- [First contact with Node-API — original Russian notes](Node-API-first-contact.md).
- [Issue #3 — chunked body parser](https://github.com/kshakirov/n0/issues/3).
- [Issue #4 — parser optimization and measurements](https://github.com/kshakirov/n0/issues/4).

Single-run POST measurements with 256 KiB bodies reached approximately 14.4k requests/s for chunked and 14.6k for fixed bodies. These are preliminary results from different runs, not a controlled comparison or a general performance guarantee. Configuration and qualifications are recorded in ADR-001 and issue #4.

The repository code shows what works today; the documentation records why I chose a design and what each experiment actually demonstrated.
