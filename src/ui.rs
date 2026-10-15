//! Shared design-system widgets for Moss & Ember.
//!
//! Palette: warm cream (#f7f4ef), forest green (#2d5a3d), terracotta (#c8603a),
//! sand (#e8ddd0), muted (#7a6b5d).  See site.css for CSS custom properties.
use fission::prelude::*;

// ── Layout helpers ────────────────────────────────────────────────────────

/// A semantics-identified column of widgets, styled via [data-fission-semantics].
#[derive(Clone)]
pub struct Region(pub &'static str, pub Vec<Widget>);
impl From<Region> for Widget {
    fn from(r: Region) -> Self {
        SemanticsRegion::new(Column {
            children: r.1,
            gap: Some(20.0),
            ..Default::default()
        })
        .identifier(r.0)
        .into()
    }
}

/// Semantic heading: (heading level 1–3, CSS identifier, display text).
#[derive(Clone)]
pub struct Heading(pub u8, pub &'static str, pub &'static str);
impl From<Heading> for Widget {
    fn from(h: Heading) -> Self {
        Text::new(h.2)
            .semantics_identifier(format!("site-heading-{}:{}", h.0, h.1))
            .into()
    }
}

/// A hyperlink rendered as a text node with a markdown-link semantics identifier.
#[derive(Clone)]
pub struct Link(pub &'static str, pub &'static str);
impl From<Link> for Widget {
    fn from(l: Link) -> Self {
        Text::new(l.0)
            .semantics_identifier(format!("markdown-link:{}", l.1))
            .into()
    }
}

// ── Navigation ────────────────────────────────────────────────────────────

/// Top navigation bar: wordmark left, links right.
#[derive(Clone)]
pub struct Navigation;
impl From<Navigation> for Widget {
    fn from(_: Navigation) -> Self {
        Region(
            "navigation",
            vec![
                Link("moss & ember", "/").into(),
                Region(
                    "nav-links",
                    vec![
                        Link("Menu", "/menu/").into(),
                        Link("Visit & hours", "/visit/").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}

// ── Footer ────────────────────────────────────────────────────────────────

/// Page footer — brand info on the left, links on the right.
/// Retains required "Built with Fission" attribution.
#[derive(Clone)]
pub struct Footer;
impl From<Footer> for Widget {
    fn from(_: Footer) -> Self {
        Region(
            "footer",
            vec![
                Region(
                    "footer-brand",
                    vec![
                        Text::new("Moss & Ember").into(),
                        Text::new(
                            "14 Ferndale Road, Thornfield Village · \
                             Open daily 7 am – 5 pm · Demo site",
                        )
                        .into(),
                    ],
                )
                .into(),
                Region(
                    "footer-links",
                    vec![
                        Link("Menu", "/menu/").into(),
                        Link("Visit & hours", "/visit/").into(),
                        Link("Built with Fission", "https://fission.rs").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}
