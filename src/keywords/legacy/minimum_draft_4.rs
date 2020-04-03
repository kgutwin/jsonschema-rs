use super::super::CompilationResult;
use super::super::{exclusive_minimum, minimum};
use crate::compilation::CompilationContext;
use serde_json::{Map, Value};

pub(crate) fn compile(
    parent: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    match parent.get("exclusiveMinimum") {
        Some(Value::Bool(true)) => exclusive_minimum::compile(parent, schema, context),
        _ => minimum::compile(parent, schema, context),
    }
}