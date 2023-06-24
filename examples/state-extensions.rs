//! A demo of how state extensions work in penrose
use penrose::{core::WindowManager, x11rb::RustConn};

#[derive(Debug)]
struct MyExtension {
    n: usize,
}

fn main() -> anyhow::Result<()> {
    // The only way to get at a State is from a WindowManager so we need to make a stub one
    let mut wm = WindowManager::new(
        Default::default(),
        Default::default(),
        Default::default(),
        RustConn::new()?,
    )?;

    // State extensions can be added directly to the WindowManager
    wm.add_extension("a str as a state extension");
    // Or via the State itself if that's what you have access to
    wm.state
        .add_extension("a String as a state extension".to_string());

    // Either method results in the state being accessible in the same way
    // The ::<> syntax here is known lovingly as "turbofish" syntax by the Rust community.
    // It is a way of specifying a generic parameter to a function when specifying the
    // type of a generic return value is clunky (here `ext` is an Arc<RefCell<ExampleExtension>>
    // so it's a lot simpler to use the turbofish!)
    println!("&'static str: {:?}", wm.state.extension::<&'static str>());
    println!("String: {:?}", wm.state.extension::<String>());

    // Attempting to extract a state extension of a type that doesn't exist results in an error
    println!("i64: {:?}", wm.state.extension::<i64>());

    wm.state.add_extension(MyExtension { n: 42 });
    println!("MyExtension: {:?}", wm.state.extension::<MyExtension>());
    add_one(&wm)?;
    println!("MyExtension: {:?}", wm.state.extension::<MyExtension>());

    // The `extension_or_default` method can be used to initialise a state extension value using
    // Default if it is not already present in State
    println!("bool: {:?}", wm.state.extension_or_default::<bool>());

    // Finally, state extensions can be removed
    wm.state.add_extension(42_i64);
    println!(
        "removed extension: {:?}",
        wm.state.remove_extension::<i64>()
    );

    // You'll get back `None` if the type requested isn't in the extensions
    println!(
        "removed extension: {:?}",
        wm.state.remove_extension::<i32>()
    );

    Ok(())
}

// When we extract a state extension it is possible to mutate the value so long as the currently
// running code is the only thing with a reference to the value (be careful with threads!)
fn add_one(wm: &WindowManager<RustConn>) -> anyhow::Result<()> {
    let ext = wm.state.extension::<MyExtension>()?;

    ext.borrow_mut().n += 1;

    Ok(())
}
