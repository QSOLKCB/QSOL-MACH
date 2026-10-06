# AGENTS.md

Instructions for coding agents working in QSOL-MACH.

- Treat docs/INVARIANTS.md as normative.
- Keep Phase 0 free of third-party runtime dependencies unless the task explicitly changes that contract.
- Never bypass contract validation to improve a benchmark.
- Never execute a blocked action.
- Do not infer hidden environment state or fabricate deterministic certainty.
- Prefer typed internal structures; serialization belongs at boundaries.
- Keep changes local. A second concrete implementation should earn a shared abstraction.
- Every bug fix requires a failing regression case that passes after the fix.
- Performance work must preserve behavior and report the benchmark method, hardware, build mode, sample count, and raw measurements.
- Formal verification belongs after runtime semantics stabilize; do not prematurely encode a moving design in proofs.
