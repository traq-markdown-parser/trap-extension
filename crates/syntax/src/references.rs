use markdown_parser::{
    ParseError,
    engine::{
        Budget, Plugin,
        inline::{InlineInput, InlineMatch, InlineRule},
    },
};
pub use markdown_trap_contracts::{ReferenceData, ReferenceKind};

pub fn inline_rule() -> &'static InlineRule {
    static RULE: std::sync::LazyLock<InlineRule> =
        std::sync::LazyLock::new(|| InlineRule::new(b"!", parse).named("json_reference"));
    &RULE
}

pub fn plugin() -> &'static Plugin {
    static PLUGIN: std::sync::LazyLock<Plugin> = std::sync::LazyLock::new(|| {
        let mut plugin = Plugin::new(&markdown_trap_contracts::preset().references);
        plugin.add(inline_rule());
        plugin
    });
    &PLUGIN
}

fn parse(input: &InlineInput<'_>, budget: &mut Budget) -> Result<Option<InlineMatch>, ParseError> {
    if !input.tail().starts_with("!{") {
        return Ok(None);
    }
    let Some(end) = json_end(input, budget)? else {
        return Ok(None);
    };
    budget.spend(end - input.position)?;

    let text = &input.source.text()[input.position + 1..end];
    let Some(data) = reference_data(text) else {
        return Ok(None);
    };

    Ok(Some(InlineMatch::leaf(
        end,
        markdown_parser::NodeKind::new(data),
    )))
}

fn json_end(input: &InlineInput<'_>, budget: &mut Budget) -> Result<Option<usize>, ParseError> {
    let bytes = input.source.text().as_bytes();
    let mut stack = vec![];
    let mut quote = false;
    let mut escaped = false;

    for (index, &byte) in bytes.iter().enumerate().skip(input.position + 1) {
        budget.spend(1)?;
        if quote {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quote = false;
            }
            continue;
        }
        match byte {
            b'"' => quote = true,
            b'{' | b'[' => {
                stack.push(byte);
                budget.depth(stack.len())?;
            }
            b'}' | b']' => {
                if stack.pop() != Some(if byte == b'}' { b'{' } else { b'[' }) {
                    return Ok(None);
                }
                if stack.is_empty() {
                    return Ok(Some(index + 1));
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn reference_data(text: &str) -> Option<ReferenceData> {
    let value = serde_json::from_str::<serde_json::Value>(text).ok()?;
    let target = value.get("type").and_then(|value| value.as_str())?;
    let id = value.get("id").and_then(|value| value.as_str())?;
    let label = value.get("raw").and_then(|value| value.as_str())?;
    let target = match target {
        "user" => ReferenceKind::User,
        "group" => ReferenceKind::Group,
        "channel" => ReferenceKind::Channel,
        _ => return None,
    };

    Some(ReferenceData {
        target,
        id: id.into(),
        label: label.into(),
    })
}
