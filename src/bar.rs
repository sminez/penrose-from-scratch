//! Config and logic for our status bar
use crate::{BAR_HEIGHT_PX, BLACK, BLUE, FONT, GREY, POINT_SIZE, WHITE};
use penrose::{util::spawn_for_output_with_args, x::XConn, Color};
use penrose_ui::{
    bar::{
        widgets::{
            amixer_volume, battery_summary, current_date_and_time, wifi_network, ActiveWindowName,
            CurrentLayout, IntervalText, Workspaces,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};
use std::time::Duration;

const MAX_ACTIVE_WINDOW_CHARS: usize = 50;

// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = BLUE.into();
    let empty_ws: Color = GREY.into();

    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };

    let padded_style = TextStyle {
        padding: (4, 2),
        ..style
    };

    StatusBar::try_new(
        Position::Bottom,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        FONT,
        POINT_SIZE,
        vec![
            Box::new(Workspaces::new(style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(style)),
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                TextStyle {
                    bg: Some(highlight),
                    padding: (6, 4),
                    ..style
                },
                true,
                false,
            )),
            Box::new(current_weather_info(padded_style)),
            Box::new(wifi_network(padded_style)),
            Box::new(battery_summary("BAT1", padded_style)),
            Box::new(battery_summary("BAT0", padded_style)),
            Box::new(amixer_volume("Master", padded_style)),
            Box::new(current_date_and_time(padded_style)),
        ],
    )
}

fn current_weather_info(style: TextStyle) -> IntervalText {
    IntervalText::new(style, get_weather_text, Duration::from_secs(60 * 5))
}

// Make a curl request to wttr.in to fetch the current weather information
// for our location.
fn get_weather_text() -> String {
    spawn_for_output_with_args("curl", &["-s", "http://wttr.in?format=%c%t"])
        .unwrap_or_default()
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_weather_text_works() {
        let s = get_weather_text();
        assert!(!s.is_empty());
    }
}
