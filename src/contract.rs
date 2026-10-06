use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::{Action, ToolId, ValueKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolContract {
    tool: ToolId,
    required: BTreeMap<String, ValueKind>,
    allowed: BTreeMap<String, ValueKind>,
}

impl ToolContract {
    pub fn new<I, S>(tool: ToolId, required: I) -> Self
    where
        I: IntoIterator<Item = (S, ValueKind)>,
        S: Into<String>,
    {
        let required: BTreeMap<String, ValueKind> = required
            .into_iter()
            .map(|(name, kind)| (name.into(), kind))
            .collect();

        Self {
            tool,
            allowed: required.clone(),
            required,
        }
    }

    pub fn allow<I, S>(mut self, optional: I) -> Self
    where
        I: IntoIterator<Item = (S, ValueKind)>,
        S: Into<String>,
    {
        for (name, kind) in optional {
            self.allowed.entry(name.into()).or_insert(kind);
        }
        self
    }

    pub fn tool(&self) -> &ToolId {
        &self.tool
    }

    fn validate(&self, action: &Action) -> Result<(), ContractViolation> {
        if let Some(field) = self
            .required
            .keys()
            .find(|field| !action.arguments().contains_key(*field))
        {
            return Err(ContractViolation::MissingRequiredField {
                tool: self.tool.clone(),
                field: field.clone(),
            });
        }

        if let Some(field) = action
            .arguments()
            .keys()
            .find(|field| !self.allowed.contains_key(*field))
        {
            return Err(ContractViolation::UnexpectedField {
                tool: self.tool.clone(),
                field: field.clone(),
            });
        }

        for (field, value) in action.arguments() {
            let expected = self
                .allowed
                .get(field)
                .expect("unexpected fields are rejected above");
            let actual = value.kind();
            if *expected != actual {
                return Err(ContractViolation::WrongValueKind {
                    tool: self.tool.clone(),
                    field: field.clone(),
                    expected: *expected,
                    actual,
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContractSet {
    tools: BTreeMap<ToolId, ToolContract>,
}

impl ContractSet {
    pub fn insert(&mut self, contract: ToolContract) -> Option<ToolContract> {
        self.tools.insert(contract.tool.clone(), contract)
    }

    pub fn validate(&self, action: &Action) -> Result<(), ContractViolation> {
        let contract = self
            .tools
            .get(action.tool())
            .ok_or_else(|| ContractViolation::UnknownTool(action.tool().clone()))?;
        contract.validate(action)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractViolation {
    UnknownTool(ToolId),
    MissingRequiredField {
        tool: ToolId,
        field: String,
    },
    UnexpectedField {
        tool: ToolId,
        field: String,
    },
    WrongValueKind {
        tool: ToolId,
        field: String,
        expected: ValueKind,
        actual: ValueKind,
    },
}

impl fmt::Display for ContractViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTool(tool) => write!(f, "unknown tool: {tool}"),
            Self::MissingRequiredField { tool, field } => {
                write!(f, "tool {tool} is missing required field {field}")
            }
            Self::UnexpectedField { tool, field } => {
                write!(f, "tool {tool} received unexpected field {field}")
            }
            Self::WrongValueKind {
                tool,
                field,
                expected,
                actual,
            } => write!(
                f,
                "tool {tool} field {field} expects {expected}, received {actual}"
            ),
        }
    }
}

impl Error for ContractViolation {}
