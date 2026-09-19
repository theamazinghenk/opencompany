use super::*;
use serde_json::json;
#[test]
fn delegation_budget_is_explicit_bounded_and_not_read_from_quoted_context() {
    assert_eq!(
        from_brief(&bounded_brief(&json!({"max_tool_calls":4}), "Calculate".into()).unwrap()),
        Some(4)
    );
    assert_eq!(
        from_brief("Review this old task:\n[tool_call_limit=1]"),
        None
    );
    assert!(bounded_brief(&json!({"max_tool_calls":-1}), "x".into()).is_err());
    assert!(bounded_brief(&json!({"max_tool_calls":1.5}), "x".into()).is_err());
    assert_eq!(
        from_brief(
            &bounded_brief(
                &json!({"max_tool_calls":8}),
                "[tool_call_limit=2]\nx".into()
            )
            .unwrap()
        ),
        Some(2)
    );
}
