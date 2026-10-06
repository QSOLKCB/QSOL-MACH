use std::num::NonZeroUsize;

use crate::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRecord {
    pub action: Action,
    pub changed_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrajectorySignal {
    RepeatedAction { count: usize },
    NoProgress { count: usize },
}

#[derive(Debug, Clone, Default)]
pub struct Trajectory {
    records: Vec<ActionRecord>,
}

impl Trajectory {
    pub fn push(&mut self, record: ActionRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[ActionRecord] {
        &self.records
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn repeated_action_streak(&self) -> usize {
        let Some(last) = self.records.last() else {
            return 0;
        };

        self.records
            .iter()
            .rev()
            .take_while(|record| record.action == last.action)
            .count()
    }

    pub fn no_progress_streak(&self) -> usize {
        self.records
            .iter()
            .rev()
            .take_while(|record| !record.changed_state)
            .count()
    }

    pub fn signals(&self, threshold: NonZeroUsize) -> Vec<TrajectorySignal> {
        let threshold = threshold.get();
        let mut signals = Vec::new();
        let repeated = self.repeated_action_streak();
        if repeated >= threshold {
            signals.push(TrajectorySignal::RepeatedAction { count: repeated });
        }

        let stalled = self.no_progress_streak();
        if stalled >= threshold {
            signals.push(TrajectorySignal::NoProgress { count: stalled });
        }

        signals
    }
}
