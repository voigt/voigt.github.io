---
title: Automatically adapt Zellij Theme to macOS System Appearance
date: 2025-08-07
published: 2025-08-07
author:
  - Christoph Voigt
showAuthor: false
draft: false
tags:
  - notes
---
The problem: you switch macOS system appearance to Light/Dark theme, but Zellij‘s appearance does not adjust. Sounds familiar? I recently worked around the same [problem for Helix](/notes/toggle-helix-darkmode/).

Apparently, I'm not the only one who is bothering to switch Zellij's appearance at runtime. There are a couple of GitHub issues discussing the topic from different angles:

- [#3277 - Light/Dark theme option and mirroring system setting](https://github.com/zellij-org/zellij/issues/3277)
- [ #3831 - Allow automatic light/dark mode detection from applications running within zellij (CSI 2031)](https://github.com/zellij-org/zellij/issues/3831)

While there is no native solution (yet), I found a fairly reliable workaround that I can use until native support lands on main.

The solution is built around the fact that Zellij automatically [reloads on a change in its config file](https://zellij.dev/documentation/configuration.html#how-do-i-update-the-config-file-for-running-sessions).

> Zellij actively watches for changes in the [active configuration file](https://zellij.dev/documentation/configuration.html#where-does-zellij-look-for-the-config-file). Most fields will be applied immediately without the need for a restart. Otherwise, this will be mentioned in the commentary of the relevant field.
> 
> *– [Zellij User Guide > Configuration](https://zellij.dev/documentation/configuration.html#how-do-i-update-the-config-file-for-running-sessions)*

So setting my beloved `cattpuccin-frappe` can be achieved in a one liner:

```bash
$ sed -i '' -E "s/^theme .*/theme \"$ZELLIJ_THEME\"/" ~/.config/zellij/config.kdl
```

{{< alert >}}
**NOTE:** If you are like me and maintain symlinks to config files, be aware that Zellijs config reload doesn't work on symlinks. So if you want to utilize Zellijs automatic config reload, you have to work with an actual file path, not a symlink.
{{< /alert >}}

To automate the switch on system appearance changes, we can use the same tool Neovim people use: [dark-notify](https://github.com/cormacrelf/dark-notify). It detects when macOS switches between dark and light mode and "informs us". We can install it via brew:

```bash
$ brew install cormacrelf/tap/dark-notify
```

Now we slightly improve our interface to toggle Zellij theme via a short bash script:

```bash
#!/usr/bin/env bash

set -e -u -o pipefail

ZELLIJ_CONFIG_PATH="${HOME}/.config/zellij/config.kdl"
ZELLIJ_THEME_LIGHT="catppuccin-latte"
ZELLIJ_THEME_DARK="catppuccin-frappe"

switch_zellij_theme() {
  local theme=$1
  if [[ $theme == "light" ]]; then
  
    # Set the light theme
    sed -i '' -E "s/^theme .*/theme \"$ZELLIJ_THEME_LIGHT\"/" "$ZELLIJ_CONFIG_PATH"
  elif [[ $theme == "dark" ]]; then
  
    # Set the dark theme
    sed -i '' -E "s/^theme .*/theme \"$ZELLIJ_THEME_DARK\"/" "$ZELLIJ_CONFIG_PATH"
  else
    echo "Error: Invalid theme '$theme'. Expected 'light' or 'dark'." >&2
    exit 1
  fi
}


if [[ $# -eq 0 ]]; then
  echo "Error: Missing theme argument." >&2
  exit 1
fi

switch_zellij_theme "$1"
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

Done correctly, your Zellij theme should now adapt within milliseconds to your system's appearance.

{{< video src="toggle-zellij-darkmode.mp4" autoplay="true" loop="true" >}}