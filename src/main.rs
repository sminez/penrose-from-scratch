//! penrose from scratch
use penrose::{
    builtin::hooks::SpacingHook,
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::{
        add_ewmh_hooks,
        manage::{FloatingCentered, SetWorkspace},
        named_scratchpads::{add_named_scratchpads, NamedScratchPad},
        startup::SpawnOnStartup,
    },
    x::query::ClassName,
    x11rb::RustConn,
};
use penrose_from_scratch::{
    bar::status_bar, bindings::raw_key_bindings, layouts::layouts, BAR_HEIGHT_PX, GAP_PX,
    STARTUP_SCRIPT,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let (nsp, toggle_scratch) = NamedScratchPad::new(
        "terminal",
        "st -c=StScratch",
        ClassName("StScratch"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );

    let wm = WindowManager::new(
        add_ewmh_hooks(config()),
        parse_keybindings_with_xmodmap(raw_key_bindings(toggle_scratch))?,
        HashMap::new(),
        RustConn::new()?,
    )?;

    let wm = add_named_scratchpads(status_bar()?.add_to(wm), vec![nsp]);
    wm.run()?;

    Ok(())
}

fn config() -> Config<RustConn> {
    let startup_hook = SpawnOnStartup::boxed(STARTUP_SCRIPT);
    let manage_hook = Box::new((ClassName("obs"), SetWorkspace("1")));
    let layout_hook = Box::new(SpacingHook {
        inner_px: GAP_PX,
        outer_px: GAP_PX,
        top_px: 0,
        bottom_px: BAR_HEIGHT_PX,
    });

    Config {
        default_layouts: layouts(),
        startup_hook: Some(startup_hook),
        layout_hook: Some(layout_hook),
        manage_hook: Some(manage_hook),
        ..Config::default()
    }
}
