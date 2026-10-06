# Instructions

QSOL-MACH is a performance project, but correctness comes before benchmark numbers.

## Change discipline

1. Preserve the invariants in docs/INVARIANTS.md.
2. Keep the runtime kernel deterministic and small.
3. Add dependencies only when a measured requirement justifies them.
4. Keep serialization, model APIs, MCP, and transport concerns at explicit boundaries.
5. Add a regression test for every correctness fix.
6. Do not turn an ambiguous model decision into a deterministic runtime decision.
7. Do not make performance claims without reproducible evidence defined by docs/BENCHMARKS.md.

## Pull requests

A PR should state:

- the runtime behavior being changed;
- the invariant or roadmap phase it advances;
- tests added or updated;
- benchmark impact, if measured;
- any new nondeterminism or external dependency introduced.

## Optimization rule

Prefer removing unnecessary work over making unnecessary work faster.
