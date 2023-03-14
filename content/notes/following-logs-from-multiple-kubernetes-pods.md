---
title: "Following logs of multiple Kubernetes Pods"
date: 2019-12-29
author: Christoph Voigt
showAuthor: false
description: "Follow logs of multiple Kubernetes pods via a label selector."
tags:
  - kubernetes
  - notes
---

Following your logs is a crucial part of application and operation debugging. `tail -f` is every sysadmins best friend - easy, fast, and `grep`able!

While centralized logging solutions such as ELK/EFK, Splunk or Grafana Loki are almost common sense, they are some times overkill. Also, they are not accessible from the command line - which is usually the center of your debugging operations.

To stream logs of Kubernetes workloads we are already used to:

```bash
$ kubctl -n <namespace> logs -f <pod-id>
```

But what if you want to read log interactions of different workloads? What if you want to see the logs of all replicas of that same pod?

For this purpose Kubernetes allows log streaming based on label selectors:

```bash
$ kubectl -n namespace logs -l app=myapp
```

This will stream logs of all Pods with the label `app=myapp` to your console.

As already mentioned, this does not only affect pods of the same type but all pods sharing the same label. This might include Replicasets and Jobs as well!

Of course, `grep`ing and other magic is possible as well ❤.
