# Progress so far

A bit of a high level change log of what we've done so far in the series and what's
up next. This isn't a full project roadmap by any means but it should serve as a
bit of a guide for where to find things and where we're heading.

> **NOTE**: Code coming from the episodes should have commit messages prefixed with
>           the episode number they came from so that you can look up where changes
>           came from relatively easily.
>
>           `ep42: found the meaning of life (bound to "M-S-/")`


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

  [0]: https://sminez.github.io/penrose/getting-started.html
  [1]: https://github.com/sminez/penrose/blob/develop/examples/minimal/main.rs
  [2]: https://specifications.freedesktop.org/wm-spec/latest/
  [3]: https://sminez.github.io/penrose/rustdoc/penrose/builtin/layout/index.html
