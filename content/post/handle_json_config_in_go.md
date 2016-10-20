+++
title = "How to handle JSON configs in Go"
draft = true
date = "2016-10-20T20:37:05+02:00"

+++

At a certain point of development it becomes good practice to decouple the config from the code. Since I'm coming from a Node.js background, I'm used to use JSON for configuration. Luckily the Go standard library already comes with JSON support.


The benefits of JSON are that it is fairly simple to parse and human readable/editable while offering semantics for lists and mappings (which can become quite handy), which is not the case with many ini-type config parsers.

Example Usage

**config.json**
```
{
    "host": "http://host.name",
    "path": "/api/v1/you/know/the/route/",
    "apikey": "abcdefg"
}
```

**main.go**
```go
package main

import (
	"encoding/json"
	"os"
)

type Configuration struct {
	Host   string
	Path   string
	Apikey string
}

func (c *Configuration) UrlPath() string {
	return c.Host + c.Path
}

func (c *Configuration) ApiSuffix() string {
	return "?apikey=" + c.Apikey
}

func main() {

	configureApp("config.json")

}

func configureApp(file string) {

	// read configuraiton
	configFile, _ := os.Open(file)
	decoder := json.NewDecoder(configFile)
	c := Configuration{}
	err := decoder.Decode(&c)
	if err != nil {
		fmt.Println("Could not Read Configuration", err)
		return
	}

	Urlpath = c.UrlPath()
	Apisuffix = c.ApiSuffix()

}
```

Of course, other config formats, such as YAML or TOML would also do their work, but unfortunately they'd need to be added separately.