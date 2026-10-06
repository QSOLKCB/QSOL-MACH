# Architecture

## Goal

QSOL-MACH is a deterministic runtime substrate for tool-using agents. It is designed around **time to successful useful work**, not tokens per second in isolation.

The runtime should remove work from the probabilistic model when — and only when — the next operation is mechanically decidable from explicit contracts and observed state.

## Phase 0 data path

    Typed Action
        |
        v
    ContractSet::validate
        | valid name + value kind
        v
    Executor::execute
        |
        +---- success ----> ExecutionReceipt<Output>
        |                  | environment_changed
        |                  | task_progress
        |                  v
        |             ActionOutcome::Succeeded
        |
        +---- error ------> ActionOutcome::Unknown
                           (dispatch is still recorded)
        |
        v
    bounded Trajectory
        |
        v
    incremental Trajectory::signals

Blocked actions stop before executor dispatch and are not added to the dispatched trajectory. Once dispatch occurs, an attempt is recorded even when the executor reports an error, because external side effects may already have occurred.

## Boundaries

The kernel owns:

- typed action representation and value kinds;
- deterministic contract checks;
- dispatch ordering;
- successful tool outputs;
- explicit successful-versus-unknown dispatch outcomes;
- bounded recent trajectory history;
- incremental repeated-action and task-progress streaks;
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

Phase 0 uses BTreeMap where iteration order can affect diagnostics. This gives stable first-failure behavior without depending on hash iteration order. Contracts bind every allowed argument name to an expected value kind before dispatch.

## Execution receipts

A successful executor call returns a typed output plus two distinct facts:

- environment_changed: whether the external environment was mutated;
- task_progress: whether the action advanced the task.

These are deliberately separate. A file read can be useful progress without mutating anything.

An executor error is different: the runtime cannot safely assume that no side effect occurred before the error was returned. The dispatched action is therefore recorded with ActionOutcome::Unknown.

## Bounded trajectory state

Trajectory history uses an explicit record-count retention limit supplied when the runtime is created. Recent records are held in a bounded deque, while cumulative record count and streak counters are maintained incrementally.

This keeps repeated-action and no-progress signal updates O(1) per dispatch instead of rescanning an ever-growing suffix.

The first retention budget is record-count based. A later phase may add byte-aware evidence storage once boundary payload limits are defined.

## Action realization

Later phases will add safe canonicalization for mechanically unambiguous interface mistakes. Canonicalization must never guess model intent. If more than one valid correction exists, the runtime must block or return control to the policy.

## Trajectory regulation

Phase 0 emits signals only. It does not autonomously rewrite plans. Repeated actions and consecutive explicit TaskProgress::NoProgress receipts are observable runtime facts; choosing a new semantic strategy remains a policy concern until a future phase defines a provably safe intervention.

## Inference elision

A later optimization target is **deterministic inference elision**: skip a model call when the runtime can prove there is exactly one admissible continuation under the active contract and observed state.

That optimization must be auditable and replayable. "Likely" is not sufficient.
