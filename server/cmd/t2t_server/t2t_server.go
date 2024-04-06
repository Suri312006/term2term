package main

import (
	"log"
	"net/http"

	api "github.com/suri312006/term2term/v2/internal/api"
)

func main(){
    server := api.Server{}
	log.Fatal(http.ListenAndServe(":6969", server))
}
