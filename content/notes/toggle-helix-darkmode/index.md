---
title: Automatically adapt Helix Theme to macOS System Appearance
date: 2025-07-28
published: 2025-07-28
author:
  - Christoph Voigt
showAuthor: false
draft: false
tags:
  - notes
---
The problem: you switch macOS system appearance to Light/Dark theme, but Helix‘s appearance does not adjust.

Apparently, I'm not the only one who is bothering to switch Helix's appearance at runtime. There are a couple of GitHub issues discussing the topic from different angles:

- [#8899 - Automatically select light/dark theme variant](https://github.com/helix-editor/helix/issues/8899)
- [#10281 - Ability to define second theme to sync with OS](https://github.com/helix-editor/helix/discussions/10281#discussioncomment-11246639)
- [#13281 - Support mode 2031 dark/light mode detection](https://github.com/helix-editor/helix/issues/13281)

While there is no native solution (yet), I found a fairly reliable workaround that I can use until native support lands on main.

The solution is built around the fact that helix config can be reloaded via

```bash
$ pkill -USR1 hx
```

So setting my beloved `cattpuccin_frappe` can be achieved in a one liner:

```bash
$ sed -i 's/theme .*/theme = "cattpuccin_frappe"/' ~/.config/helix/config.toml && \
  pkill -USR1 hx
```

To automate the switch on system appearance changes, we can use the same tool Neovim people use: [dark-notify](https://github.com/cormacrelf/dark-notify). It is a program that watches for when macOS switches its appearance (dark/light mode). We can install it via brew:

```bash
$ brew install cormacrelf/tap/dark-notify
```

Now we slightly improve our interface to toggle helix theme via a short bash script:

```bash
#!/usr/bin/env bash

set -e -u -o pipefail

HELIX_CONFIG_PATH="${HOME}/.config/helix/config.toml"
HELIX_THEME_LIGHT="catppuccin_latte"
HELIX_THEME_DARK="catppuccin_frappe"

switch_helix_theme() {
  local theme=$1
  if [[ $theme == "light" ]]; then
    
    # Set the light theme
    sed -i '' -E "s/^theme .*/theme = \"$HELIX_THEME_LIGHT\"/" "$HELIX_CONFIG_PATH"
  elif [[ $theme == "dark" ]]; then
      
    # Set the dark theme
    sed -i '' -E "s/^theme .*/theme = \"$HELIX_THEME_DARK\"/" "$HELIX_CONFIG_PATH"
  else
    echo "Error: Invalid theme '$theme'. Expected 'light' or 'dark'." >&2
    exit 1
  fi

  pkill -USR1 hx || true
}

if [[ $# -eq 0 ]]; then
  echo "Error: Missing theme argument." >&2
  exit 1
fi

switch_helix_theme "$1"
```

Now we need to set a launchd configuration:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>local.cvoigt.dark-notify</string>

    <key>ProgramArguments</key>
    <array>
      <string>/opt/homebrew/bin/dark-notify</string>
      <string>-c</string>
      <string>/Users/c.voigt/go/src/github.com/voigt/dotfiles/themes/toggle_system_appearance.sh</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <true/>
 	</dict>
</plist>

```

Obviously, you will have to adjust the path to your bash script. Please note that the path has to be absolute; relative paths won't work.

I stored the service in `~/Library/LaunchAgents` as `local.cvoigt.dark-notify.plist`.


{{< alert >}}
**Note:** the filename and the Label value need to match. So my file name is `local.cvoigt.dark-notify.plist`, which means the `Label` needs to be `local.cvoigt.dark-notify`
{{< /alert >}}

Finally, load the service and make sure it is listed among the running services.

```bash
$ launchctl load ~/Library/LaunchAgents/local.cvoigt.dark-notify.plist

$ launchctl list | grep dark-notify
61513   0       local.cvoigt.dark-notif
```

Done correctly, your Helix theme should now adapt within milliseconds to your system's appearance.

{{< video src="toggle-helix-darkmode.mp4" autoplay="true" loop="true" >}}