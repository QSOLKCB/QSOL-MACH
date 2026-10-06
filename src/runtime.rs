use std::num::NonZeroUsize;

use crate::{
    Action, ActionOutcome, ActionRecord, ContractSet, ContractViolation, TaskProgress, Trajectory,
    TrajectorySignal,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReceipt<O> {
    pub output: O,
    pub environment_changed: bool,
    pub task_progress: TaskProgress,
}

pub trait Executor {
    type Output;
    type Error;

    fn execute(&mut self, action: &Action) -> Result<ExecutionReceipt<Self::Output>, Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum StepOutcome<O, E> {
    Executed {
        receipt: ExecutionReceipt<O>,
        signals: Vec<TrajectorySignal>,
    },
    Blocked(ContractViolation),
    ExecutionFailed {
        error: E,
        signals: Vec<TrajectorySignal>,
    },
}

#[derive(Debug, Clone)]
pub struct Runtime {
    contracts: ContractSet,
    trajectory: Trajectory,
    signal_threshold: NonZeroUsize,
}

impl Runtime {
    pub fn new(
        contracts: ContractSet,
        signal_threshold: NonZeroUsize,
        trajectory_retention: NonZeroUsize,
    ) -> Self {
        Self {
            contracts,
            trajectory: Trajectory::new(trajectory_retention),
            signal_threshold,
        }
    }

    pub fn submit<E: Executor>(
        &mut self,
        action: Action,
        executor: &mut E,
    ) -> StepOutcome<E::Output, E::Error> {
        if let Err(violation) = self.contracts.validate(&action) {
            return StepOutcome::Blocked(violation);
        }

        match executor.execute(&action) {
            Ok(receipt) => {
                self.trajectory.push(ActionRecord {
                    action,
                    outcome: ActionOutcome::Succeeded {
                        environment_changed: receipt.environment_changed,
                        task_progress: receipt.task_progress,
                    },
                });
                let signals = self.trajectory.signals(self.signal_threshold);

                StepOutcome::Executed { receipt, signals }
            }
            Err(error) => {
                self.trajectory.push(ActionRecord {
                    action,
                    outcome: ActionOutcome::Unknown,
                });
                let signals = self.trajectory.signals(self.signal_threshold);

                StepOutcome::ExecutionFailed { error, signals }
            }
        }
    }

    pub fn trajectory(&self) -> &Trajectory {
        &self.trajectory
    }

    pub fn contracts(&self) -> &ContractSet {
        &self.contracts
    }
}
