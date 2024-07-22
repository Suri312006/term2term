package main

import (
	"flag"

	"github.com/joho/godotenv"
	"github.com/suri312006/term2term/v2/internal/config"
	"github.com/suri312006/term2term/v2/internal/db"
)

func main() {
	godotenv.Load("../../../.env")

	// resetSchemaPtr := flag.Bool("r", false, "reset schema")
	// updateSchemaPtr := flag.Bool("u", false, "update schema")
	// testPtr := flag.Bool("t", false, "test")

    flag.Parse()

    config := config.Source()

    dbm := db.Init(config)

    dbm.ResetSchema()
}
