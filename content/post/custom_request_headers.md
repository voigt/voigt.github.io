+++
title = "Golang net/http - Set Custom Request Headers"
draft = true
date = "2016-10-24T20:37:05+02:00"

+++


[http.Post](https://golang.org/pkg/net/http/#Client.Post)
[http.Get](https://golang.org/pkg/net/http/#Client.Get)

but consequently no http.Put or http.Del (not to mention PATCH). To me this looks a little inconsistent to only implement shortcuts for two HTTP methods.
However, it's pretty straight forward to implement those functions by your own:


To create a request with a custom header we need to create a `NewRequest` and use `client.Do` to execute it:

```go
# Build up the request
	req, err := http.NewRequest("GET", url, nil)
	req.Header.Set("Content-Type", "application/json")


# Setup the client and execute
    client := &http.Client{}
	resp, err := client.Do(req)
    if err != nil {
		log.Panic(err)
	}
	defer resp.Body.Close()
```
