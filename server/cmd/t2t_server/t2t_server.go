package main

import (
	"log"
	"net/http"
	"github.com/suri312006/term2term/v2/internal/config"
)





func main() {
	//TODO: create db here, and pass it into a db wrapper?


	env := config.Source()
	

	server := api.Server{}
	log.Fatal(http.ListenAndServe(port, server))

}
