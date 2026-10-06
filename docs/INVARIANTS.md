# Runtime Invariants

These invariants are normative for the MACH kernel.

## MACH-I001 — Validate before execute

No action reaches an executor until it passes the active contract.

## MACH-I002 — Blocked means not executed

A blocked action must not call the executor and must not be appended to the executed trajectory.

## MACH-I003 — Deterministic validation

For identical contracts and identical typed actions, validation returns the same result and the same first reported violation.

## MACH-I004 — No hidden inference

The runtime kernel never invokes a model implicitly. Model calls must be explicit boundary operations so model-call counts remain measurable.

## MACH-I005 — Evidence-bound trajectory signals

Trajectory signals are derived only from recorded actions, execution receipts, configured thresholds, and later explicitly declared environment evidence.

## MACH-I006 — No invented certainty

The runtime must not canonicalize, execute, or elide inference when multiple semantically valid continuations remain.

## MACH-I007 — Typed hot path

Serialization formats are boundary concerns. Core validation and trajectory logic operate on typed values.

## MACH-I008 — Safe kernel baseline

Phase 0 forbids unsafe Rust in the crate.

## MACH-I009 — Benchmark claims require receipts

Performance claims require reproducible configuration, hardware/software identity, build mode, raw measurements, and comparison methodology.
