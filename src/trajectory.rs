use std::collections::VecDeque;
use std::num::NonZeroUsize;

use crate::Action;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskProgress {
    Advanced,
    NoProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionOutcome {
    Succeeded {
        environment_changed: bool,
        task_progress: TaskProgress,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRecord {
    pub action: Action,
    pub outcome: ActionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrajectorySignal {
    RepeatedAction { count: usize },
    NoProgress { count: usize },
}

#[derive(Debug, Clone)]
pub struct Trajectory {
    records: VecDeque<ActionRecord>,
    retention_limit: NonZeroUsize,
    total_records: u64,
    last_action: Option<Action>,
    repeated_action_streak: usize,
    no_progress_streak: usize,
}

impl Trajectory {
    pub fn new(retention_limit: NonZeroUsize) -> Self {
        Self {
            records: VecDeque::with_capacity(retention_limit.get()),
            retention_limit,
            total_records: 0,
            last_action: None,
            repeated_action_streak: 0,
            no_progress_streak: 0,
        }
    }

    pub fn push(&mut self, record: ActionRecord) {
        self.repeated_action_streak = if self.last_action.as_ref() == Some(&record.action) {
            self.repeated_action_streak.saturating_add(1)
        } else {
            1
        };
        self.last_action = Some(record.action.clone());

        self.no_progress_streak = match record.outcome {
            ActionOutcome::Succeeded {
                task_progress: TaskProgress::NoProgress,
                ..
            } => self.no_progress_streak.saturating_add(1),
            ActionOutcome::Succeeded {
                task_progress: TaskProgress::Advanced,
                ..
            }
            | ActionOutcome::Unknown => 0,
        };

        self.total_records = self.total_records.saturating_add(1);
        self.records.push_back(record);

        while self.records.len() > self.retention_limit.get() {
            self.records.pop_front();
        }
    }

    pub fn records(&self) -> &VecDeque<ActionRecord> {
        &self.records
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn total_records(&self) -> u64 {
        self.total_records
    }

    pub fn retention_limit(&self) -> NonZeroUsize {
        self.retention_limit
    }

    pub fn repeated_action_streak(&self) -> usize {
        self.repeated_action_streak
    }

    pub fn no_progress_streak(&self) -> usize {
        self.no_progress_streak
    }

    pub fn signals(&self, threshold: NonZeroUsize) -> Vec<TrajectorySignal> {
        let threshold = threshold.get();
        let mut signals = Vec::new();

        if self.repeated_action_streak >= threshold {
            signals.push(TrajectorySignal::RepeatedAction {
                count: self.repeated_action_streak,
            });
        }

        if self.no_progress_streak >= threshold {
            signals.push(TrajectorySignal::NoProgress {
                count: self.no_progress_streak,
            });
        }

        signals
    }
}
