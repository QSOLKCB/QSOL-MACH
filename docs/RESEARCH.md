# Research Basis

## Core question

How much agent work can be moved from probabilistic inference into deterministic runtime machinery without reducing task correctness or silently replacing model reasoning?

A related engineering question follows:

> Can reducing unnecessary inference produce larger end-to-end gains than optimizing each model invocation in isolation?

## Runtime-harness motivation

The initial research motivation comes from:

Tianshi Xu, Huifeng Wen, and Meng Li, **"Adapting the Interface, Not the Model: Runtime Harness Adaptation for Deterministic LLM Agents"**, arXiv:2605.22166v2, 2026.

The paper treats an LLM agent as a model embedded in a stateful interaction loop and adapts the runtime interface rather than model weights. Its LIFE-HARNESS organizes interventions across four lifecycle layers:

1. environment contract;
2. procedural skill;
3. action realization;
4. trajectory regulation.

The reported evaluation spans seven deterministic environments and 18 model backbones. The authors report improvement in 116 of 126 model-environment settings, with an average relative improvement of 88.5%.

## MACH's distinction

QSOL-MACH is not an implementation of LIFE-HARNESS. It uses the paper as a research prompt and asks a different systems question: what happens when the runtime interface itself is designed as a small, measured, deterministic performance substrate?

MACH additionally treats the following as first-class research targets:

- runtime overhead;
- model calls avoided;
- tokens avoided;
- compiled contracts;
- deterministic replay;
- concurrency and scheduling;
- end-to-end time to successful work;
- inference elision when a unique continuation can be proven.

## Scope discipline

The paper explicitly focuses on stable, rule-governed environments. MACH adopts the same useful constraint for early phases. Filesystems, structured APIs, databases, synthetic tool environments, and replay fixtures are better first targets than unrestricted open-domain autonomy because they make correctness and equivalence measurable.
