use kale_syntax::span::Span;
use crate::diagnostic::Diagnostic;

pub struct Report<'a> {
    source: &'a str,
}

impl<'a> Report<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn render<T: Diagnostic>(&self, diagnostic: &T) -> String {
        /*
        error: <msg>
         --> <line>:<col>
              |
          <l> | <code> ... <code>
              |            ^^^^^^
          <l> | <code> ... <code>
              | ^^^^^^
              |
         */

        // we assume that the given `self.source` and `diagnostic` match
        // for simplicity. mismatches will most likely panic due to invalid
        // utf-8 boundaries

        let mut out = String::new();
        let loc = Loc::at(self.source, diagnostic.span());

        out.push_str(&format!("error: {}\n", diagnostic.message()));
        out.push_str(&format!(" --> {}:{}\n", loc.lines[0].number, loc.start_col));

        // <l> width in the gutter, so multi-lined errors will not
        // mess up the gutter by being a digit too big or small
        let line_num_width = loc.lines
            .last()
            .expect("`Loc.lines` should contain at least 1 line")
            .number
            .to_string()
            .len();

        let gutter = " ".repeat(line_num_width + 2);
        out.push_str(&format!("{gutter}|\n"));

        for (i, line) in loc.lines.iter().enumerate() {
            // print the loc
            out.push_str(&format!(
                " {:>width$} | {}\n",
                line.number, line.text, width = line_num_width
            ));

            let num_lines = loc.lines.len();
            // we must use char-based lengths, not byte-based so columns
            // are correctly handled
            let text_len = line.text.chars().count();

            let (caret_start, caret_len) = if num_lines == 1 {
                // case 1: single line
                (loc.start_col - 1, loc.width)
            } else if i == 0 {
                // case 2: first line of many
                let caret_start = loc.start_col - 1;
                // only underline until the end of this line
                (caret_start, text_len - caret_start)
            } else if i == num_lines - 1 {
                // case 3: last line of many
                (0, loc.end_col - 1)
            } else {
                // case 4: middle line of many
                (0, text_len)
            };

            // print the carets
            // use `.max(1)` to ensure empty spans get at least one `^`
            out.push_str(&format!(
                " {:>width$} | {}{}\n",
                "",
                " ".repeat(caret_start),
                "^".repeat(caret_len.max(1)),
                width = line_num_width,
            ));
        }

        out.push_str(&format!("{gutter}|\n"));
        out
    }
}

struct Loc<'a> {
    lines: Vec<Line<'a>>,

    // both `start_col` and `end_col` are 1-indexed and measured
    // in chars, not byte offsets, so columns are measured correctly
    // for non-ascii utf-8 characters
    start_col: usize, // start col on first line
    end_col: usize, // end col on last line

    // measured in chars
    width: usize, // span width
}

impl<'a> Loc<'a> {
    fn at(source: &'a str, span: Span) -> Self {
        let (start, end) = span.into_parts();

        let mut lines = Vec::new();
        let mut offset = 0;

        // use `split_inclusive('\n')` to keep the trailing `\n` so offsets
        // are calculated correctly
        for (i, line) in source.split_inclusive('\n').enumerate() {
            let number = i + 1;

            // now trim `\n` and `\r` for visual display
            let text = line.trim_end_matches('\n').trim_end_matches('\r');

            // because we used `split_inclusive`, `line.len()` includes the newline,
            // so offset now correctly advances through `source`
            let line_end = offset + line.len();

            // check if this line overlaps with `span`
            if start < line_end && end > offset {
                lines.push(Line { number, text });
            }

            offset = line_end;

            // optimization: `stop looking if we've passed the span
            if offset > end { break; }
        }

        assert!(!lines.is_empty(), "no lines found for span {span}");

        // allow panicking on slicing the string at invalid utf-8 boundaries
        // as this would be the result of a mismatch between the given
        // `source` and `span`, and this is considered unrecoverable
        // and not feasible to catch

        // the byte offset of the start of the first line in the span
        let first_line = source[..start]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        // the byte offset of the start of the last line in the span
        let last_line = source[..end]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        // +1 as columns are 1-indexed
        let start_col = source[first_line..start]
            .chars()
            .count() + 1;
        let end_col = source[last_line..end]
            .chars()
            .count() + 1;

        let width = source[start..end]
            .chars()
            .count();

        Self { lines, start_col, end_col, width }
    }
}

struct Line<'a> {
    number: usize, // 1-indexed line number
    text: &'a str,
}
