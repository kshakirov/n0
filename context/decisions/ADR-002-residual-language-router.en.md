# ADR-002 — Streaming residual-language router

[Русский](ADR-002-residual-language-router.ru.md) · [All decisions](README.md)

Project: N0. Status: **Proposed — selected for experimental implementation**.
Date: 2026-11-23. Component: Rust URI Router. Branch: `dev-optimized`.

## Context

I want to select routes while reading the URI sequentially, starting with a single byte. I retain all compatible alternatives, eliminate incompatible routes, support variable-length parameters, and avoid rescanning the URI. I aim to minimize temporary structures and allocations, but their cost still needs measurement.

I considered Radix/Trie, hashing, Brzozowski derivatives, and Boolean matrix algebra. For the first experiment I choose a custom residual-language representation. Neither code nor layout has been validated; this is not a production-readiness decision.

## Mathematical basis

For route language L and consumed prefix p:

```text
D_p(L) = { w | pw ∈ L }
```

This is the set of valid continuations. I represent it through active routes and their working states, without directly computing classical regular-expression derivatives.

## Experimental decision

### Compilation and identity

JS defines routes and retains handler functions. Rust receives a byte representation and returns the original RouteID; JS invokes the corresponding function. Conceptually, parallel arrays associate routes and handlers:

```rust
routes: Vec<Vec<u8>>
handlers: Vec<Handler>
```

`Handler` denotes a JS handler reference here, not an established Rust type. If necessary I sort routes once during compilation, preserving original RouteIDs independently of sorted positions.

### Parameters

I compile a parameter into a dedicated route region, initially 128 bytes, filled with the `{` marker. I retain its name and region boundaries. `/` is the designated segment delimiter. Literal `{`, encoded delimiters, and the precise layout remain unspecified.

### Working set and matching

Each request gets independent mutable candidates; compiled routes remain immutable:

```rust
struct Candidate {
    route_id: usize,
    bytes: Vec<u8>,
}

active.retain_mut(|candidate| step(candidate, byte));
```

This is a sketch, not a complete state layout. For each byte, `step` checks the expected position: a literal match retains a candidate, a mismatch removes it, and a parameter region accepts the byte. A `/` finishes the parameter and switches addressing to the next segment. The prototype will define the required indices and metadata.

### Addressing

The proposed access is `route[base + offset]`: offset tracks input progress, and base switches between compiled regions without moving the suffix. Parameter metadata is expected to occupy a fixed region describing the next route section. Its layout and base-switch mechanism are experimental.

### Completion and result

Prefix compatibility does not establish a full match. At the end of the URI I check whole-path acceptance, then resolve surviving alternatives. Results contain original RouteIDs, working state, and captured parameters. Static-versus-parameter route priorities will be defined separately.

## Invariant

After n bytes, the working set contains all and only routes compatible with the consumed prefix:

```text
Active_(n+1) = Step(Active_n, b_n)
```

I check this invariant in the executable program by comparing candidates against prefix validity, rather than checking only the final answer. Since `retain_mut` cannot restore removed alternatives, premature elimination is invalid.

## Prototype checks

Initial routes:

```text
/users/{id}/name
/update/name
/users/add/
/mail/user/{id}/reset
```

I will check literal matches and eliminations, simultaneous static and parameter alternatives, different parameter lengths and multiple parameters, empty and oversized values, URI termination within a parameter, bytewise input, and arbitrary fragmentation. Checks include array boundaries, the invariant at every prefix, and stable RouteID-to-handler identity.

## Open questions

- Parameter prefix/metadata format and base switching.
- Overflow of the 128-byte region: no policy selected yet.
- Resolution of multiple full matches and separation of static/parameter routes.
- Cost of copying working routes and possibilities for memory reuse.
- URI/query-string boundaries, encoding, and HTTP method matching.

## Alternatives

Radix/Trie is not selected for the first experiment. Hashing alone does not define streaming parameter matching. Brzozowski derivatives provide the mathematical idea, not the literal data representation. Boolean matrix algebra remains a separate research direction for Haskell Proxy.

## Principle

Routing progressively reduces the set of valid URI continuations. The program implements the transformation and checks its invariants. Selecting this experiment does not establish correctness or a performance advantage.
