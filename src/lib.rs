#![forbid(unsafe_code)]

//! QSOL-MACH runtime substrate.
//!
//! Phase 0 intentionally keeps the core small: typed actions, deterministic
//! contracts, executor dispatch, and trajectory-derived signals. Model and
//! transport adapters belong at the boundary, not in the runtime kernel.

pub mod action;
pub mod contract;
pub mod runtime;
pub mod trajectory;

pub use action::{Action, ActionError, ToolId, Value, ValueKind};
pub use contract::{ContractSet, ContractViolation, ToolContract};
pub use runtime::{ExecutionReceipt, Executor, Runtime, StepOutcome};
pub use trajectory::{
    ActionOutcome, ActionRecord, TaskProgress, Trajectory, TrajectorySignal,
};
