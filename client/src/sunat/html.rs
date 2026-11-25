pub(crate) fn html_to_text(html: &str) -> String {
    let mut out = String::new();
    let mut tag = String::new();
    let mut in_tag = false;
    let mut skip_tag: Option<String> = None;

    for character in html.chars() {
        if in_tag {
            if character == '>' {
                let lower = tag.trim().to_ascii_lowercase();
                if let Some(skip) = skip_tag.clone() {
                    if lower == format!("/{skip}") {
                        skip_tag = None;
                    }
                } else if lower.starts_with("script") || lower.starts_with("style") {
                    let name = lower
                        .split_whitespace()
                        .next()
                        .unwrap_or("script")
                        .trim_start_matches('<')
                        .to_string();
                    skip_tag = Some(name);
                } else if lower.starts_with("br")
                    || lower.starts_with("/p")
                    || lower.starts_with("/div")
                    || lower.starts_with("/tr")
                    || lower.starts_with("/li")
                    || lower.starts_with("/h")
                {
                    out.push('\n');
                } else if lower.starts_with("td") || lower.starts_with("/td") {
                    out.push(' ');
                }
                tag.clear();
                in_tag = false;
            } else {
                tag.push(character);
            }
        } else if character == '<' {
            in_tag = true;
            tag.clear();
        } else if skip_tag.is_none() {
            out.push(character);
        }
    }

    let decoded = decode_entities(&out);
    let mut lines = Vec::new();
    let mut blank = false;
    for line in decoded.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !blank {
                lines.push(String::new());
            }
            blank = true;
        } else {
            lines.push(line.to_string());
            blank = false;
        }
    }
    lines.join("\n").trim().to_string()
}

fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(index) = rest.find('&') {
        out.push_str(&rest[..index]);
        rest = &rest[index..];
        let Some(end) = rest.find(';') else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        if end > 12 {
            out.push('&');
            rest = &rest[1..];
            continue;
        }
        let entity = &rest[1..end];
        if let Some(decoded) = decode_entity(entity) {
            out.push(decoded);
            rest = &rest[end + 1..];
        } else {
            out.push('&');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

fn decode_entity(entity: &str) -> Option<char> {
    let named = match entity {
        "nbsp" => ' ',
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "aacute" => 'á',
        "eacute" => 'é',
        "iacute" => 'í',
        "oacute" => 'ó',
        "uacute" => 'ú',
        "ntilde" => 'ñ',
        "Aacute" => 'Á',
        "Eacute" => 'É',
        "Iacute" => 'Í',
        "Oacute" => 'Ó',
        "Uacute" => 'Ú',
        "Ntilde" => 'Ñ',
        "ordf" => 'ª',
        "ordm" => 'º',
        "deg" => '°',
        "hellip" => '…',
        "mdash" => '—',
        "ndash" => '–',
        _ => {
            if let Some(hex) = entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
            {
                return u32::from_str_radix(hex, 16).ok().and_then(char::from_u32);
            }
            if let Some(decimal) = entity.strip_prefix('#') {
                return decimal.parse::<u32>().ok().and_then(char::from_u32);
            }
            return None;
        }
    };
    Some(named)
}
