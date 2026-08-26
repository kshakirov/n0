# N0

N0 is an experimental minimal server composed of two parts:

- **Node.js** provides raw TCP networking through the built-in `node:net` module.
- **Rust** provides the native byte-processing core through a shared-library boundary.

## Initial Goal

The first approximation must establish the smallest complete working path:

```text
TCP client
    -> Node.js Buffer
    -> Rust shared library
    -> processed bytes
    -> Node.js
    -> TCP client
```

The initial transformation may return the input unchanged. Its purpose is to prove that bytes can travel through the complete physical path from the network into Rust and back to the client.

## Principles

### Incremental construction

Introduce a component only when the current working approximation requires it.

### MaxEnt and Occam

Do not prematurely constrain the architecture. Choose the smallest structure consistent with known requirements, and do not introduce abstractions for hypothetical future needs.

### Raw bytes first

The initial system operates on byte buffers. Do not introduce DTOs, DOM trees, object graphs, serialization frameworks, or intermediate string representations unless their necessity is demonstrated.

### Explicit division of responsibility

- Node.js owns networking and event dispatch.
- Rust owns native byte processing.
- The boundary between them must remain narrow and measurable.

### Composition over monoliths

Future functionality may be introduced as small composable tools. Markov algorithms, Refal-like transformations, JSON processing, arenas, nodes, and offset tables are research directions, not parts of the initial architecture.

### Measure before optimizing

Minimize allocations and copying where practical, but verify their actual cost through measurements rather than assumptions.

## Initial Structure

```text
n0/
├── src/
│   ├── node/
│   └── rust/
├── README.md
└── .gitignore
```

No additional directories should be introduced without actual content and a demonstrated need.

## First Milestone

The first milestone is complete when:

1. A TCP client sends a byte sequence.
2. Node.js receives it as a `Buffer`.
3. Rust processes the bytes through a shared-library boundary.
4. Node.js receives the result.
5. The server sends the resulting bytes back to the client.

## Current Non-Goals

The first approximation does not include:

- HTTP;
- JSON parsing;
- Markov algorithms;
- Refal;
- arenas;
- node graphs or trees;
- plugin systems;
- generalized tool interfaces;
- persistence;
- multithreaded processing;
- production deployment infrastructure.

These elements may appear only when required by a later experiment.

## Development Rule

The local agent may inspect, analyze, test, and suggest changes. Architectural decisions, refactoring, and source-code changes remain under the project owner's control unless explicitly requested.
