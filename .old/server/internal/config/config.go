package config

import (
	"os"

	"github.com/joho/godotenv"
)

type Env struct {
	Port     string
	DBString string
}

func Source() Env {
	godotenv.Load("../../.env")

	port := find("PORT")

	port = ":" + port

	dbstring := find("DBSTRING")

	return Env{
		Port:     port,
		DBString: dbstring,
	}

}

func find(envVar string) string {
	val, found := os.LookupEnv(envVar)
	if !found {
		panic(envVar + " not found.")
	}

	return val
}
