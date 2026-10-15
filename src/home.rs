//! Home page — hero, daily specials strip, intro and CTA.
use crate::ui::{Heading, Link, Navigation, Region};
use fission::prelude::*;

/// Three rotating daily-specials cards shown below the hero.
#[derive(Clone)]
pub struct DailySpecials;
impl From<DailySpecials> for Widget {
    fn from(_: DailySpecials) -> Self {
        Region(
            "specials",
            vec![
                Region(
                    "special",
                    vec![
                        Text::new("TODAY'S BAKE").into(),
                        Heading(3, "special-bake", "Brown butter\ncardamom bun").into(),
                        Text::new(
                            "Laminated dough, brown-butter frangipane, \
                             crushed cardamom sugar. From 8 am.",
                        )
                        .into(),
                    ],
                )
                .into(),
                Region(
                    "special",
                    vec![
                        Text::new("COFFEE SPECIAL").into(),
                        Heading(3, "special-coffee", "Iced honey\noat latte").into(),
                        Text::new(
                            "Single-origin espresso, house-made oat milk, \
                             wildflower honey. Available all day.",
                        )
                        .into(),
                    ],
                )
                .into(),
                Region(
                    "special",
                    vec![
                        Text::new("THIS WEEK'S LUNCH SOUP").into(),
                        Heading(3, "special-soup", "Roasted tomato\n& harissa").into(),
                        Text::new(
                            "Served with sourdough and cultured butter. \
                             Noon – 3 pm, while it lasts.",
                        )
                        .into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}

/// The home page widget.
#[derive(Clone)]
pub struct Home;
impl From<Home> for Widget {
    fn from(_: Home) -> Self {
        Region(
            "page",
            vec![
                Navigation.into(),
                // ── Hero ──────────────────────────────────────────────────
                Region(
                    "hero",
                    vec![
                        Region(
                            "hero-copy",
                            vec![
                                Text::new("NEIGHBOURHOOD CAFÉ & BAKERY · THORNFIELD VILLAGE · DEMO").into(),
                                Heading(1, "home-title", "Good mornings\nstart here.").into(),
                                Text::new(
                                    "House-roasted coffee, seasonal pastries and \
                                     honest lunches made from scratch. Moss & Ember \
                                     has been a warm corner of Thornfield since 2019.",
                                )
                                .into(),
                                Region(
                                    "actions",
                                    vec![
                                        Link("See our menu →", "/menu/").into(),
                                        Link("Find us", "/visit/").into(),
                                    ],
                                )
                                .into(),
                                Text::new(
                                    "Open daily  7 am – 5 pm  ·  \
                                     14 Ferndale Road, Thornfield",
                                )
                                .into(),
                            ],
                        )
                        .into(),
                        // Mood card — purely decorative copy block
                        Region(
                            "hero-mood",
                            vec![
                                Region(
                                    "mood-card",
                                    vec![
                                        Text::new("THE SPACE").into(),
                                        Text::new(
                                            "Twelve seats inside, six on the terrace. \
                                             Exposed brick, trailing ivy and the smell \
                                             of butter browning at 8 am.",
                                        )
                                        .into(),
                                        Text::new(
                                            "Dogs welcome on the terrace. \
                                             Buggy-friendly entrance on Ferndale Road.",
                                        )
                                        .into(),
                                    ],
                                )
                                .into(),
                            ],
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Intro band ────────────────────────────────────────────
                Region(
                    "intro",
                    vec![
                        Text::new("FROM THE BAKERY").into(),
                        Heading(2, "intro-title", "Baked fresh,\nevery single day.").into(),
                        Text::new(
                            "Our baker arrives at 5:30 am. By the time you walk in, \
                             the laminated doughs are already out of the oven and the \
                             kitchen smells of toasted nuts and warm spice. \
                             Whatever's left at 4:30 goes to the Thornfield food bank.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Specials strip ────────────────────────────────────────
                DailySpecials.into(),
                // ── CTA ───────────────────────────────────────────────────
                Region(
                    "next-step",
                    vec![
                        Heading(2, "home-cta", "Stay a while.").into(),
                        Text::new(
                            "Browse the full menu or find out \
                             how to get here.",
                        )
                        .into(),
                        Link("View the full menu →", "/menu/").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}
