package main

import (
	"log"
	"net/http"

	api "github.com/suri312006/term2term/v2/internal/api"
)

const dbFilePath = "/home/suri/coding/term2term/server/internal/db/db.json"

func main() {
	server := api.Server{}
	log.Fatal(http.ListenAndServe(":6969", server))
	

	
}
