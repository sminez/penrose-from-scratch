#! /usr/bin/env bash
# ----------------------------------------
# Bootstrap the start of a penrose session
# >> This get's run on restart as well!
# ----------------------------------------

# Make sure we only run once
pid=$$
pgrep -fi penrose-startup.sh | grep -v "^$pid$" | xargs -I{} kill {}

# Set screen resolutions (add additional screens here)
xrandr --output HDMI-2 --auto --right-of eDP-1 &

# fix a couple of quirks with my thinkpad: enable tap-click for the touchpad
# and slow down the track point accelleration
xinput --set-prop "11" "libinput Tapping Enabled" 1
xinput --set-prop "12" "libinput Accel Speed" 0.0

# Keyboard overrides
setxkbmap -option caps:super
xsetroot -cursor_name left_ptr

pkill -fi picom; picom &
pkill -fi nm-applet; nm-applet &
pkill -fi udiskie; udiskie -a -n -t &
pkill -fi volumeicon; volumeicon &
pkill -fi dunst; dunst &
pkill -fi blueman-applet; blueman-applet &
pkill -fi xfce4-power-man; xfce4-power-manager &  # for some reason, this ends up running as xcfe4-power-man
pkill -fi xfce4-screensaver; xfce4-screensaver &
pkill -fi gnome-keyring-daemon; gnome-keyring-daemon --start --components=pkcs11,secrets,ssh &

"$HOME/.fehbg"
