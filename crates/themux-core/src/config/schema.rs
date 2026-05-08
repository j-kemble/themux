// Config schema: stub for cmux.schema.json compatibility.

/// Returns the embedded JSON Schema for config validation.
pub fn schema_json() -> &'static str {
    include_str!("./schema.json")
}
