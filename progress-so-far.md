# Progress so far

A bit of a high level change log of what we've done so far in the series and what's
up next. This isn't a full project roadmap by any means but it should serve as a
bit of a guide for where to find things and where we're heading.

> **NOTE**: Code coming from the episodes should have commit messages prefixed with
>           the episode number they came from so that you can look up where changes
>           came from relatively easily.
>
> `ep42: found the meaning of life (bound to "M-S-/")`


## Ep01
_Bootstrapping our window manager_

We went through setting up our initial `main.rs` following the steps covered in the
[getting started guide][0] in the Penrose docsite (along with some helper scripts
to make things a little smoother to iterate on going forward). What we end up with
is pretty bare bones but it works and we have a starting point to start iterating
on our window manager!

- [x] Initial crate with dependencies
- [x] Copy in the `main.rs` from the [minimal example][1] in GitHub
- [x] `Makefile` and helper scripts
- [x] xsession file

---

## Ep02
_Making things a little more comfortable to live in_

Minimal really does mean minimal: there's not much here to start so lets hook up
some quality of life improvements we can copy from the examples so that we have
a decent working environment to start customising things.

- [x] Add in [EWMH][2] hooks so that programs which need EWMH properties can find them
- [x] Set up a simple status bar (also requires telling the layouts to reserve screen space)
- [x] Take a look at layouts and the [built in layout algorithms][3]
- [x] Logout / restart keybindings

---

## Ep03
_Configuring keybindings_

Now that we have something that's a little nicer to live in we can start digging in
to specfic things work. This episode we're taking a look at keybindings: how they
work, how to write custom KeyEventHandlers and what Penrose is doing behind the
scenes with your bindings to update your window manager state.

- [x] Reviewing the logout / restart keybindings we wrote last time
- [x] Looking at the [KeyEventHandler][4] trait
  - [Traits in the Rust book][5]
- [x] The [key_handler][6] helper function
  - Looking at the source code of some of the built in actions
  - [Trait objects][7] in Rust
- [x] The [modify_and_refresh][8] method on `XConn`
- [x] Using [dmenu][9] to write ourselves a power menu

---

## Ep04
_Hooking into window manager execution_

In addition to running your own code in response to a key binding, you can also set
up custom hooks to tell Penrose to run some additional logic when specific parts of
the window manager event loop are hit. In this episode we take a look at what the
different hook points are, the traits involved for writing hooks and we have a go
at writing each kind of hook.

- [x] Overview of how the window manager runs
  - Details on why we need set the sigchild handler can be found [here][10]
- [x] The [different kinds of hooks][11] we can write
  - [x] Adding a startup hook ([execute a script][12])
  - [x] Adding a layout hook ([spacing and gaps][13])
  - [x] Adding a manage hook ([move a certain program to a specific workspace][14])
  - [ ] ~Refresh and event hooks in the status bar~

---

## Ep05
_State extensions, scratchpads and the statusbar_

Sometimes you find yourself needing to make use of persistant state that is not provided
by the window manager itself. There are lots of ways to achieve this in Rust but one easy
way to track state which you are using for your bindings and hooks is to use a `state extension`.
In this episode we take a look at what a state extension is, how to set one up and where
some of the code provided in the main Penrose crates makes use of them.

- [x] What are [state extensions][15]?
- [x] Why do you need these in the first place?
- [x] Adding a [NamedScratchpad][16]

---

## Ep06
_Writing status bar widgets_

The status bar provided by the `penrose-ui` crate allows you to write simple text based widgets
which you can drive from your window manager state or external data sources. The default set up
is configured to mimic the status bar from `dwm` with the root window name being used as a way
of placing arbitrary text in your status bar. The [widget support][18] in `penrose-ui` extends
this idea to allow you to drive individual sections of the bar using custom code.
In this episode we go over how the status bar works and how you can write your own widgets to
extend the behaviour.

- [x] Taking a look at the [penrose-ui crate][17]
- [x] How [widgets][18] work
- [x] Rewriting our status bar to customise which widgets we use
- [x] Writing a our own simple widget using [IntervalText][19]

---

## Ep07
_Writing a custom layout_

One of the most appealing things about a tiling window manager is having layout algorithms
automatically position your windows for you on the screen. Penrose comes with a few simple
algorithms out of the box and in this episode we'll write a new one, show how to work with
the [Rect][20] struct to easily divide up screen space.

- [x] A quick look at the [Layout][21] trait.
- [x] What the Penrose book says about layouts
  - [Built-in layouts][22]
  - [A guide to writing layouts][23]
- [x] Writing a fibonacci layout (inspired by [this dwm patch][24])
  - We'll be using the [print_layout_result][25] helper for this
- [x] The [entr][26] program I use to auto-run our example file when changes are made
      can be found [here][26].

---

## Ep08
_Handling layout messages and fun with layout transformers_

As we've already seen with the `MainAndStack` layout, it is possible to modify how the
active layout is working by sending it [Messages][27]. These allow us to bind keys to
changes in our layout behaviour in pretty much any way that we want!
For layouts that are not implemented by us, there is the [LayoutTransformer][28] trait
which allows for per-layout modifications to how layouts run (similar to a layout hook).
There are a couple of built-in layout transformers available to use but it is also possible
to hand write our own higher-order layouts in order to _really_ customise how things work.

- [ ] Sending dynamically typed messages to layouts
- [ ] Handling messages in our layout
- [ ] Applying layout transformers to existing layouts
- [ ] Writing a custom meta-layout because we can
  - This last one might sound intimidating but it's actually pretty easy!

---


  [0]: https://sminez.github.io/penrose/getting-started.html
  [1]: https://github.com/sminez/penrose/blob/develop/examples/minimal/main.rs
  [2]: https://specifications.freedesktop.org/wm-spec/latest/
  [3]: https://sminez.github.io/penrose/rustdoc/penrose/builtin/layout/index.html
  [4]: https://sminez.github.io/penrose/rustdoc/penrose/core/bindings/trait.KeyEventHandler.html
  [5]: https://doc.rust-lang.org/book/ch10-02-traits.html
  [6]: https://sminez.github.io/penrose/rustdoc/penrose/builtin/actions/fn.key_handler.html
  [7]: https://doc.rust-lang.org/reference/types/trait-object.html
  [8]: https://sminez.github.io/penrose/rustdoc/penrose/x/trait.XConnExt.html#method.modify_and_refresh
  [9]: http://tools.suckless.org/dmenu/
  [10]: https://doc.rust-lang.org/std/process/struct.Child.html#warning
  [11]: https://sminez.github.io/penrose/rustdoc/penrose/core/hooks/index.html
  [12]: https://sminez.github.io/penrose/rustdoc/penrose/extensions/hooks/startup/struct.SpawnOnStartup.html
  [13]: https://sminez.github.io/penrose/rustdoc/penrose/builtin/hooks/struct.SpacingHook.html
  [14]: https://sminez.github.io/penrose/rustdoc/penrose/extensions/hooks/manage/index.html
  [15]: https://sminez.github.io/penrose/rustdoc/penrose/core/struct.State.html#method.extension
  [16]: https://sminez.github.io/penrose/rustdoc/penrose/extensions/hooks/named_scratchpads/index.html
  [17]: https://sminez.github.io/penrose/rustdoc/penrose_ui/index.html
  [18]: https://sminez.github.io/penrose/rustdoc/penrose_ui/bar/widgets/trait.Widget.html
  [19]: https://sminez.github.io/penrose/rustdoc/penrose_ui/bar/widgets/struct.IntervalText.html
  [20]: https://sminez.github.io/penrose/rustdoc/penrose/pure/geometry/struct.Rect.html
  [21]: https://sminez.github.io/penrose/rustdoc/penrose/core/layout/trait.Layout.html
  [22]: https://sminez.github.io/penrose/builtin/layouts.html
  [23]: https://sminez.github.io/penrose/building/layouts.html
  [24]: https://dwm.suckless.org/patches/fibonacci/
  [25]: https://sminez.github.io/penrose/rustdoc/penrose/util/fn.print_layout_result.html
  [26]: https://github.com/eradman/entr
  [27]: https://sminez.github.io/penrose/rustdoc/penrose/core/layout/trait.IntoMessage.html
  [28]: https://sminez.github.io/penrose/rustdoc/penrose/core/layout/trait.LayoutTransformer.html
