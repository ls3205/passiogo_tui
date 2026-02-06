use ratatui::{
    prelude::Stylize,
    style::Color,
    text::{Span, ToSpan},
};

pub fn keybindinator(
    binds: Vec<(String, String)>,
    c1: Color,
    c2: Color,
) -> ratatui::prelude::Line<'static> {
    let mut out = ratatui::prelude::Line::from(vec![" ".to_span()]);

    for (k, v) in binds {
        out.push_span(Span::from(k).fg(c1));
        out.push_span(" ".to_span());
        out.push_span(Span::from(v).fg(c2));
        out.push_span(" ".to_span());
    }

    out
}
