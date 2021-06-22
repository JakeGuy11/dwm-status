# dwm-status

dwm-status is a hardware monitor written in rust targeted at the DWM window manager. It uses `xsetroot` to display the details, so any window manager that uses that command should also be able to use it.

I will add more details in the future when I implement command line arguments but for now, this is it.

To use it, put `/path/to/dwm-status &` in your `.xinitrc` or equivalent file.
