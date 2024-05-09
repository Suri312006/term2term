package main

import (
	"github.com/suri312006/term2term/v2/internal/apiserver"
	"github.com/suri312006/term2term/v2/internal/config"
	"github.com/suri312006/term2term/v2/internal/db"
)

func main() {
	env := config.Source()
	db := db.Init(env)
	apiserver := apiserver.Init(env, db)
	apiserver.Start()
}
