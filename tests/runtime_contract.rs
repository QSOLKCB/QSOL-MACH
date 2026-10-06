use std::num::NonZeroUsize;

use qsol_mach::{
    Action, ActionOutcome, ContractSet, ContractViolation, ExecutionReceipt, Executor, Runtime,
    StepOutcome, TaskProgress, ToolContract, ToolId, TrajectorySignal, Value, ValueKind,
};

struct CountingExecutor {
    calls: usize,
    environment_changed: bool,
    task_progress: TaskProgress,
    fail: bool,
    output: &'static str,
}

impl Default for CountingExecutor {
    fn default() -> Self {
        Self {
            calls: 0,
            environment_changed: false,
            task_progress: TaskProgress::NoProgress,
            fail: false,
            output: "ok",
        }
    }
}

impl Executor for CountingExecutor {
    type Output = &'static str;
    type Error = &'static str;

    fn execute(&mut self, _action: &Action) -> Result<ExecutionReceipt<Self::Output>, Self::Error> {
        self.calls += 1;
        if self.fail {
            return Err("executor failed");
        }

        Ok(ExecutionReceipt {
            output: self.output,
            environment_changed: self.environment_changed,
            task_progress: self.task_progress,
        })
    }
}

fn runtime_with_retention(threshold: usize, retention: usize) -> Runtime {
    let tool = ToolId::new("read_file").unwrap();
    let contract =
        ToolContract::new(tool, [("path", ValueKind::Text)]).allow([("encoding", ValueKind::Text)]);
    let mut contracts = ContractSet::default();
    contracts.insert(contract);

    Runtime::new(
        contracts,
        NonZeroUsize::new(threshold).unwrap(),
        NonZeroUsize::new(retention).unwrap(),
    )
}

fn runtime(threshold: usize) -> Runtime {
    runtime_with_retention(threshold, 16)
}

fn read_action(path: &str) -> Action {
    Action::new(ToolId::new("read_file").unwrap()).with_argument("path", Value::Text(path.into()))
}

#[test]
fn valid_action_executes_is_recorded_and_returns_output() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor {
        environment_changed: true,
        task_progress: TaskProgress::Advanced,
        output: "file contents",
        ..Default::default()
    };

    let outcome = runtime.submit(read_action("README.md"), &mut executor);

    assert_eq!(executor.calls, 1);
    assert_eq!(runtime.trajectory().len(), 1);
    assert_eq!(
        outcome,
        StepOutcome::Executed {
            receipt: ExecutionReceipt {
                output: "file contents",
                environment_changed: true,
                task_progress: TaskProgress::Advanced,
            },
            signals: vec![],
        }
    );
}

#[test]
fn unknown_tool_is_blocked_before_execution() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor::default();
    let action = Action::new(ToolId::new("delete_everything").unwrap());

    let outcome = runtime.submit(action, &mut executor);

    assert_eq!(executor.calls, 0);
    assert!(runtime.trajectory().is_empty());
    assert!(matches!(
        outcome,
        StepOutcome::Blocked(ContractViolation::UnknownTool(_))
    ));
}

#[test]
fn missing_required_field_is_blocked_before_execution() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor::default();
    let action = Action::new(ToolId::new("read_file").unwrap());

    let outcome = runtime.submit(action, &mut executor);

    assert_eq!(executor.calls, 0);
    match outcome {
        StepOutcome::Blocked(ContractViolation::MissingRequiredField { field, .. }) => {
            assert_eq!(field, "path");
        }
        other => panic!("expected missing-field block, got {other:?}"),
    }
}

#[test]
fn unexpected_field_is_blocked_before_execution() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor::default();
    let action = read_action("README.md").with_argument("surprise", Value::Bool(true));

    let outcome = runtime.submit(action, &mut executor);

    assert_eq!(executor.calls, 0);
    match outcome {
        StepOutcome::Blocked(ContractViolation::UnexpectedField { field, .. }) => {
            assert_eq!(field, "surprise");
        }
        other => panic!("expected unexpected-field block, got {other:?}"),
    }
}

#[test]
fn wrong_value_kind_is_blocked_before_execution() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor::default();
    let action =
        Action::new(ToolId::new("read_file").unwrap()).with_argument("path", Value::Integer(7));

    let outcome = runtime.submit(action, &mut executor);

    assert_eq!(executor.calls, 0);
    match outcome {
        StepOutcome::Blocked(ContractViolation::WrongValueKind {
            field,
            expected,
            actual,
            ..
        }) => {
            assert_eq!(field, "path");
            assert_eq!(expected, ValueKind::Text);
            assert_eq!(actual, ValueKind::Integer);
        }
        other => panic!("expected value-kind block, got {other:?}"),
    }
}

#[test]
fn failed_dispatch_is_recorded_with_unknown_outcome() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor {
        fail: true,
        ..Default::default()
    };

    let outcome = runtime.submit(read_action("README.md"), &mut executor);

    assert_eq!(executor.calls, 1);
    assert_eq!(runtime.trajectory().len(), 1);
    assert_eq!(
        runtime.trajectory().records().back().unwrap().outcome,
        ActionOutcome::Unknown
    );
    assert_eq!(
        outcome,
        StepOutcome::ExecutionFailed {
            error: "executor failed",
            signals: vec![],
        }
    );
}

#[test]
fn repeated_actions_emit_a_deterministic_signal() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor {
        environment_changed: true,
        task_progress: TaskProgress::Advanced,
        ..Default::default()
    };

    runtime.submit(read_action("README.md"), &mut executor);
    runtime.submit(read_action("README.md"), &mut executor);
    let outcome = runtime.submit(read_action("README.md"), &mut executor);

    let StepOutcome::Executed { signals, .. } = outcome else {
        panic!("expected execution");
    };
    assert_eq!(signals, vec![TrajectorySignal::RepeatedAction { count: 3 }]);
}

#[test]
fn useful_read_only_actions_count_as_task_progress() {
    let mut runtime = runtime(2);
    let mut executor = CountingExecutor {
        environment_changed: false,
        task_progress: TaskProgress::Advanced,
        ..Default::default()
    };

    runtime.submit(read_action("README.md"), &mut executor);
    let outcome = runtime.submit(read_action("LICENSE"), &mut executor);

    let StepOutcome::Executed { signals, .. } = outcome else {
        panic!("expected execution");
    };
    assert_eq!(runtime.trajectory().no_progress_streak(), 0);
    assert!(
        !signals
            .iter()
            .any(|signal| matches!(signal, TrajectorySignal::NoProgress { .. }))
    );
}

#[test]
fn explicit_task_stalls_emit_no_progress() {
    let mut runtime = runtime(2);
    let mut executor = CountingExecutor::default();

    runtime.submit(read_action("README.md"), &mut executor);
    let outcome = runtime.submit(read_action("LICENSE"), &mut executor);

    let StepOutcome::Executed { signals, .. } = outcome else {
        panic!("expected execution");
    };
    assert_eq!(signals, vec![TrajectorySignal::NoProgress { count: 2 }]);
}

#[test]
fn streaks_are_incremental_and_history_is_bounded() {
    let mut runtime = runtime_with_retention(2, 3);
    let mut executor = CountingExecutor::default();

    let mut last = None;
    for _ in 0..100 {
        last = Some(runtime.submit(read_action("README.md"), &mut executor));
    }

    assert_eq!(runtime.trajectory().len(), 3);
    assert_eq!(runtime.trajectory().total_records(), 100);
    assert_eq!(runtime.trajectory().repeated_action_streak(), 100);
    assert_eq!(runtime.trajectory().no_progress_streak(), 100);

    let StepOutcome::Executed { signals, .. } = last.unwrap() else {
        panic!("expected execution");
    };
    assert_eq!(
        signals,
        vec![
            TrajectorySignal::RepeatedAction { count: 100 },
            TrajectorySignal::NoProgress { count: 100 },
        ]
    );
}
