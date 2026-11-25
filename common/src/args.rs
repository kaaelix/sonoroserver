//! Positional-flag CLI parser matching the original `common.args` behavior.

use std::collections::VecDeque;

/// Specification for a single short-flag CLI option (first letter of `name`).
#[derive(Debug, Clone, Copy)]
pub struct FlagSpec {
    pub flag: char,
    pub name: &'static str,
}

/// Parses short flags (`-s val` or grouped `-sl val1 val2`) into `apply`.
///
/// Returns `false` if an unknown flag, missing value, or unexpected positional argument is seen.
pub fn parse_flags<F>(args: &[String], specs: &[FlagSpec], mut apply: F) -> bool
where
    F: FnMut(char, &str) -> bool,
{
    let mut pending: VecDeque<char> = VecDeque::new();

    for arg in args {
        if let Some(rest) = arg.strip_prefix('-') {
            if rest.is_empty() {
                return false;
            }
            for ch in rest.chars() {
                if pending.len() >= specs.len() || !specs.iter().any(|s| s.flag == ch) {
                    return false;
                }
                pending.push_back(ch);
            }
        } else {
            let Some(flag) = pending.pop_front() else {
                return false;
            };
            if !apply(flag, arg) {
                return false;
            }
        }
    }

    pending.is_empty()
}

/// Formats a usage string matching `Usage: <program> [-abc] [field_a] [field_b] [field_c]`.
pub fn format_usage(program_name: &str, specs: &[FlagSpec]) -> String {
    let flags: String = specs.iter().map(|s| s.flag).collect();
    let names: Vec<String> = specs.iter().map(|s| format!("[{}]", s.name)).collect();
    format!("Usage: {program_name} [-{flags}] {}", names.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPECS: &[FlagSpec] = &[
        FlagSpec {
            flag: 's',
            name: "state_dir",
        },
        FlagSpec {
            flag: 'l',
            name: "listen_address",
        },
    ];

    #[test]
    fn parses_separate_and_grouped_flags() {
        let mut state = String::from("state");
        let mut listen = String::from("127.0.0.1:5500");
        let args = vec![
            "-sl".to_string(),
            "/tmp/s".to_string(),
            "0.0.0.0:5501".to_string(),
        ];

        assert!(parse_flags(&args, SPECS, |flag, val| {
            match flag {
                's' => state = val.to_string(),
                'l' => listen = val.to_string(),
                _ => return false,
            }
            true
        }));
        assert_eq!(state, "/tmp/s");
        assert_eq!(listen, "0.0.0.0:5501");
    }

    #[test]
    fn rejects_missing_value_or_unknown_flag() {
        assert!(!parse_flags(&["-s".to_string()], SPECS, |_, _| true));
        assert!(!parse_flags(
            &["-x".to_string(), "val".to_string()],
            SPECS,
            |_, _| true
        ));
        assert!(!parse_flags(&["extra".to_string()], SPECS, |_, _| true));
    }
}
