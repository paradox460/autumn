use crate::themes::Theme;
use inkjet::{constants, formatter::Formatter, Result};
use tree_sitter_highlight::HighlightEvent;

#[derive(Debug)]
pub struct InlineHTML<'a> {
    lang: &'a str,
    theme: &'a Theme,
    pre_class: &'a str,
}

impl<'a> InlineHTML<'a> {
    pub fn new(lang: &'a str, theme: &'a Theme, pre_class: &'a str) -> Self {
        Self {
            lang,
            theme,
            pre_class,
        }
    }
}

impl<'a> Formatter for InlineHTML<'a> {
    fn write<W>(&self, source: &str, w: &mut W, event: HighlightEvent) -> Result<()>
    where
        W: std::fmt::Write,
    {
        match event {
            HighlightEvent::Source { start, end } => {
                let span = source
                    .get(start..end)
                    .expect("Source bounds should be in bounds!");
                let span = v_htmlescape::escape(span).to_string();
                w.write_str(&span)?;
            }
            HighlightEvent::HighlightStart(idx) => {
                let scope = constants::HIGHLIGHT_NAMES[idx.0];
                let (class, style) = self.theme.get_scope(scope);
                write!(w, "<span class=\"{}\" style=\"{}\">", class, style)?;
            }
            HighlightEvent::HighlightEnd => {
                w.write_str("</span>")?;
            }
        }

        Ok(())
    }

    fn start<W>(&self, _: &str, w: &mut W) -> Result<()>
    where
        W: std::fmt::Write,
    {
        let (_class, style) = self.theme.get_scope("background");

        write!(
            w,
            "<pre class=\"{}\" style=\"{}\">\n<code class=\"language-{}\">\n",
            self.pre_class, style, self.lang
        )?;
        Ok(())
    }

    fn finish<W>(&self, _: &str, writer: &mut W) -> Result<()>
    where
        W: std::fmt::Write,
    {
        writeln!(writer, "\n</code></pre>")?;

        Ok(())
    }
}
