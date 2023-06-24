//! penrose from scratch
use penrose::{
    builtin::{
        actions::{key_handler, modify_with, send_layout_message, spawn},
        hooks::SpacingHook,
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::{
        hooks::{
            add_ewmh_hooks,
            manage::{FloatingCentered, SetWorkspace},
            named_scratchpads::{add_named_scratchpads, NamedScratchPad, ToggleNamedScratchPad},
            startup::SpawnOnStartup,
        },
        util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    },
    map, util,
    x::query::ClassName,
    x11rb::RustConn,
    Result,
};
use penrose_ui::{bar::Position, core::TextStyle, status_bar};
use std::{collections::HashMap, process::exit};
use tracing_subscriber::{self, prelude::*};

// UI style for the status bar
const FONT: &str = "ProFontIIx Nerd Font";
const BAR_HEIGHT_PX: u32 = 24;
const BLACK: u32 = 0x282828ff;
const WHITE: u32 = 0xebdbb2ff;
const GREY: u32 = 0x3c3836ff;
const BLUE: u32 = 0x458588ff;

fn power_menu() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|state, _| {
        let screen_index = state.client_set.current_screen().index();
        let dmenu = DMenu::new(
            &DMenuConfig {
                custom_prompt: Some(">>> ".to_string()),
                ..Default::default()
            },
            screen_index,
        );
        let choices = vec!["restart-penrose", "logout"];

        if let Ok(MenuMatch::Line(_, choice)) = dmenu.build_menu(choices) {
            match choice.as_ref() {
                "restart-penrose" => exit(0),
                "logout" => util::spawn("pkill -fi penrose"),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    })
}

fn raw_key_bindings(
    toggle_scratch: ToggleNamedScratchPad,
) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-S-q" => modify_with(|cs| cs.kill_focused()),

        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-S-bracketright" => modify_with(|cs| cs.drag_workspace_forward()),
        "M-S-bracketleft" => modify_with(|cs| cs.drag_workspace_backward()),

        "M-grave" => modify_with(|cs| cs.next_layout()),
        "M-S-grave" => modify_with(|cs| cs.previous_layout()),

        "M-S-Up" => send_layout_message(|| IncMain(1)),
        "M-S-Down" => send_layout_message(|| IncMain(-1)),
        "M-S-Right" => send_layout_message(|| ExpandMain),
        "M-S-Left" => send_layout_message(|| ShrinkMain),

        "M-semicolon" => spawn("rofi-apps"),
        "M-Return" => spawn("kitty"),
        "M-slash" => Box::new(toggle_scratch),

        "M-A-Escape" => power_menu(),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn layouts() -> LayoutStack {
    LayoutStack::default()
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let style = TextStyle {
        font: FONT.to_string(),
        point_size: 10,
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2.0, 2.0),
    };

    let (nsp, toggle_scratch) = NamedScratchPad::new(
        "terminal",
        "kitty --class=KittyScratch",
        ClassName("KittyScratch"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings(toggle_scratch))?;

    let startup_hook = SpawnOnStartup::boxed("/usr/local/scripts/penrose-startup.sh");
    let manage_hook = Box::new((ClassName("qutebrowser"), SetWorkspace("5")));
    let layout_hook = Box::new(SpacingHook {
        inner_px: 5,
        outer_px: 5,
        top_px: 0,
        bottom_px: BAR_HEIGHT_PX,
    });

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        startup_hook: Some(startup_hook),
        layout_hook: Some(layout_hook),
        manage_hook: Some(manage_hook),
        ..Config::default()
    });

    let bar = status_bar(BAR_HEIGHT_PX, &style, BLUE, GREY, Position::Bottom).unwrap();
    let wm = bar.add_to(WindowManager::new(
        config,
        key_bindings,
        HashMap::new(),
        conn,
    )?);

    let wm = add_named_scratchpads(wm, vec![nsp]);

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let (_, toggle_scratch) = NamedScratchPad::<RustConn>::new(
            "terminal",
            "kitty --class=KittyScratch",
            ClassName("KittyScratch"),
            FloatingCentered::new(0.8, 0.8),
            true,
        );

        let res = parse_keybindings_with_xmodmap(raw_key_bindings(toggle_scratch));

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
