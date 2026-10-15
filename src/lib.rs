mod home;
mod menu;
mod ui;
mod visit;

use fission::prelude::*;
use fission::site::FissionSite;

#[derive(Debug, Default)]
pub struct SiteState;
impl GlobalState for SiteState {}

pub fn site_app() -> FissionSite {
    FissionSite::new()
        .user_css(include_str!("site.css"))
        .route_widget::<SiteState, _>(
            "/",
            "Moss & Ember — Neighbourhood Café & Bakery",
            Some(
                "House-roasted coffee, seasonal pastries and honest lunches \
                 made from scratch in Thornfield Village. Demo site."
                    .into(),
            ),
            home::Home,
        )
        .route_widget::<SiteState, _>(
            "/menu/",
            "Menu — Moss & Ember",
            Some("Coffee, pastries, light bites and lunch at Moss & Ember.".into()),
            menu::Menu,
        )
        .route_widget::<SiteState, _>(
            "/visit/",
            "Visit — Moss & Ember",
            Some(
                "Opening hours, address, parking and accessibility \
                 information for Moss & Ember in Thornfield Village."
                    .into(),
            ),
            visit::Visit,
        )
        .footer_widget::<SiteState, _>(ui::Footer)
}
