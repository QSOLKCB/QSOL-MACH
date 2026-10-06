use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::{Action, ToolId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolContract {
    tool: ToolId,
    required: BTreeSet<String>,
    allowed: BTreeSet<String>,
}

impl ToolContract {
    pub fn new<I, S>(tool: ToolId, required: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let required: BTreeSet<String> = required.into_iter().map(Into::into).collect();
        Self {
            tool,
            allowed: required.clone(),
            required,
        }
    }

    pub fn allow<I, S>(mut self, optional: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.allowed.extend(optional.into_iter().map(Into::into));
        self
    }

    pub fn tool(&self) -> &ToolId {
        &self.tool
    }

    fn validate(&self, action: &Action) -> Result<(), ContractViolation> {
        if let Some(field) = self
            .required
            .iter()
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
            .find(|field| !self.allowed.contains(*field))
        {
            return Err(ContractViolation::UnexpectedField {
                tool: self.tool.clone(),
                field: field.clone(),
            });
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
    MissingRequiredField { tool: ToolId, field: String },
    UnexpectedField { tool: ToolId, field: String },
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
        }
    }
}

impl Error for ContractViolation {}
