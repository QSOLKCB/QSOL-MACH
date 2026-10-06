# Roadmap

The roadmap deliberately separates runtime semantics from inference-engine optimization.

## Phase 0 — Deterministic kernel foundation

**Status: implemented by the initial PR.**

- typed actions;
- deterministic tool contracts;
- validate-before-execute dispatch;
- execution receipts;
- trajectory recording;
- repeated-action and no-progress signals;
- tests and CI;
- benchmark and invariant contracts.

## Phase 1 — Canonical wire format and contract compiler

- canonical machine-readable contract format;
- parser at the boundary;
- compiled internal validators;
- stable contract identity/hash;
- fixtures for malformed and ambiguous contracts.

## Phase 2 — Action realization

- deterministic interface-error canonicalization;
- explicit block reasons;
- admissible-action checks;
- proof-by-test that ambiguous corrections are never guessed.

## Phase 3 — Trajectory regulation

- configurable loop/stall detectors;
- budget signals;
- state-equivalence hooks;
- soft recovery directives without hidden semantic planning.

## Phase 4 — Replay and evidence

- canonical execution receipts;
- deterministic replay where environment semantics permit it;
- raw evidence bundle for correctness and performance runs;
- divergence reporting.

## Phase 5 — Tool and model adapters

- MCP adapter;
- OpenAI-compatible model adapter;
- local model adapter;
- shell/filesystem/database test environments behind capability boundaries.

## Phase 6 — Scheduler and cache awareness

- concurrent trajectories;
- prefix/KV lineage hints for supporting inference backends;
- batching-aware scheduling;
- backpressure and bounded memory policies.

## Phase 7 — Deterministic inference elision

- unique-continuation detection;
- auditable inference-skip receipts;
- token/model-call savings accounting;
- equivalence tests against non-elided execution.

## Phase 8 — Benchmark suite

- kernel microbenchmarks;
- deterministic end-to-end tasks;
- time-to-success measurements;
- concurrency and memory studies;
- reproducible comparison bundles.

## Phase 9 — Isolation and capability execution

- executor sandbox boundary;
- least-capability tool grants;
- time/resource limits;
- failure containment.

## Phase 10 — Offline harness evolution

- trajectory failure mining;
- local proposed contract/regulation updates;
- regression gates;
- freeze-and-evaluate workflow.

## Phase 11 — Optimization passes

Only after profiling:

- allocation reduction;
- specialized contract layouts;
- zero-copy boundary experiments;
- branch/layout tuning;
- backend-specific scheduling hooks.

## Phase 12 — Semantic freeze and formal verification

Once the runtime semantics stop moving:

- freeze the invariant set;
- formalize the smallest useful core;
- prove selected transition/validation properties;
- archive the verified release and evidence.
