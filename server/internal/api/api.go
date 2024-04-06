package listen

import (
	"fmt"
	"net/http"
)

type Server struct {

}


func (s Server) ServeHTTP(w http.ResponseWriter, r *http.Request){
    w.WriteHeader(http.StatusOK)

    fmt.Fprintf(w, "lmfao")
}

