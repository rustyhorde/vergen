// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// rkyv ArchiveWith wrappers for console::Style and tracing::Level

#[cfg(feature = "color")]
use console::Style;
use rkyv::{
    Place, SerializeUnsized,
    rancor::{Fallible, Source},
    string::{ArchivedString, StringResolver},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
};
#[cfg(feature = "trace")]
use tracing::Level;

#[cfg(feature = "color")]
/// rkyv [`ArchiveWith`] wrapper for [`console::Style`].
///
/// Serializes a [`Style`] as its dotted attribute string (e.g.
/// `"bold.red.on_blue"`), reconstructable via [`Style::from_dotted_str`].
///
/// The conversion works by forcing ANSI escape code emission on a clone of
/// the style, extracting the codes from the output, and mapping them back to
/// their named dotted-format equivalents.  The roundtrip is functionally
/// lossless: the deserialized style produces identical terminal output, though
/// the internal representation may differ for edge cases such as 256-color
/// indices 8–15 which overlap with bright basic colors.
///
/// Use with `#[rkyv(with = rkyv::with::Map<StyleWith>)]` on `Option<Style>`
/// fields.
#[cfg(feature = "color")]
#[derive(Clone, Copy, Debug)]
pub struct StyleWith;

#[cfg(feature = "color")]
/// Convert a [`Style`] to its dotted attribute string representation.
///
/// Clones and forces styling so that ANSI escape sequences are always emitted
/// regardless of the current terminal, then parses those sequences back into
/// the `"bold.red.on_blue"` format understood by [`Style::from_dotted_str`].
fn style_to_dotted(style: &Style) -> String {
    #[allow(clippy::items_after_statements)]
    let raw = style.clone().force_styling(true).apply_to("").to_string();
    if raw.is_empty() {
        return String::new();
    }
    let mut parts: Vec<String> = Vec::new();
    let mut chars = raw.chars();
    while let Some(ch) = chars.next() {
        if ch != '\x1b' {
            continue;
        }
        if chars.next() != Some('[') {
            continue;
        }
        let mut code = String::new();
        for c in chars.by_ref() {
            if c == 'm' {
                break;
            }
            code.push(c);
        }
        // "0" is the reset marker that terminates the style prefix.
        if code == "0" {
            break;
        }
        push_dotted_parts(&code, &mut parts);
    }
    parts.join(".")
}

#[cfg(feature = "color")]
/// Parse one ANSI SGR parameter string (e.g. `"31"`, `"38;5;196"`,
/// `"48;2;255;0;128"`) and push the corresponding dotted format part(s).
fn push_dotted_parts(code: &str, parts: &mut Vec<String>) {
    #[allow(clippy::items_after_statements)]
    let segs: Vec<&str> = code.split(';').collect();
    match segs.as_slice() {
        // Single numeric code
        [n_str] => {
            let Ok(n) = n_str.parse::<u8>() else { return };
            match n {
                // Attributes: Bold=1 … StrikeThrough=9
                1..=9 => {
                    const ATTRS: [&str; 9] = [
                        "bold",
                        "dim",
                        "italic",
                        "underlined",
                        "blink",
                        "blink_fast",
                        "reverse",
                        "hidden",
                        "strikethrough",
                    ];
                    parts.push(ATTRS[(n - 1) as usize].to_string());
                }
                // Basic foreground colors: 30 (Black) … 37 (White)
                30..=37 => {
                    const FG: [&str; 8] = [
                        "black", "red", "green", "yellow", "blue", "magenta",
                        "cyan", "white",
                    ];
                    parts.push(FG[(n - 30) as usize].to_string());
                }
                // Basic background colors: 40 (Black) … 47 (White)
                40..=47 => {
                    const BG: [&str; 8] = [
                        "on_black", "on_red", "on_green", "on_yellow",
                        "on_blue", "on_magenta", "on_cyan", "on_white",
                    ];
                    parts.push(BG[(n - 40) as usize].to_string());
                }
                _ => {}
            }
        }
        // Foreground 256-color or bright-basic: ESC[38;5;Nm
        // Bright-basic (8–15) will be stored as their 256-color index, which
        // round-trips to the same visual output via `from_dotted_str`.
        ["38", "5", n_str] => {
            if let Ok(n) = n_str.parse::<u8>() {
                parts.push(n.to_string());
            }
        }
        // Foreground true-color: ESC[38;2;R;G;Bm  →  "#RRGGBB"
        ["38", "2", r_str, g_str, b_str] => {
            if let (Ok(r), Ok(g), Ok(b)) = (
                r_str.parse::<u8>(),
                g_str.parse::<u8>(),
                b_str.parse::<u8>(),
            ) {
                parts.push(format!("#{r:02X}{g:02X}{b:02X}"));
            }
        }
        // Background 256-color or bright-basic: ESC[48;5;Nm  →  "on_N"
        ["48", "5", n_str] => {
            if let Ok(n) = n_str.parse::<u8>() {
                parts.push(format!("on_{n}"));
            }
        }
        // Background true-color: ESC[48;2;R;G;Bm  →  "on_#RRGGBB"
        ["48", "2", r_str, g_str, b_str] => {
            if let (Ok(r), Ok(g), Ok(b)) = (
                r_str.parse::<u8>(),
                g_str.parse::<u8>(),
                b_str.parse::<u8>(),
            ) {
                parts.push(format!("on_#{r:02X}{g:02X}{b:02X}"));
            }
        }
        _ => {}
    }
}

#[cfg(feature = "color")]
impl ArchiveWith<Style> for StyleWith {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    fn resolve_with(
        field: &Style,
        resolver: Self::Resolver,
        out: Place<Self::Archived>,
    ) {
        ArchivedString::resolve_from_str(&style_to_dotted(field), resolver, out);
    }
}

#[cfg(feature = "color")]
impl<S> SerializeWith<Style, S> for StyleWith
where
    S: Fallible + ?Sized,
    S::Error: Source,
    str: SerializeUnsized<S>,
{
    fn serialize_with(
        field: &Style,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(&style_to_dotted(field), serializer)
    }
}

#[cfg(feature = "color")]
impl<D> DeserializeWith<ArchivedString, Style, D> for StyleWith
where
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedString,
        _: &mut D,
    ) -> Result<Style, D::Error> {
        Ok(Style::from_dotted_str(field.as_str()))
    }
}

// ── LevelWith ────────────────────────────────────────────────────────────────

/// rkyv [`ArchiveWith`] wrapper for [`tracing::Level`].
///
/// Serializes a [`Level`] as its uppercase name string (`"TRACE"`, `"DEBUG"`,
/// `"INFO"`, `"WARN"`, `"ERROR"`), reconstructable via a simple match.
///
/// Use with `#[rkyv(with = LevelWith)]` on `Level` fields.
#[cfg(feature = "trace")]
#[derive(Clone, Copy, Debug)]
pub(crate) struct LevelWith;

#[cfg(feature = "trace")]
impl ArchiveWith<Level> for LevelWith {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    fn resolve_with(
        field: &Level,
        resolver: Self::Resolver,
        out: Place<Self::Archived>,
    ) {
        ArchivedString::resolve_from_str(field.as_str(), resolver, out);
    }
}

#[cfg(feature = "trace")]
impl<S> SerializeWith<Level, S> for LevelWith
where
    S: Fallible + ?Sized,
    S::Error: Source,
    str: SerializeUnsized<S>,
{
    fn serialize_with(
        field: &Level,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(field.as_str(), serializer)
    }
}

#[cfg(feature = "trace")]
impl<D: Fallible + ?Sized> DeserializeWith<ArchivedString, Level, D> for LevelWith {
    fn deserialize_with(
        field: &ArchivedString,
        _: &mut D,
    ) -> Result<Level, D::Error> {
        Ok(match field.as_str() {
            "TRACE" => Level::TRACE,
            "DEBUG" => Level::DEBUG,
            "WARN" => Level::WARN,
            "ERROR" => Level::ERROR,
            _ => Level::INFO,
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    #[cfg(feature = "color")]
    use super::style_to_dotted;
    #[cfg(feature = "color")]
    use console::Style;

    #[cfg(feature = "color")]
    #[test]
    fn empty_style_round_trips() {
        let s = Style::new();
        assert_eq!(style_to_dotted(&s), "");
        let _ = Style::from_dotted_str(&style_to_dotted(&s));
    }

    #[cfg(feature = "color")]
    #[test]
    fn basic_fg_color_round_trips() {
        for (style, expected) in [
            (Style::new().black(), "black"),
            (Style::new().red(), "red"),
            (Style::new().green(), "green"),
            (Style::new().yellow(), "yellow"),
            (Style::new().blue(), "blue"),
            (Style::new().magenta(), "magenta"),
            (Style::new().cyan(), "cyan"),
            (Style::new().white(), "white"),
        ] {
            assert_eq!(style_to_dotted(&style), expected);
        }
    }

    #[cfg(feature = "color")]
    #[test]
    fn basic_bg_color_round_trips() {
        assert_eq!(style_to_dotted(&Style::new().on_red()), "on_red");
        assert_eq!(style_to_dotted(&Style::new().on_blue()), "on_blue");
    }

    #[cfg(feature = "color")]
    #[test]
    fn attrs_round_trips() {
        assert_eq!(style_to_dotted(&Style::new().bold()), "bold");
        assert_eq!(style_to_dotted(&Style::new().underlined()), "underlined");
        assert_eq!(style_to_dotted(&Style::new().italic()), "italic");
        assert_eq!(
            style_to_dotted(&Style::new().strikethrough()),
            "strikethrough"
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn compound_style_round_trips() {
        // bold + green; serialized as "green.bold" (fg first, attrs second)
        let s = Style::new().bold().green();
        let dotted = style_to_dotted(&s);
        assert!(dotted.contains("green"));
        assert!(dotted.contains("bold"));
        let restored = Style::from_dotted_str(&dotted);
        // both styles should produce the same ANSI output
        assert_eq!(
            s.force_styling(true).apply_to("x").to_string(),
            restored.force_styling(true).apply_to("x").to_string(),
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn true_color_round_trips() {
        let s = Style::new().true_color(0xFF, 0x00, 0x80);
        let dotted = style_to_dotted(&s);
        assert_eq!(dotted, "#FF0080");
        let restored = Style::from_dotted_str(&dotted);
        assert_eq!(
            s.force_styling(true).apply_to("x").to_string(),
            restored.force_styling(true).apply_to("x").to_string(),
        );
    }

    #[cfg(feature = "color")]
    #[test]
    fn color256_round_trips() {
        let s = Style::new().color256(200);
        let dotted = style_to_dotted(&s);
        assert_eq!(dotted, "200");
        let restored = Style::from_dotted_str(&dotted);
        assert_eq!(
            s.force_styling(true).apply_to("x").to_string(),
            restored.force_styling(true).apply_to("x").to_string(),
        );
    }

    #[cfg(feature = "trace")]
    #[test]
    fn level_as_str_round_trips() {
        use tracing::Level;
        for (level, expected) in [
            (Level::TRACE, "TRACE"),
            (Level::DEBUG, "DEBUG"),
            (Level::INFO, "INFO"),
            (Level::WARN, "WARN"),
            (Level::ERROR, "ERROR"),
        ] {
            assert_eq!(level.as_str(), expected);
        }
    }

    #[cfg(feature = "trace")]
    #[test]
    fn level_default_fallback() {
        use super::LevelWith;
        use tracing::Level;
        let levels = [
            Level::TRACE,
            Level::DEBUG,
            Level::INFO,
            Level::WARN,
            Level::ERROR,
        ];
        for level in levels {
            assert_eq!(level.as_str().parse::<Level>().unwrap(), level);
        }
        let _ = LevelWith;
    }
}
