use std::num::NonZeroUsize;

use crate::{Action, ActionRecord, ContractSet, ContractViolation, Trajectory, TrajectorySignal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub changed_state: bool,
}

pub trait Executor {
    type Error;

    fn execute(&mut self, action: &Action) -> Result<ExecutionReceipt, Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum StepOutcome<E> {
    Executed {
        receipt: ExecutionReceipt,
        signals: Vec<TrajectorySignal>,
    },
    Blocked(ContractViolation),
    ExecutionFailed(E),
}

#[derive(Debug, Clone)]
pub struct Runtime {
    contracts: ContractSet,
    trajectory: Trajectory,
    signal_threshold: NonZeroUsize,
}

impl Runtime {
    pub fn new(contracts: ContractSet, signal_threshold: NonZeroUsize) -> Self {
        Self {
            contracts,
            trajectory: Trajectory::default(),
            signal_threshold,
        }
    }

    pub fn submit<E: Executor>(
        &mut self,
        action: Action,
        executor: &mut E,
    ) -> StepOutcome<E::Error> {
        if let Err(violation) = self.contracts.validate(&action) {
            return StepOutcome::Blocked(violation);
        }

        let receipt = match executor.execute(&action) {
            Ok(receipt) => receipt,
            Err(error) => return StepOutcome::ExecutionFailed(error),
        };

        self.trajectory.push(ActionRecord {
            action,
            changed_state: receipt.changed_state,
        });
        let signals = self.trajectory.signals(self.signal_threshold);

        StepOutcome::Executed { receipt, signals }
    }

    pub fn trajectory(&self) -> &Trajectory {
        &self.trajectory
    }

    pub fn contracts(&self) -> &ContractSet {
        &self.contracts
    }
}
