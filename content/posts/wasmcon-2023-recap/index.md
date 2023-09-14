---
author: Christoph Voigt
title: WasmCon 2023 - Recap
date: 2023-09-14
description: WasmCon 2023 - Recap
tags:
  - wasm
---

In this post, I'm excited to share some insights from my recent journey to Seattle for WasmCon NA 2023. I'll delve into the highlights of the conference, the thriving community, and the current landscape of WebAssembly.

For those seeking a quick summary of overarching trends, look no further:

- WebAssembly is increasingly finding its niche in Serverless and Plugin Systems.
- The push to integrate WebAssembly with embedded devices is an emerging trend that several companies are actively exploring.
- The WebAssembly component model continues to mature and is inching closer to fulfilling its potential. Full-fledged support for other languages like Rust, JavaScript, and Python could be just a few months away.
- Significant efforts are underway to enhance WebAssembly's security, focusing on both runtime security and higher-order systems like Hyperlight, GVisor or similar.

### City and Conference Venue

Let's kick things off by discussing Seattle, the conference city, and the venue. Seattle captivated me, even though my time there was brief.

Home to over 700,000 residents, Seattle is the largest city in both the state of Washington and the Pacific Northwest region of North America. I endured a 10-hour flight from Frankfurt, Germany, so jet lag was inescapable.

The conference was hosted at the Hyatt Regency Hotel—a stellar venue choice. With around 350 attendees, the setting felt just right. Conveniently situated in downtown Seattle, the hotel had spacious, well-appointed conference rooms. Plus, it's connected to a large mall, an ideal location for after-hours networking.

WasmCon featured four parallel tracks, offering close to 40 talks in addition to keynotes. The event was backed by a robust list of sponsors and partners. Big names like Nginx and Cloudflare were present, along with WebAssembly-focused companies like Fermyon and Cosmonic, and (WebAssembly-)newcomers like Golem and Midokura.

<center>
{{< tweet user="vogti" id="1699242161995407419" >}}
</center>

### My Top Talks

The notion of tightly integrating WebAssembly with operating systems was a recurring theme. ["Let's Build a Linux Just for Wasm!"](https://sched.co/1PCLu) by Andrew Randall and Ralph Squillace (Microsoft) used FlatCar Linux to create an OS optimized for WebAssembly—highly engaging!  
Dan Phillips from Loophole Labs went on an unconventional adventure in ["Building a WebAssembly-first OS – An Adventure Into the Unorthodox"](https://sched.co/1Q1wG), exploring topics like syscall handling, process scheduling, and hardware compatibility—all from a WebAssembly perspective. Truly a thought-provoking experiment!

A plethora of experimental WebAssembly runtimes exist. In ["Security and Correctness in Wasmtime"](https://sched.co/1PkHX) Nick Fitzgerald dissected how Wasmtime is striving to ensure both runtime security and functional correctness. It was a technically dense but incredibly enlightening talk.

#### Real-World WebAssembly Applications

Beyond theoretical discourse, there were several practical, hands-on sessions. I want to highlight ["Dapr-Enabled Wasm Microservices"](https://sched.co/1PCML) by Michael Yuan (WasmEdge / Second State) and Yaron Schneider (Dapr / Diagrid). They discussed enabling WebAssembly microservices on Kubernetes via Dapr. It's invigorating to see discussions transitioning from theory to real-world applications.

Similarly, ["Getting Started with AI and WebAssembly"](https://sched.co/1PCLs) by Angel M De Miguel Meana (VMWare) and Michael Yuan explained how to deploy machine learning models as WebAssembly modules using WASI-nn. They presented various demos on different runtimes like WasmTime and WasmEdge, demonstrating the flexibility of applying the same model server-side and in-browser.

#### The WebAssembly Component Model

This year's WasmCon was abuzz with discussions about the component model. Luke Wagner's talk, ["What is a Component (and Why)?"](https://sched.co/1P96K), is essential viewing for anyone interested in WebAssembly's future.  
Kyle Brown's session, ["Wasm Components for Every Language"](https://sched.co/1PCLo), surveyed how various languages are adapting to the component model. ["Package Management for Wasm Components"](https://sched.co/1PCMF) by Daniel Macovei offered insights into the complexity of using and distributing components via WARG, a WebAssembly registry.

#### A Quick Plug

I also presented a talk on [the role of WebAssembly in building more energy-efficient systems](https://sched.co/1PiwW). Stay tuned for an upcoming blog post on the topic.

<center>
{{< youtube zi4d4Hu1HtY >}}
</center>

### Side Events and BACON

#### Rust Global

Given Rust's significant role in the WebAssembly sphere, a mini-conference focused on Rust was a welcome addition. However, the name "Rust Global" might have been a bit ambitious for a six-talk event.

#### Evening Shenanigans

Wednesday evening saw Fermyon host a "Fermyon Cocktail Affair" in celebration of their latest [Serverless AI announcement](https://www.fermyon.com/blog/introducing-fermyon-serverless-ai). WasmCon's closing event at Lucky Strike Bowling & Arcade made for an excellent networking opportunity, complete with drinks and 4-player PacMan games.

#### Componentize the World Hackathon

Though not officially part of WasmCon, the Byte Code Alliance's "Componentize the World Hackathon" (affectionately known as BACON 🥓) was a standout event. Held at Microsoft in Redmond, the hackathon aimed to deepen understanding of the component model while offering direct access to its creators. The event was [live-streamed](https://www.youtube.com/watch?v=LavA2evpMos), making it accessible to all.

<center>
{{< tweet user="vogti" id="1700299883234902476" >}}
</center>

### The Current State of the WebAssembly Ecosystem

WebAssembly is evolving at a measured yet consistent pace. The industry is still working to establish agreed-upon standards. Among these, the Component Model stands out as a pivotal development, impacting other proposals like Threads and GC. There's a lot of work ahead, especially for languages like Golang and Kotlin, but the potential payoff is immense.

WebAssembly's sweet spot isn't just the browser; it's also making strides in serverless, plugin systems, and increasingly, embedded systems. I eagerly await how the industry will continue to embrace WebAssembly over the next year.

Was the conference worth it? Absolutely! The community is vibrant, filled with incredibly smart and welcoming individuals. The pace of development is rapid yet inclusive. I can't wait for the next WasmCon, and I hope to see you there!
