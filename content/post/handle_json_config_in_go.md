+++
title = "How to handle JSON configs in Go"
draft = false
date = "2016-10-20T20:37:05+02:00"

+++

At a certain point of development it becomes good practice to decouple the config from the code. Since I'm coming from a Node.js background, I'm used to use JSON for configuration. Luckily the Go standard library already comes with JSON support.


The benefits of JSON are that it is fairly simple to parse and human readable/editable while offering semantics for lists and mappings (which can become quite handy), which is not the case with many ini-type config parsers.

#### Example Usage

Lets assume we have following JSON config:

**config.json**
```
{
    "host": "http://host.name",
    "path": "/api/v1/you/know/the/route/",
    "apikey": "abcdefg"
}
```

Reading this config can look like this:

**main.go**
```go
package main

import (
	"fmt"
	"encoding/json"
	"os"
)

// Create a configuration struct
type Configuration struct {
	Host   string `json:"host"`
	Path   string `json:"path"`
	Apikey string `json:"apikey"`
}

var Urlpath string

func main() {

	// Open config file
	configFile, _ := os.Open("config.json")

	decoder := json.NewDecoder(configFile) 	// Initialize a JSON decoder
	c := Configuration{}                    //
	err := decoder.Decode(&c)               // Decode JSON and save it in c

	if err != nil {
		fmt.Println("Could not Read Configuration", err)
		return
	}

	Urlpath = c.Host + c.Path
	fmt.Println(Urlpath)

}
```

So running our program would output the following:

```
$ go run main.go
http://host.name/api/v1/you/know/the/route/
```

#### Extend the `Configuration struct`

main.go
```go
[...]

type Configuration struct {
	Host   string `json:"host"`
	Path   string `json:"path"`
	Apikey string `json:"apikey"`
}

// this will allow to access the whole URL via c.UrlPath()
func (c *Configuration) UrlPath() string {
	return c.Host + c.Path
}

func (c *Configuration) ApiSuffix() string {
	return "?apikey=" + c.Apikey
}

func main() {

	configFile, _ := os.Open("config.json")
	decoder := json.NewDecoder(configFile)

	c := Configuration{}
	err := decoder.Decode(&c)

	if err != nil {
		fmt.Println("Could not Read Configuration", err)
		return
	}

	fmt.Println(c.UrlPath() + "someParameter" + c.ApiSuffix())

}
```

Of course, other config formats, such as YAML or TOML would also do their work, but they'd need to be added separately. JSON is build in and awesome ;)