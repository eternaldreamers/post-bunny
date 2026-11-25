pub fn is_valid_dni(dni: &str) -> bool {
    dni.len() == 8 && dni.chars().all(|character| character.is_ascii_digit())
}

pub fn is_valid_ruc(ruc: &str) -> bool {
    if ruc.len() != 11 || !ruc.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }
    ruc_check_digit(&ruc[..10])
        .map(|digit| digit == ruc.as_bytes()[10] - b'0')
        .unwrap_or(false)
}

pub fn ruc_from_dni(dni: &str) -> Option<String> {
    if !is_valid_dni(dni) {
        return None;
    }
    let base = format!("10{dni}");
    let check = ruc_check_digit(&base)?;
    Some(format!("{base}{check}"))
}

fn ruc_check_digit(base: &str) -> Option<u8> {
    if base.len() != 10 {
        return None;
    }
    const WEIGHTS: [u32; 10] = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = base
        .chars()
        .zip(WEIGHTS)
        .map(|(character, weight)| character.to_digit(10).map(|digit| digit * weight))
        .sum::<Option<u32>>()?;
    let remainder = 11 - (sum % 11);
    Some(match remainder {
        11 => 1,
        10 => 0,
        other => other as u8,
    })
}

#[cfg(test)]
mod tests {
    use super::{is_valid_dni, is_valid_ruc, ruc_from_dni};

    #[test]
    fn validates_dni() {
        assert!(is_valid_dni("48703474"));
        assert!(!is_valid_dni("4870347"));
        assert!(!is_valid_dni("4870347A"));
    }

    #[test]
    fn derives_ruc_from_dni() {
        assert_eq!(ruc_from_dni("48703474").as_deref(), Some("10487034741"));
    }

    #[test]
    fn validates_ruc_check_digit() {
        assert!(is_valid_ruc("20123456786"));
        assert!(!is_valid_ruc("20123456785"));
    }
}
