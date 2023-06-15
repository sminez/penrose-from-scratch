# Let's write a window manager from scratch

Writing an X11 window manager from scratch using [penrose](https://github.com/sminez/penrose).

You can follow along with the development of this repo on [youtube](https://www.youtube.com/playlist?list=PLy2HjaQiG8lOxCKzuWKfmmXov4iEVOGOF).

This is a work in progress project to build up a fully featured tiling window manager from scratch
so please make sure you have an alternative desktop environment available to use in case anything
breaks!

## Installation

> **NOTE**: Really _do_ read the Makefile before installing: there's nothing harmful in
> there but you should always know what you are running under `sudo`!

Make sure you have [Rust](https://rust-lang.org) installed on your system and take a look
at the default key bindings in `main.rs`, you will want to swap out `st` and `dmenu_run` for
a terminal and program launcher you have installed if you're not using them.

With that done, read the contents of the `Makefile` in the root of the repo before running
the following in a terminal to build and install the window manager:

```sh
$ make build && sudo make install
```

This should set you up for running `penrose-from-scratch` as a desktop session from your
[display manager](https://wiki.archlinux.org/title/Display_manager) when you log in.
