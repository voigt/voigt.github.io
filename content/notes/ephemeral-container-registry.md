---
title: Ephemeral Container Registry
date: 2025-05-23
published: 2025-05-23
author: Christoph Voigt
showAuthor: false
tags:
  - containers
  - notes
---
After the third time remembering, but not finding the ephemeral container registry service it is time to write a note about it.

 https://ttl.sh/ is a neat service to push an image, test it (e.g. during CI) and forget about it. The image will be automagically deleted, based on a defined time.

```bash
# linux
$ IMAGE_NAME=$(uuidgen)
# macos
$ IMAGE_NAME=$(uuidgen | awk '{print tolower($0)}')

$ docker build -t ttl.sh/${IMAGE_NAME}:1h .
$ docker push ttl.sh/${IMAGE_NAME}:1h
 
 ................................................
 image ttl.sh/xxxx-yyyy-nnnn-2a2222-4b44 is available for 1 hour
 
 ttl.sh is contributed by Replicated (www.replicated.com)
```