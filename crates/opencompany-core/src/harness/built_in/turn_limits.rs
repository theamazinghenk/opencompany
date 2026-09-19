//! Explicit per-brief budgets, transported with persisted delegation briefs.
//! Only a leading control line counts: quoted history or tool output cannot
//! accidentally change the caller's budget.
use serde_json::Value;
const PREFIX: &str = "[tool_call_limit=";

pub(super) fn from_brief(message: &str) -> Option<usize> {
    let line = message.lines().next()?.trim();
    line.strip_prefix(PREFIX)?.strip_suffix(']')?.parse().ok()
}

pub(super) fn bounded_brief(args: &Value, instruction: String) -> anyhow::Result<String> {
    let Some(value) = args.get("max_tool_calls") else {
        return Ok(instruction);
    };
    let limit = value
        .as_u64()
        .filter(|n| *n <= 1024)
        .ok_or_else(|| anyhow::anyhow!("max_tool_calls must be an integer from 0 through 1024"))?;
    // A pre-existing stricter cap survives a re-delegation.
    let limit = from_brief(&instruction).map_or(limit, |old| limit.min(old as u64));
    Ok(format!("{PREFIX}{limit}]\n{instruction}"))
}

#[cfg(test)]
#[path = "turn_limits_tests.rs"]
mod tests;
