# ADR-001 — Streaming HTTP parser optimization

[Русский](ADR-001-http-parser-optimization.ru.md) · [All decisions](README.md)

Project: N0. Status: **Accepted — implemented**. Recorded: 2026-11-23.
Component: Rust HTTP parser / N-API. Working branch: `dev-optimized`.

## Context

I use a streaming Rust HTTP parser integrated with Node.js through N-API. Node owns networking, input memory, and the output arena. Applications decide whether to accumulate the body.

The bytewise automaton makes transitions explicit and supports continuation across TCP fragments. However, executing a state step for every body byte was expensive, especially for chunked bodies. A CPU profile attributed about 64.6% of samples to `parse_http_body_step` and 21.1% to `parse_http_req`, including the inlined body loop. These are separate shares, not nested percentages or a measurement of header parsing.

## Decision

I retain the finite-state machine as the semantic reference and separate framing from contiguous data. I first moved the body step into the loop, then replaced bytewise writes in `ReadChunkData` with bulk `copy_from_slice`. I have also implemented bulk handling for `ReadFixedData`.

The automaton still controls framing, transitions, remaining length, continuation across input buffers, completion, and errors. Within a known-length data region, I copy the available payload in a block and update counters by that block length. CRLF inside payload is data, not framing. Each call uses a fresh arena and exposes only its written portion.

## Rationale

A known data region does not require a new state decision for every byte. Repeated equivalent steps can be replaced by a single operation preserving bytes, counters, and the next state. This is the optimization rationale, not a claim of complete formal verification of the Rust implementation.

## Results

POST: 256 KiB, 256 HTTP chunks of 1 KiB; 10 connections, one 10-second run per variant on the isolated pmd test setup.

| N0 implementation | Requests/s |
|---|---:|
| Loop without bulk copying | 4,211.62 |
| Bulk copying for chunked data | 14,377.20 |

The ratio is approximately 3.41. After bulk copying, mean latency was 0.670 ms and p99 was 1.64 ms, with 143,900 responses and no unsuccessful HTTP status codes. The earlier per-byte function-call implementation delivered approximately 2,048 requests/s.

A separate run for a 256 KiB fixed body delivered 14,592.81 requests/s, mean latency 0.655 ms, and p99 1.67 ms, without unsuccessful HTTP responses. There is no earlier fixed-body baseline for estimating its speedup.

Runs took place on different days and are not a controlled A/B experiment. Allocations were not measured. Performance results do not establish protocol correctness: the benchmark handler checks body length; functional tests check contents separately.

## Consequences and validation

I preserve streaming behavior and use bulk operations only where data length is known. I check fixed and chunked bodies, complete and fragmented input, multiple HTTP chunks, CRLF splits, and completion. Moving the step into the loop must preserve immediate exit on Success.

I also fixed a separate header-order defect: temporary flags identify the current header from its colon through the end of its value. The retained body type alone no longer selects how subsequent values are parsed. Local checks covered fixed and chunked requests with Connection both before and after the relevant header.

## Further work

- Investigate `memchr` for skipping irrelevant header regions.
- Continue header refactoring and boundary/error-input checks.
- Explore routing while reading the URI under [ADR-002](ADR-002-residual-language-router.en.md).
- Compare individual optimizations in controlled series and measure allocations separately.

## Principle

The formal automaton defines semantics; the optimized executor reduces overhead while preserving observable behavior.

Related material: [body model](../../nuxmv/body_parse_automaton_ver_5.smv), [optimization issue mirror](../issues/0004-parser-optimization.md).
