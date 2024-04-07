package main

import (
	"log"
	"net/http"

	api "github.com/suri312006/term2term/v2/internal/api"
)

func main() {
	//TODO: create db here, and pass it into a db wrapper?
	server := api.Server{}
	log.Fatal(http.ListenAndServe(":6969", server))
	
}
