//! Writes `src/Cases.mw` for comfyTable.
//!
//! ```text
//! cargo run --release -- <package root>
//! ```
//!
//! Random tables — styles, columns, constraints, arrangements, widths,
//! wrapping, truncation and colours — with what the crate draws for them. The
//! library is ported by hand into `src/`, and the crate's source is
//! fingerprinted, along with the part of crossterm that writes its colours.

use comfy_table::{
    Attribute, Cell, CellAlignment, Color, ColumnConstraint, ContentArrangement, Row, Table,
    TableStyle, Width, presets,
};
use std::fmt::Write as _;
use std::path::PathBuf;

/// The crate version pinned in `Cargo.toml`.
const UPSTREAM_VERSION: &str = "8.0.0";

/// The fingerprint of the sources `src/` ports.
const SOURCES: u64 = 0x9aab_6e10_2625_c0ce;

fn main() {
    let root = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| "../..".into()));

    let print = fingerprint(include_str!(concat!(env!("OUT_DIR"), "/sources.rs.txt")));
    if print != SOURCES {
        eprintln!(
            "error: comfy-table is not the version src/ ports.\n\
             Compare its source in {} with the previous version, carry any change\n\
             into src/, then set SOURCES in scripts/generate/src/main.rs to\n\
             {print:#x}",
            env!("UPSTREAM_DIR")
        );
        std::process::exit(1);
    }

    // Colours are written whatever `NO_COLOR` says.
    crossterm::style::Colored::set_ansi_color_disabled(false);

    let cases = cases();
    let path = root.join("src/Cases.mw");
    std::fs::write(&path, &cases).unwrap();
    eprintln!("wrote {} ({} bytes)", path.display(), cases.len());
}

/// FNV-1a: stable across builds, which `DefaultHasher` does not promise.
fn fingerprint(text: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in text.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

// --- encoding -----------------------------------------------------------------------

/// A number as `digits` base-64 digits, most significant first, each digit the
/// character `'0' + d`: `'0'` to `'o'`, one contiguous run of ASCII.
fn digits(out: &mut String, value: u64, digits: u32) {
    assert!(
        value < 1 << (6 * digits),
        "{value} does not fit in {digits} digits"
    );
    for k in (0..digits).rev() {
        out.push(char::from(b'0' + ((value >> (6 * k)) & 63) as u8));
    }
}

/// A string, as its length in bytes (3 digits) and then its bytes.
fn text(out: &mut String, s: &str) {
    digits(out, s.len() as u64, 3);
    out.push_str(s);
}

fn flag(out: &mut String, b: bool) {
    digits(out, u64::from(b), 1);
}

/// `text` as one Meadow string literal, broken with `\`-newline every `width`
/// characters. Printable ASCII, box-drawing characters, Cyrillic and CJK are
/// written raw, and everything else escaped; a space that would start a line
/// is `\x20`, since a continuation drops leading whitespace.
fn long_literal(text: &str, width: usize) -> String {
    let mut out = String::with_capacity(text.len() + text.len() / width * 4 + 2);
    out.push('"');
    for (i, c) in text.chars().enumerate() {
        let line_start = i > 0 && i % width == 0;
        if line_start {
            out.push_str("\\\n    ");
        }
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '$' => out.push_str("\\$"),
            ' ' if line_start => out.push_str("\\x20"),
            ' '..='~' | '\u{400}'..='\u{4FF}' | '\u{2500}'..='\u{259F}' => out.push(c),
            '\u{3040}'..='\u{30FF}' | '\u{4E00}'..='\u{9FFF}' => out.push(c),
            _ => {
                let _ = write!(out, "\\u{{{:X}}}", u32::from(c));
            }
        }
    }
    out.push('"');
    out
}

// --- inputs -------------------------------------------------------------------------

/// A small deterministic generator, so that the cases are the same on every run.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }

    fn pick<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[self.below(xs.len() as u64) as usize]
    }
}

const PRESETS: [TableStyle; 12] = [
    presets::ASCII_FULL,
    presets::ASCII_FULL_CONDENSED,
    presets::ASCII_NO_BORDERS,
    presets::ASCII_BORDERS_ONLY,
    presets::ASCII_BORDERS_ONLY_CONDENSED,
    presets::ASCII_HORIZONTAL_ONLY,
    presets::ASCII_MARKDOWN,
    presets::UTF8_FULL,
    presets::UTF8_FULL_CONDENSED,
    presets::UTF8_NO_BORDERS,
    presets::UTF8_BORDERS_ONLY,
    presets::UTF8_HORIZONTAL_ONLY,
];

const COLORS: [Color; 17] = [
    Color::Reset,
    Color::Black,
    Color::DarkGrey,
    Color::Red,
    Color::DarkRed,
    Color::Green,
    Color::DarkGreen,
    Color::Yellow,
    Color::DarkYellow,
    Color::Blue,
    Color::DarkBlue,
    Color::Magenta,
    Color::DarkMagenta,
    Color::Cyan,
    Color::DarkCyan,
    Color::White,
    Color::Grey,
];

const ATTRIBUTES: [Attribute; 28] = [
    Attribute::Reset,
    Attribute::Bold,
    Attribute::Dim,
    Attribute::Italic,
    Attribute::Underlined,
    Attribute::DoubleUnderlined,
    Attribute::Undercurled,
    Attribute::Underdotted,
    Attribute::Underdashed,
    Attribute::SlowBlink,
    Attribute::RapidBlink,
    Attribute::Reverse,
    Attribute::Hidden,
    Attribute::CrossedOut,
    Attribute::Fraktur,
    Attribute::NoBold,
    Attribute::NormalIntensity,
    Attribute::NoItalic,
    Attribute::NoUnderline,
    Attribute::NoBlink,
    Attribute::NoReverse,
    Attribute::NoHidden,
    Attribute::NotCrossedOut,
    Attribute::Framed,
    Attribute::Encircled,
    Attribute::OverLined,
    Attribute::NotFramedOrEncircled,
    Attribute::NotOverLined,
];

const WORDS: &[&str] = &[
    "a",
    "an",
    "the",
    "table",
    "header",
    "content",
    "comfy",
    "wrapping",
    "supercalifragilisticexpialidocious",
    "0123456789012345678901234567890",
    "日本語",
    "テキスト",
    "e\u{301}l\u{e8}ve",
    "\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467}",
    "\u{1f600}\u{1f600}\u{1f600}",
    "x-y-z",
    "a,b,c",
    "tab\there",
    "",
    "\u{301}",
    "Здравствуйте",
    "…",
];

const DELIMITERS: [char; 6] = [' ', '-', ',', '日', '|', 'e'];

const LINE_CHARS: [char; 8] = ['#', '*', '═', '│', ' ', '▓', 'x', '╳'];

/// Text for a cell: words joined by spaces and other delimiters, perhaps over
/// several lines.
fn random_text(rng: &mut Rng) -> String {
    let n = rng.below(7);
    let mut s = String::new();
    for i in 0..n {
        if i > 0 {
            s.push(match rng.below(10) {
                0 => '\n',
                1 => '-',
                2 => ',',
                3 => '日',
                _ => ' ',
            });
        }
        s.push_str(rng.pick(WORDS));
    }
    s
}

fn random_color(rng: &mut Rng, out: &mut String) -> Option<Color> {
    if !rng.chance(25) {
        digits(out, 0, 1);
        return None;
    }
    match rng.below(3) {
        0 => {
            let i = rng.below(17);
            digits(out, 1, 1);
            digits(out, i, 1);
            Some(COLORS[i as usize])
        }
        1 => {
            let [r, g, b] = [rng.below(256), rng.below(256), rng.below(256)];
            digits(out, 2, 1);
            for c in [r, g, b] {
                digits(out, c, 2);
            }
            Some(Color::Rgb {
                r: r as u8,
                g: g as u8,
                b: b as u8,
            })
        }
        _ => {
            let v = rng.below(256);
            digits(out, 3, 1);
            digits(out, v, 2);
            Some(Color::AnsiValue(v as u8))
        }
    }
}

fn random_char(rng: &mut Rng, out: &mut String, pool: &[char]) -> char {
    let c = rng.pick(pool);
    text(out, &c.to_string());
    c
}

fn random_cell(rng: &mut Rng, out: &mut String) -> Cell {
    let content = random_text(rng);
    text(out, &content);
    let mut cell = Cell::new(&content);
    let has_delimiter = rng.chance(15);
    flag(out, has_delimiter);
    if has_delimiter {
        cell = cell.set_delimiter(random_char(rng, out, &DELIMITERS));
    }
    let alignment = if rng.chance(30) { rng.below(3) + 1 } else { 0 };
    digits(out, alignment, 1);
    if alignment > 0 {
        cell = cell.set_alignment(
            [
                CellAlignment::Left,
                CellAlignment::Right,
                CellAlignment::Center,
            ][alignment as usize - 1],
        );
    }
    if let Some(c) = random_color(rng, out) {
        cell = cell.fg(c);
    }
    if let Some(c) = random_color(rng, out) {
        cell = cell.bg(c);
    }
    let attributes = if rng.chance(20) { rng.below(4) } else { 0 };
    digits(out, attributes, 1);
    for _ in 0..attributes {
        let i = rng.below(28);
        digits(out, i, 1);
        cell = cell.add_attribute(ATTRIBUTES[i as usize]);
    }
    cell
}

fn random_row(rng: &mut Rng, out: &mut String, columns: u64) -> Row {
    let max_height = if rng.chance(20) {
        Some(rng.below(4) as usize)
    } else {
        None
    };
    flag(out, max_height.is_some());
    if let Some(h) = max_height {
        digits(out, h as u64, 1);
    }
    let n = if rng.chance(15) {
        rng.below(columns + 2)
    } else {
        columns
    };
    digits(out, n, 1);
    let cells: Vec<Cell> = (0..n).map(|_| random_cell(rng, out)).collect();
    let mut row = Row::from(cells);
    if let Some(h) = max_height {
        row.max_height(h);
    }
    row
}

fn random_width(rng: &mut Rng, out: &mut String) -> Width {
    let percent = rng.chance(40);
    flag(out, percent);
    if percent {
        let p = rng.below(121);
        digits(out, p, 2);
        Width::Percentage(p as u16)
    } else {
        let w = rng.below(30);
        digits(out, w, 2);
        Width::Fixed(w as u16)
    }
}

fn random_constraint(rng: &mut Rng, out: &mut String) -> ColumnConstraint {
    let kind = rng.below(6);
    digits(out, kind, 1);
    match kind {
        0 => ColumnConstraint::Hidden,
        1 => ColumnConstraint::ContentWidth,
        2 => ColumnConstraint::Absolute(random_width(rng, out)),
        3 => ColumnConstraint::LowerBoundary(random_width(rng, out)),
        4 => ColumnConstraint::UpperBoundary(random_width(rng, out)),
        _ => {
            let lower = random_width(rng, out);
            let upper = random_width(rng, out);
            ColumnConstraint::Boundaries { lower, upper }
        }
    }
}

/// A style: a preset, perhaps rounded or with solid inner lines, and some
/// characters changed.
fn random_style(rng: &mut Rng, out: &mut String) -> TableStyle {
    let preset = rng.below(PRESETS.len() as u64 + 1);
    digits(out, preset, 1);
    let mut style = PRESETS
        .get(preset as usize)
        .copied()
        .unwrap_or(presets::NOTHING);
    let rounded = rng.chance(20);
    let solid = rng.chance(20);
    flag(out, rounded);
    flag(out, solid);
    if rounded {
        style = style.with_rounded_corners();
    }
    if solid {
        style = style.with_solid_inner_borders();
    }
    let overrides = if rng.chance(30) { rng.below(4) } else { 0 };
    digits(out, overrides, 1);
    for _ in 0..overrides {
        let part = rng.below(6);
        digits(out, part, 1);
        let is_content = part == 1 || part == 3;
        let slot = rng.below(if is_content { 3 } else { 4 });
        digits(out, slot, 1);
        let value = if rng.chance(70) {
            flag(out, true);
            Some(random_char(rng, out, &LINE_CHARS))
        } else {
            flag(out, false);
            None
        };
        if is_content {
            let line = if part == 1 {
                &mut style.header_lines
            } else {
                &mut style.content_lines
            };
            *[&mut line.left, &mut line.junction, &mut line.right][slot as usize] = value;
        } else {
            let line = match part {
                0 => &mut style.top_border,
                2 => &mut style.header_separator,
                4 => &mut style.row_separator,
                _ => &mut style.bottom_border,
            };
            *[
                &mut line.left,
                &mut line.fill,
                &mut line.junction,
                &mut line.right,
            ][slot as usize] = value;
        }
    }
    style
}

fn random_table(rng: &mut Rng, out: &mut String) -> Table {
    let mut table = Table::new();
    table.force_no_tty();
    table.load_style(random_style(rng, out));
    let arrangement = rng.below(3);
    digits(out, arrangement, 1);
    table.set_content_arrangement(
        [
            ContentArrangement::Disabled,
            ContentArrangement::Dynamic,
            ContentArrangement::DynamicFullWidth,
        ][arrangement as usize]
            .clone(),
    );
    let has_width = rng.chance(75);
    flag(out, has_width);
    if has_width {
        let w = if rng.chance(10) {
            rng.below(4)
        } else {
            rng.below(70)
        };
        digits(out, w, 2);
        table.set_width(w as u16);
    }
    let has_delimiter = rng.chance(15);
    flag(out, has_delimiter);
    if has_delimiter {
        table.set_delimiter(random_char(rng, out, &DELIMITERS));
    }
    let has_indicator = rng.chance(20);
    flag(out, has_indicator);
    if has_indicator {
        let indicator = rng.pick(&["...", "", "»", "[more]", "日"]);
        text(out, indicator);
        table.set_truncation_indicator(indicator);
    }
    let styled = rng.chance(30);
    flag(out, styled);
    if styled {
        table.enforce_styling();
    }
    let text_only = rng.chance(30);
    flag(out, text_only);
    if text_only {
        table.style_text_only();
    }

    let columns = rng.below(6) + 1;
    let has_header = rng.chance(70);
    flag(out, has_header);
    if has_header {
        let header = random_row(rng, out, columns);
        table.set_header(header);
    }
    let rows = rng.below(5);
    digits(out, rows, 1);
    for _ in 0..rows {
        let row = random_row(rng, out, columns);
        table.add_row(row);
    }

    let ops = if rng.chance(50) { rng.below(6) } else { 0 };
    digits(out, ops, 1);
    for _ in 0..ops {
        let index = rng.below(columns + 1);
        digits(out, index, 1);
        let op = rng.below(5);
        digits(out, op, 1);
        // What an op needs is written whether or not the column exists.
        match op {
            0 => {
                let (l, r) = (rng.below(4), rng.below(4));
                digits(out, l, 1);
                digits(out, r, 1);
                if let Some(c) = table.column_mut(index as usize) {
                    c.set_padding((l as u16, r as u16));
                }
            }
            1 => {
                let d = random_char(rng, out, &DELIMITERS);
                if let Some(c) = table.column_mut(index as usize) {
                    c.set_delimiter(d);
                }
            }
            2 => {
                let a = rng.below(3);
                digits(out, a, 1);
                if let Some(c) = table.column_mut(index as usize) {
                    c.set_cell_alignment(
                        [
                            CellAlignment::Left,
                            CellAlignment::Right,
                            CellAlignment::Center,
                        ][a as usize],
                    );
                }
            }
            3 => {
                let k = random_constraint(rng, out);
                if let Some(c) = table.column_mut(index as usize) {
                    c.set_constraint(k);
                }
            }
            _ => {
                if let Some(c) = table.column_mut(index as usize) {
                    c.remove_constraint();
                }
            }
        }
    }
    // Now and then every column is hidden, which leaves rows with no parts.
    let all_hidden = rng.chance(4);
    let constraints = if all_hidden {
        table.column_count() as u64
    } else if rng.chance(25) {
        rng.below(5)
    } else {
        0
    };
    digits(out, constraints, 1);
    let list: Vec<ColumnConstraint> = (0..constraints)
        .map(|_| {
            if all_hidden {
                digits(out, 0, 1);
                ColumnConstraint::Hidden
            } else {
                random_constraint(rng, out)
            }
        })
        .collect();
    table.set_constraints(list);
    table
}

// --- cases --------------------------------------------------------------------------

fn cases() -> String {
    let mut rng = Rng(0xc0f7_7ab1_e0ff_ee42);
    let mut body = String::new();
    let count = 1000;
    for k in 0..count {
        let mut table = random_table(&mut rng, &mut body);
        let widths: Vec<String> = table
            .column_max_content_widths()
            .iter()
            .map(u16::to_string)
            .collect();
        let summary = format!(
            "{} {} {} {}",
            widths.join(","),
            table.column_count(),
            table.row_count(),
            table.is_empty()
        );
        text(&mut body, &table.to_string());
        // Trimming is simple, and a few tables are enough to check it.
        let trimmed = k % 8 == 0;
        flag(&mut body, trimmed);
        if trimmed {
            text(&mut body, &table.trim_fmt());
        }
        text(&mut body, &summary);
    }

    let mut out = String::new();
    let _ = writeln!(
        out,
        "-- GENERATED by scripts/generate.sh from comfy-table {UPSTREAM_VERSION}.
-- Do not edit: run the script again instead.
--
-- Inputs, with what the crate makes of them, for `Tests.mw`: {count} random tables.
--
-- Copyright Arne Beer and the comfy-table contributors, and the Meadow port's
-- authors. Licensed under MIT: see LICENSE and COPYRIGHT.

-- Each table is written as the steps that build it, in the order `Tests.mw`
-- reads and takes them, and then what the crate draws: the table, whether
-- the trimmed table follows and, if so, that, and the column widths and
-- counts. A number is base-64 digits; a
-- string is its length in bytes (3 digits) and then its bytes.
@cfg(test)
@pub(pkg) def cases =
  {}",
        long_literal(&body, 96)
    );
    out
}
