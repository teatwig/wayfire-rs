# wayfire-toggle-active-alpha

This little program is used to toggle the opacity (alpha value) of windows (views) in Wayfire

After running it, trigger the toggle by sending `SIGUSR1`, like this:
```sh
kill -n 10 $(pidof wayfire-toggle-active-alpha)
```
If this doesn't work, double check with `kill -l` that the signal number of `SIGUSR1` is 10

## Use this for real
To actually use this, run the following:
```sh
cargo build --release
sudo mv target/release/wayfire-toggle-active-alpha /usr/local/bin
rm -r target
```

Then modify your `wayfire.ini` by adding smth like the following:
```ini
[autostart]
alpha_toggle = wayfire-toggle-active-alpha

[command]
binding_alpha_toggle = <alt> <shift> KEY_O
command_alpha_toggle = kill -n 10 $(pidof wayfire-toggle-active-alpha)
```
