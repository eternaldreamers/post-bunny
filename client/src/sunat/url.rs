pub(crate) fn absolute_url(base: &str, location: &str) -> String {
    if location.starts_with("http://") || location.starts_with("https://") {
        location.to_string()
    } else if let Some(rest) = location.strip_prefix("//") {
        format!("https://{rest}")
    } else if location.starts_with('/') {
        let scheme_end = base.find("://").map(|index| index + 3).unwrap_or(0);
        let host_end = base[scheme_end..]
            .find('/')
            .map(|index| scheme_end + index)
            .unwrap_or(base.len());
        format!("{}{}", &base[..host_end], location)
    } else if let Some(index) = base.rfind('/') {
        format!("{}{}", &base[..index + 1], location)
    } else {
        location.to_string()
    }
}

pub(crate) fn extract_between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_index = text.find(start)? + start.len();
    let rest = &text[start_index..];
    let end_index = rest.find(end)?;
    Some(&rest[..end_index])
}

pub(crate) fn query_param(url: &str, key: &str) -> Option<String> {
    let query = url.split_once('?')?.1;
    for pair in query.split('&') {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        if name == key {
            return urlencoding::decode(value)
                .ok()
                .map(|value| value.into_owned());
        }
    }
    None
}

pub(crate) fn sanitize_url(url: &str) -> String {
    let bytes = url.as_bytes();
    let mut out = String::with_capacity(url.len() + 32);
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%'
            && index + 2 < bytes.len()
            && bytes[index + 1].is_ascii_hexdigit()
            && bytes[index + 2].is_ascii_hexdigit()
        {
            out.push('%');
            out.push(bytes[index + 1] as char);
            out.push(bytes[index + 2] as char);
            index += 3;
            continue;
        }
        let character = byte as char;
        if character.is_ascii_alphanumeric() || "-._~:/?#[]@!$&'()*+,;=".contains(character) {
            out.push(character);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
        index += 1;
    }
    out
}
