# Architecture

## Goal

QSOL-MACH is a deterministic runtime substrate for tool-using agents. It is designed around **time to successful useful work**, not tokens per second in isolation.

The runtime should remove work from the probabilistic model when — and only when — the next operation is mechanically decidable from explicit contracts and observed state.

## Phase 0 data path

    Action
      |
      v
    ContractSet::validate
      | valid
      v
    Executor::execute
      |
      v
    ExecutionReceipt
      |
      v
    Trajectory::push
      |
      v
    Trajectory::signals

Invalid actions stop before executor dispatch and are not added to the executed trajectory.

## Boundaries

The kernel owns:

- typed action representation;
- deterministic contract checks;
- dispatch ordering;
- executed-action trajectory state;
- mechanically derived trajectory signals.

The kernel does not currently own:

- model inference;
- tokenization;
- KV-cache management;
- HTTP or MCP transport;
- JSON parsing;
- operating-system sandboxing;
- semantic task reasoning.

Those facilities will be introduced as explicit adapters or later runtime layers so their costs and semantics remain measurable.

## Why typed internal actions

MACH should not repeatedly parse and reserialize text inside the hot loop. Textual protocols are boundary formats. Once decoded, the runtime works on typed actions and deterministic ordered collections.

Phase 0 uses BTreeMap and BTreeSet where iteration order can affect diagnostics. This gives stable first-failure behavior without depending on hash iteration order.

## Action realization

Later phases will add safe canonicalization for mechanically unambiguous interface mistakes. Canonicalization must never guess model intent. If more than one valid correction exists, the runtime must block or return control to the policy.

## Trajectory regulation

Phase 0 emits signals only. It does not autonomously rewrite plans. Repeated actions and consecutive no-progress receipts are observable runtime facts; choosing a new semantic strategy remains a policy concern until a future phase defines a provably safe intervention.

## Inference elision

A later optimization target is **deterministic inference elision**: skip a model call when the runtime can prove there is exactly one admissible continuation under the active contract and observed state.

That optimization must be auditable and replayable. "Likely" is not sufficient.
