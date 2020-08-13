# spotify-control-rust

> A simple CLI to control Spotify via D-bus

## Installation

```
$ sudo sh -c "curl https://github.com/welcoMattic/spotify-control-rust/releases/download/v0.1.0/spotify-control -o /usr/local/bin/spotify-control && chmod a+x /usr/local/bin/spotify-control"
```

## Usage

```
Controls Spotify from CLI, retrieve current playing data

USAGE:
    spotify-control [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help          Prints this message or the help of the given subcommand(s)
    next          send next command to Spotify
    pause         send pause command to Spotify
    play          send play command to Spotify
    play_pause    send play_pause command to Spotify
    previous      send previous command to Spotify
    status        fetch current playing metadata
```

## Polybar

You can use `spotify-control` to control your Spotify, and display current song in [Polybar](https://github.com/polybar/polybar).

Bar configuration

```
[bar/main]
modules-center = previous playpause next spotify
```

Modules configuration (adapt colors to your theme)

```
[module/previous]
type = custom/script
interval = 86400
format = "%{T3}<label>"
format-padding = 3
exec = echo ""
; Check if spotify is running before displaying the icon
exec-if = "pgrep spotify"
format-underline = #3EC13F
format-foreground = #99D1CE
format-background = #0A0F14
line-size = 1
click-left = "spotify-control previous"

[module/next]
type = custom/script
interval = 86400
format = "%{T3}<label>"
format-padding = 3
; Next song icon
exec = echo ""
; Check if spotify is running before displaying the icon
exec-if = "pgrep spotify"
format-underline = #3EC13F
format-foreground = #99D1CE
format-background = #0A0F14
line-size = 1
click-left = "spotify-control next"

[module/playpause]
type = custom/ipc
hook-0 = echo " "
hook-1 = echo ""
hook-2 = echo ""
initial = 1
format-padding = 4
format-underline = #3EC13F
format-foreground = #99D1CE
format-background = #0A0F14
line-size = 1
click-left = "spotify-control play_pause"

[module/spotify]
type = custom/ipc
hook-0 = echo ""
hook-1 = spotify-control status
initial = 1
format-padding = 3
format-underline = #3EC13F
format-foreground = #99D1CE
format-background = #0A0F14
line-size = 1
; [i3wm only] - Uncomment the below line to focus on Spotify when clicking on the song name (credits to https://github.com/Esya)
click-left = i3-msg '[class="Spotify"] focus'
```

## Resources
- [Crate mpris](https://docs.rs/mpris/1.1.2/mpris/)
- [Crate clap](https://docs.rs/clap/2.33.2/clap/)

## Credits

I use the python version of dietervanhoof ([https://github.com/dietervanhoof/polybar-spotify-controls](https://github.com/dietervanhoof/polybar-spotify-controls)) for years, but I wanted an easy-to-install version.
Inspired by the C implementation by mihirlad55: [https://github.com/mihirlad55/polybar-spotify-module](https://github.com/mihirlad55/polybar-spotify-module)
