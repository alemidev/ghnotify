# ghnotify
Desktop notifications for github. Currently only supports linux (+ maybe BSD?)

This is a polling solution: no push notifications available. Create a cronjob or systemd timer launching this periodically.

It requires a GITHUB_TOKEN set in your environment.

I made this because github notifications are unreliable and messy and there isn't a way to get them on desktop.

Both `octocrab` and `notify_rust` aren't really exactly what is needed so this software is really awful in its implementation.

I built this in ~1h and ~50 lines so expect extra awful code.
