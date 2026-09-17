# comfyTable

Tables for the terminal, for [Meadow](https://github.com/mcdearman/meadow). It
draws borders, aligns text, colours cells, and wraps content to fit a width.

This package is a port of Rust's
[`comfy-table`](https://github.com/nukesor/comfy-table) 8.0.0. It lays tables
out and draws them character for character as the crate does. Colours are
written with the same escape sequences as crossterm 0.29, which the crate uses.

## Install

```sh
meadow add mcdearman/meadow-comfy-table
```

## Use

```meadow
use comfyTable

def main =
  table
    |> loadStyle (withRoundedCorners utf8Full)
    |> setContentArrangement Dynamic
    |> setWidth 40
    |> setHeader (rowOf ["Name", "Role", "Notes"])
    |> addRow (rowOf ["Ada", "Engineer", "Wrote the first published program"])
    |> addRow (row [cell "Grace", cell "Admiral", cell "Found a moth" |> setAlignment AlignRight])
    |> updateColumn 1 (setConstraint (UpperBoundary (Fixed 10)))
    |> render
```

```text
╭───────┬──────────┬───────────────────╮
│ Name  ┆ Role     ┆ Notes             │
╞═══════╪══════════╪═══════════════════╡
│ Ada   ┆ Engineer ┆ Wrote the first   │
│       ┆          ┆ published program │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Grace ┆ Admiral  ┆      Found a moth │
╰───────┴──────────┴───────────────────╯
```

The crate changes a table in place. Here every function returns a new table,
taking the table last so that calls chain with `|>`.

### Tables

- Start from `table`, which is empty and drawn with `asciiFull`.
- Add content with `setHeader`, `addRow`, `addRows`, `addRowIf` and
  `addRowsIf`. Columns are created as rows need them.
- Draw with `render`, which joins the lines with newlines, or with `lines`.
  `trimFmt` renders without trailing spaces.
- Configure with:
  - `setWidth`;
  - `setContentArrangement` with `Disabled`, `Dynamic` or `DynamicFullWidth`;
  - `setDelimiter`, the character long lines may be split at;
  - `setTruncationIndicator`, which is `…` by default.
- Query with `rowCount`, `isEmpty`, `columnCount`, `columnMaxContentWidths`,
  `getRow`, `getColumn`, `columnCells` and `columnCellsWithHeader`.
- Change existing parts with `updateRow k f`, `updateColumn k f` and
  `setConstraints [..]`.

The crate reads the terminal to find out whether to style a table and how wide
it is. This port does neither:

- a table is only as wide as `setWidth` says, and without a width it is laid
  out as if arrangement were `Disabled`;
- colours and attributes are written only after `enforceStyling`, and
  `setStyleTextOnly` keeps them off the padding.

### Rows, cells and columns

- **Rows:** `row [cells]` or `rowOf [texts]`; change them with `addCell` and
  `setMaxHeight n`. A row with a maximum height cuts its cells short and ends
  them with the truncation indicator.
- **Cells:** `cell text`, where a newline starts a new line. Change them with:
  - `setAlignment` (`AlignLeft`, `AlignRight` or `AlignCenter`);
  - `setCellDelimiter`;
  - `setFg` and `setBg` with a `Color`: the named colours, `Rgb r g b` or
    `AnsiValue n`;
  - `addAttribute` and `addAttributes` with an `Attribute`, such as
    `Attribute.Bold`.
- **Columns** are changed through `updateColumn` with:
  - `setPadding (left, right)`;
  - `setColumnDelimiter`;
  - `setCellAlignment`;
  - `setConstraint` and `removeConstraint`.

A constraint is one of `Hidden`, `ContentWidth`, `Absolute w`,
`LowerBoundary w`, `UpperBoundary w` or `Boundaries lower upper`, where `w` is
`Fixed n` or `Percentage p` of the table's width.

### Styles

The presets are:

- `asciiFull`, `asciiFullCondensed`, `asciiNoBorders`, `asciiBordersOnly`,
  `asciiBordersOnlyCondensed`, `asciiHorizontalOnly` and `asciiMarkdown`;
- `utf8Full`, `utf8FullCondensed`, `utf8NoBorders`, `utf8BordersOnly` and
  `utf8HorizontalOnly`;
- `nothing`, which draws no lines.

`withRoundedCorners` and `withSolidInnerBorders` change a preset.

A `TableStyle` is a record of six lines: `topBorder`, `headerLines`,
`headerSeparator`, `contentLines`, `rowSeparator` and `bottomBorder`. Build
lines with `lineStyle`, `noLine`, `contentLineStyle` and `noContentLine`, or
with record updates. Then pass the result to `loadStyle`, or change the
current style with `modifyStyle`.

Because constructor names clash, `Attribute` constructors are not exported
unqualified: write `Attribute.Bold`. `CellAlignment` constructors are
`AlignLeft`, `AlignRight` and `AlignCenter`, so that they do not collide with
`Either`'s `Left` and `Right`.

## How it's made

The modules in `src/` are hand translations of the crate's layout, splitting,
formatting and border drawing. Text widths come from
[unicodeWidth](https://github.com/mcdearman/meadow-unicode-width) and graphemes
from [unicodeSegmentation](https://github.com/mcdearman/meadow-unicode-segmentation),
which port the same crate versions that comfy-table 8.0.0 uses.

**`src/Cases.mw`** holds 1,000 random tables. Each one records the steps that
build it and what the crate draws for it. The tables cover every preset and
arrangement, as well as:

- constraints, padding, delimiters and alignment;
- colours and attributes;
- row heights with truncation;
- wide, combining and emoji text.

Run `scripts/generate.sh` to regenerate; it needs a Rust toolchain.

## Licence

MIT, like comfy-table and crossterm: see [LICENSE](LICENSE),
[LICENSE-crossterm](LICENSE-crossterm) and [COPYRIGHT](COPYRIGHT).
