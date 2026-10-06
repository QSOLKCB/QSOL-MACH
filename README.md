# QSOL-MACH

[![CI](https://github.com/QSOLKCB/QSOL-MACH/actions/workflows/ci.yml/badge.svg)](https://github.com/QSOLKCB/QSOL-MACH/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Phase](https://img.shields.io/badge/roadmap-Phase%200-orange.svg)](docs/ROADMAP.md)

**Deterministic low-latency runtime for LLM agents and tool-using systems.**

QSOL-MACH (Model-Agent Control Harness) explores a simple systems hypothesis:

> **The fastest model call is the one you never make.**

Rather than optimizing only tokens per second, MACH targets **time to successful useful work**. The runtime moves mechanically decidable work out of probabilistic inference and into a small deterministic substrate for contract validation, action execution, trajectory control, replay, and eventually inference elision.

## Phase 0

The initial runtime kernel provides:

- typed actions and arguments;
- deterministic tool contracts;
- pre-execution action validation;
- executor dispatch behind a narrow trait;
- trajectory recording;
- repeated-action and no-progress signals;
- zero third-party runtime dependencies;
- CI for formatting, linting, and tests.

MACH is **not** a model server and does not attempt to replace inference engines such as vLLM or llama.cpp. Model runtimes, MCP, HTTP APIs, and other transports are boundary adapters planned for later phases.

## Design target

    model / policy
          |
          v
    +-------------------------------+
    | QSOL-MACH                     |
    | contract -> action -> execute |
    |            |          |       |
    |            +-> trajectory     |
    +-------------------------------+
          |
          v
    environment / tools

The optimization target is end-to-end agent work: fewer invalid actions, fewer redundant model turns, less repeated context, lower runtime overhead, and deterministic replay where the environment permits it.

## Start here

- [Getting Started](GETTING_STARTED.md)
- [Usage and contribution instructions](INSTRUCTIONS.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Invariants](docs/INVARIANTS.md)
- [Benchmark contract](docs/BENCHMARKS.md)
- [Roadmap](docs/ROADMAP.md)
- [Research basis](docs/RESEARCH.md)

## Status

**Research prototype — Phase 0.** No performance-superiority claim is made yet. Any future "fastest" claim must be backed by reproducible benchmark evidence under the benchmark contract.
