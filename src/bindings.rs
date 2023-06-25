use penrose::{
    builtin::{
        actions::{key_handler, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    extensions::{
        hooks::named_scratchpads::ToggleNamedScratchPad,
        util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    },
    map, util,
    x11rb::RustConn,
};
use std::{collections::HashMap, process::exit};

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

pub fn raw_key_bindings(
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

#[cfg(test)]
mod tests {
    use super::*;
    use penrose::{
        core::bindings::parse_keybindings_with_xmodmap,
        extensions::hooks::{manage::FloatingCentered, named_scratchpads::NamedScratchPad},
        x::query::ClassName,
        x11rb::RustConn,
    };

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
