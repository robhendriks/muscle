use regex::Regex;

pub fn extract_wildcard_components(pattern: &str, input: &str) -> Option<Vec<String>> {
    let regex_pattern = format!("^{}$", pattern.replace("**", "(.+?)"));
    let re = Regex::new(&regex_pattern).ok()?;

    let captures = re.captures(input)?;
    let components: Vec<String> = captures
        .iter()
        .skip(1)
        .filter_map(|m| m)
        .flat_map(|m| m.as_str().split('/'))
        .map(|s| s.to_string())
        .collect();

    (!components.is_empty()).then_some(components)
}
