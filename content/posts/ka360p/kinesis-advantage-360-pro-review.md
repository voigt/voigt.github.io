---
title: One Week with the Kinesis Advantage360 Pro
date: 2025-05-12T12:43:58+02:00
draft: false
tags:
  - keyboards
---

After 25 years of typing on traditional German QWERTZ keyboards, I decided to do something radical: switch to the [Kinesis Advantage360 Pro](https://kinesis-ergo.com/shop/adv360pro/), an ergonomic split keyboard with a U.S. QWERTY layout and a price tag that makes most people flinch.

I’d been eyeing it for a while, intrigued by its promise of better ergonomics, smarter finger usage, and a more neutral typing posture. After one week with it, I can say that this keyboard demands your attention and rewards your patience. Once you know how to use it, typing is a breeze.

## Start With Why

The average typist rarely thinks about changing their existing typing setup. Usually, people use notebooks or an external keyboard that meets personal, visual standards. At least this is my history with keyboards for the last 25 years. I either used the keyboard I had (for whatever reason) on my desk or bought one that I considered visually pleasing.

Nevertheless, I have always been fascinated by keyboards and all their variations.

But then again, considering all this variety, why a split ergo keyboard? Possibly I just watched way too many videos of people showcasing their keyboards, than I like to admit. Thanks to [/r/mechanicalkeyboards](https://www.reddit.com/r/MechanicalKeyboards/) and [/r/ErgoMechKeyboards](https://www.reddit.com/r/ErgoMechKeyboards/).

## What Are We Even Talking About?

Here is an introduction for the innocent readers out there who have no clue what I'm talking about. The Kinesis Advantage360 Pro is an ergonomic split keyboard for "for serious typists and developers". As I consider myself a developer, I immediately felt appealed. The keyboard features sculpted key wells and thumb clusters to promote a natural wrist and finger posture. Something that feels super harmless but changes the entire typing game.

It is heavily customisable via an open-source software called "ZMK", which supports macros and any kind of key remapping you possibly can imagine. My version came with Gateron Brown switches; all have USB-C and Bluetooth support. A "silent" with Gateron Pink switches is also available.

## The Learning Curve Is Real

Let’s start with the obvious: if you’re not already a trained ten-finger typist, prepare for frustration. I wasn’t sloppy, but I wasn’t strict either. I had bad habits, like using different fingers for the same key. The Kinesis doesn't let you get away with that. Every key is placed with intent, which means you either follow proper technique or suffer.

What made things more interesting for me was that I used this switch to move to a U.S. QWERTY layout. I’ve typed on QWERTZ all my life, but since most of my writing is in English and I also do a lot of programming (symbols are way more accessible on QWERTY), I figured this was the right time to commit fully. Surprisingly, the layout change was less painful than expected. However, `Y`/`Z` confusion and missing German umlauts still trip me up occasionally (the latter one only for the few German sentences I have to write during the day). What I like about QWERTY is that special characters such as `, . ; [ ] /` are way more accessible (they are hidden behind the second layer on German Keyboards; this is not only relevant for programming, but also a lot of shortcuts lie on these keys).

## Typing: Slow at First, But Oh So Good

Speed-wise, I’m not back at my usual average of 90–100 WPM. But when I write longer texts—like this one—the typing experience is genuinely enjoyable. The key wells, thumb clusters, and mechanical switches create a satisfying rhythm. I can see myself getting faster and more accurate over time.

![Word Per Minute Progress Over the last 7 Days](/posts/ka360p/image-20250514223533367.png "Word Per Minute Progress Over the last 7 Days, featuring 464 typing tests")

When I first received the ka360p, my typing speed was at a boggling 21 WPM. While I'm writing this text, I managed to hit the 104 WPM.

!["Distribution of Test Results"](/posts/ka360p/image-20250514223645563.png "Distribution of Test Results proofing that I can do the 60-69 WPM relatively easy")

But typing speed and accuracy didn’t just improve on their own. Over the week, I spent roughly 6 hours of pure typing using specialised websites to support my learning journey:

[Monkeytype](https://monkeytype.com/): I’ve used this casually before, but it became my daily typing gym for the last seven days. Perfect for testing speed and layout fluency. Since I have the keyboard, I did 464 typing tests and managed to land 72 WPM on average. You'll find my high scores [here](https://monkeytype.com/profile/cvoigt).

[Keybr](https://keybr.com): This one was a recent discovery. Less pretty than Monkeytype, but arguably more effective. It tracks your weak keystrokes and tailors exercises to improve them. For example, I'm struggling typing words with `Q` or `P` and often mix  `U`, `I`, `O`. Keybr helps address these issues effectively. Here’s a snapshot of my hit/miss ratio that really helped me understand what I need to improve:

![Key Frequency Heatmap as per Keybr.com](/posts/ka360p/image-20250514224028931.png "Key Frequency Heatmap as per Keybr.com")

## The Shortcuts Struggle Is Real

When I did the switch, I underestimated how much of my daily workflow might be affected by a keyboard layout switch. Turns out, apart from typing, I use my keyboard for all sorts of things. Typing is only one part of my daily usage. The rest are shortcuts. As all the action keys (`CTRL`, `ALT`, `CMD`) are now exclusively reachable via thumb, some brain rewiring is necessary. Here are some occasions where shortcuts are important:

Examples

* Copy/paste: `Cmd` is on the right, `C`/`V` are on the left—again, two hands
* Switching desktops on macOS with `Ctrl` + `[1-9]`: anything above 5 now requires two hands
* Screenshots: `Ctrl` + `Shift` + `Cmd` + `3/4` is a finger pretzel
* Arrow keys? I know it's not a combination, but its usage in the beginning was just awful. Up to a point where I'm just not sure if the Arrow Key positioning makes any sense at all (it turns out, it does!)
* Jumping words or to the start/end of a line using Option (`⌥`)/`Cmd` + `←`,`→` just feels weird and I'm still not used to it
* Switching Tabs (`Ctrl`+`Tab`)
* Invoking Spotlight Search (`Cmd`+`Space`)

For now, I haven't even begun remapping shortcuts yet. I've focused entirely on improving typing fluency. But it's clear that to get back to the same efficient workflow, I’ll need to customize a bit. Luckily, the Kinesis comes with freely programmable keys, which gives me hope that I can eventually make it feel as fluid as my old setup.

## Small Surprises

Things no one tells you and what was unexpected to me:

**No switch swapping:** The Advantage360 Pro doesn’t support hot-swapping switches. I found videos of people on Youtube desoldering the old ones, but honestly… I’m neither talented nor patient enough.

**Sweaty wrists:** I do appreciate the keyboards wrist rests, but I noticed my palms get a bit sweaty after a while. Due to the lack of wrist rests, I never experienced this before. We’ll see if that gets annoying or just becomes part of the routine.

**Layout switching hack:** Since my MacBook uses a German layout and the Kinesis uses QWERTY, I needed a tool to switch layouts automatically. [autokbisw](https://github.com/ohueter/autokbisw) solved that perfectly by detecting the active keyboard and changing the layout accordingly.

## Silent vs Not So Silent

I wasn't sure what kind of switches I'd like to use as a daily driver. Therefore, I tried both the ka360p silent and the regular one.

While I enjoyed the typing sound of the silent one, I disliked the missing tactile feedback. Typing felt a bit mushy, and I did accidental keypresses I don't do on the regular one. I guess this is because the regular brown switches have some kind of threshold before a key is actually pressed.

In my tests, it became evident that I was somewhat slower with the silent version - in 5 trials with silent and regular, you can see a significant difference in WPM.

![image-20250514104256666](/posts/ka360p/image-20250514104256666.png)

Nevertheless, if I had to use the keyboard in an office or somewhere where noise plays a role, I'd probably have gone for the silent version, as the sound difference is significant.

## Is It Worth the Investment?

Let’s talk about the elephant in the room: more than 600 euros and a whole relearning curve is a lot to ask. But if you’re already suffering from wrist or hand pain, the equation might look different. The ergonomic benefits alone might justify the price. The keyboard definitely promotes a more natural hand posture, which could prevent long-term strain.

I can also see potential gains in speed and accuracy from a typing perspective, simply because the Kinesis enforces proper technique. But here's the catch: I probably could have achieved similar improvements by just learning proper touch typing on a regular keyboard. But apart from speed or accuracy, typing just feels excellent. Is it due to the Kinesis Advantage360 Pro or a positive effect of using a split keyboard? I can't tell due to my lack of experience with other split ergos.

Ultimately, I realized I solved a problem I didn’t fully have. Yet, I don’t regret the purchase. I like understanding how I type. I enjoy analyzing my own keystrokes and watching my progress. If that sounds like you, then yes, the Kinesis Advantage360 Pro might just be your new favorite toy ...erm... tool.

You're looking for a plug-and-play productivity boost? Look elsewhere. This is not only a keyboard—it’s a hobby.
