# Benchmark Contract

QSOL-MACH does not claim to be the world's fastest runtime at Phase 0. The repository must earn performance claims with reproducible evidence.

## Primary metric

**Time to successful task completion (TTSuccess)** for a fixed model, environment, task set, and success criterion.

This prevents a fast but ineffective agent loop from winning merely by generating tokens quickly.

## Supporting metrics

- runtime validation latency per action;
- runtime dispatch overhead per action;
- trajectory update latency;
- actions per second through the kernel;
- model calls per successful task;
- generated tokens per successful task;
- input/prefill tokens per successful task;
- invalid actions per task;
- blocked actions per task;
- tool calls per task;
- retries per task;
- peak resident memory;
- allocations per action where measurable;
- time to first valid action;
- time to first tool execution.

Inference-engine metrics such as TTFT and tokens/second remain useful but are not sufficient on their own.

## Benchmark rules

Every published result must record:

1. exact MACH commit;
2. compiler and optimization profile;
3. operating system and kernel;
4. CPU, memory, and accelerator identity where applicable;
5. model and quantization identity;
6. inference backend and version;
7. task/environment revision;
8. warm/cold cache state;
9. concurrency level;
10. sample count and aggregation method;
11. raw machine-readable measurements.

Comparisons must hold workload semantics constant. An optimization that changes success criteria or silently removes required work is invalid.

## Benchmark families

Planned suites:

- kernel microbenchmarks with no model involved;
- deterministic tool-loop benchmarks;
- replay benchmarks using recorded trajectories;
- end-to-end agent benchmarks with fixed inference backends;
- inference-elision A/B tests measuring calls and tokens avoided;
- concurrency tests for many independent trajectories.
