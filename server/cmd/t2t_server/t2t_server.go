package main

import (
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/joho/godotenv"
	api "github.com/suri312006/term2term/v2/internal/api"
)

func main() {
	//TODO: create db here, and pass it into a db wrapper?

	err := godotenv.Load("../../../.env")

	if err != nil {
		log.Fatalf("Error loading .env file")
	}

	serverAdress := os.Getenv("SERVER_HOST_ADDRESS")

	println("server adress", serverAdress)

	port := strings.TrimPrefix(serverAdress, "http://localhost")
	port = strings.TrimSuffix(port, "/")
	

	println("port adress", port)

	server := api.Server{}
	log.Fatal(http.ListenAndServe(port, server))

}
