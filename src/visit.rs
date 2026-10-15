//! Visit page — hours, address, transport, parking and accessibility.
use crate::ui::{Heading, Link, Navigation, Region};
use fission::prelude::*;

/// A single info row: label + value pair.
#[derive(Clone)]
pub struct InfoRow {
    pub identifier: &'static str,
    pub label: &'static str,
    pub value: &'static str,
}
impl From<InfoRow> for Widget {
    fn from(row: InfoRow) -> Self {
        Region(
            "info-row",
            vec![
                Text::new(row.label)
                    .semantics_identifier(format!("info-label:{}", row.identifier))
                    .into(),
                Text::new(row.value)
                    .semantics_identifier(format!("info-value:{}", row.identifier))
                    .into(),
            ],
        )
        .into()
    }
}

/// The visit/about page.
#[derive(Clone)]
pub struct Visit;
impl From<Visit> for Widget {
    fn from(_: Visit) -> Self {
        Region(
            "page",
            vec![
                Navigation.into(),
                Region(
                    "page-hero",
                    vec![
                        Text::new("VISIT").into(),
                        Heading(1, "visit-title", "Come and find us.").into(),
                        Text::new(
                            "We're on Ferndale Road in Thornfield Village, \
                             a short walk from Thornfield station. \
                             This is a demo site — the address and hours are fictional.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Opening hours ─────────────────────────────────────────
                Region(
                    "section-hours",
                    vec![
                        Text::new("OPENING HOURS").into(),
                        Heading(2, "hours-heading", "We're open\nevery day.").into(),
                        Region(
                            "hours-list",
                            vec![
                                InfoRow {
                                    identifier: "mon-fri",
                                    label: "Monday – Friday",
                                    value: "7:00 am – 5:00 pm",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "sat",
                                    label: "Saturday",
                                    value: "7:30 am – 5:00 pm",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "sun",
                                    label: "Sunday",
                                    value: "8:00 am – 4:00 pm",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "kitchen",
                                    label: "Kitchen (light bites & lunch)",
                                    value: "From opening until 3:00 pm",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "bank-holidays",
                                    label: "Bank holidays",
                                    value: "9:00 am – 3:00 pm, pastries only",
                                }
                                .into(),
                            ],
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Address & directions ──────────────────────────────────
                Region(
                    "section-address",
                    vec![
                        Text::new("ADDRESS & DIRECTIONS").into(),
                        Heading(2, "address-heading", "14 Ferndale Road,\nThornfield Village").into(),
                        Region(
                            "address-details",
                            vec![
                                InfoRow {
                                    identifier: "postcode",
                                    label: "Postcode",
                                    value: "TH4 2FE (fictional)",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "train",
                                    label: "Train",
                                    value: "Thornfield station, 6-minute walk via Station Road",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "bus",
                                    label: "Bus",
                                    value: "Routes 14 and 22, stop Ferndale Road East",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "cycling",
                                    label: "Cycling",
                                    value: "Cycle rack on Ferndale Road, 3 spaces",
                                }
                                .into(),
                            ],
                        )
                        .into(),
                        Link(
                            "Open in Google Maps ↗",
                            "https://maps.google.com/?q=14+Ferndale+Road+Thornfield",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Parking ───────────────────────────────────────────────
                Region(
                    "section-parking",
                    vec![
                        Text::new("PARKING").into(),
                        Heading(2, "parking-heading", "Parking nearby").into(),
                        Text::new(
                            "There is no dedicated car park, but Thornfield Village \
                             car park on Station Road has 40 spaces and is free for \
                             the first hour (fictional). Pay-and-display thereafter.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Accessibility ─────────────────────────────────────────
                Region(
                    "section-access",
                    vec![
                        Text::new("ACCESSIBILITY").into(),
                        Heading(2, "access-heading", "Everyone welcome").into(),
                        Region(
                            "access-list",
                            vec![
                                InfoRow {
                                    identifier: "step-free",
                                    label: "Step-free entrance",
                                    value: "Yes — level access from Ferndale Road",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "accessible-toilet",
                                    label: "Accessible toilet",
                                    value: "Yes, with baby changing",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "dogs",
                                    label: "Dogs",
                                    value: "Welcome on the terrace",
                                }
                                .into(),
                                InfoRow {
                                    identifier: "buggies",
                                    label: "Buggies",
                                    value: "Level entrance, space for two buggies inside",
                                }
                                .into(),
                            ],
                        )
                        .into(),
                        Text::new(
                            "If you have any specific requirements, \
                             please get in touch before visiting.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── About ─────────────────────────────────────────────────
                Region(
                    "section-about",
                    vec![
                        Text::new("ABOUT").into(),
                        Heading(2, "about-heading", "A neighbourhood café\nsince 2019.").into(),
                        Text::new(
                            "Moss & Ember opened in the spring of 2019 with a simple idea: \
                             make the kind of café we'd want to walk to every morning. \
                             Good coffee sourced directly from small farms, pastry made \
                             from scratch, and a room that feels calm enough to sit in \
                             for a while.",
                        )
                        .into(),
                        Text::new(
                            "We work with Tíerra Roasters — a family-run roastery \
                             five miles down the road — and change our filter offering \
                             every week. We don't have a loyalty card scheme. \
                             We just try to make everything worth coming back for.",
                        )
                        .into(),
                        Text::new(
                            "This is a fictional demo site built to demonstrate \
                             the Fission static-site framework. It is not a real business.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── CTA ───────────────────────────────────────────────────
                Region(
                    "next-step",
                    vec![
                        Heading(2, "visit-cta", "See you soon.").into(),
                        Text::new("Browse our menu while you're here.").into(),
                        Link("View the menu →", "/menu/").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}
