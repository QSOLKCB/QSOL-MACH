use std::num::NonZeroUsize;

use qsol_mach::{
    Action, ContractSet, ContractViolation, ExecutionReceipt, Executor, Runtime, StepOutcome,
    ToolContract, ToolId, TrajectorySignal, Value,
};

#[derive(Default)]
struct CountingExecutor {
    calls: usize,
    changed_state: bool,
}

impl Executor for CountingExecutor {
    type Error = &'static str;

    fn execute(&mut self, _action: &Action) -> Result<ExecutionReceipt, Self::Error> {
        self.calls += 1;
        Ok(ExecutionReceipt {
            changed_state: self.changed_state,
        })
    }
}

fn runtime(threshold: usize) -> Runtime {
    let tool = ToolId::new("read_file").unwrap();
    let contract = ToolContract::new(tool, ["path"]).allow(["encoding"]);
    let mut contracts = ContractSet::default();
    contracts.insert(contract);
    Runtime::new(contracts, NonZeroUsize::new(threshold).unwrap())
}

fn read_action() -> Action {
    Action::new(ToolId::new("read_file").unwrap())
        .with_argument("path", Value::Text("README.md".into()))
}

#[test]
fn valid_action_executes_and_is_recorded() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor {
        changed_state: true,
        ..Default::default()
    };

    let outcome = runtime.submit(read_action(), &mut executor);

    assert_eq!(executor.calls, 1);
    assert_eq!(runtime.trajectory().len(), 1);
    assert_eq!(
        outcome,
        StepOutcome::Executed {
            receipt: ExecutionReceipt {
                changed_state: true,
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
    let action = read_action().with_argument("surprise", Value::Bool(true));

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
fn repeated_actions_emit_a_deterministic_signal() {
    let mut runtime = runtime(3);
    let mut executor = CountingExecutor {
        changed_state: true,
        ..Default::default()
    };

    runtime.submit(read_action(), &mut executor);
    runtime.submit(read_action(), &mut executor);
    let outcome = runtime.submit(read_action(), &mut executor);

    let StepOutcome::Executed { signals, .. } = outcome else {
        panic!("expected execution");
    };
    assert_eq!(signals, vec![TrajectorySignal::RepeatedAction { count: 3 }]);
}

#[test]
fn no_progress_emits_a_deterministic_signal() {
    let mut runtime = runtime(2);
    let mut executor = CountingExecutor::default();

    runtime.submit(read_action(), &mut executor);
    let outcome = runtime.submit(
        Action::new(ToolId::new("read_file").unwrap())
            .with_argument("path", Value::Text("LICENSE".into())),
        &mut executor,
    );

    let StepOutcome::Executed { signals, .. } = outcome else {
        panic!("expected execution");
    };
    assert_eq!(signals, vec![TrajectorySignal::NoProgress { count: 2 }]);
}
