use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToolId(String);

impl ToolId {
    pub fn new(value: impl Into<String>) -> Result<Self, ActionError> {
        let value = value.into();
        if value.is_empty() {
            return Err(ActionError::EmptyToolId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ToolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    EmptyToolId,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyToolId => f.write_str("tool id must not be empty"),
        }
    }
}

impl Error for ActionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Text(String),
    Integer(i64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    tool: ToolId,
    arguments: BTreeMap<String, Value>,
}

impl Action {
    pub fn new(tool: ToolId) -> Self {
        Self {
            tool,
            arguments: BTreeMap::new(),
        }
    }

    pub fn with_argument(mut self, name: impl Into<String>, value: Value) -> Self {
        self.arguments.insert(name.into(), value);
        self
    }

    pub fn tool(&self) -> &ToolId {
        &self.tool
    }

    pub fn arguments(&self) -> &BTreeMap<String, Value> {
        &self.arguments
    }
}
