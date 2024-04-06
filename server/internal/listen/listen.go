package listen

import "net/http"

type Server struct {
}


func (s *Server) ServeHttp(w http.ResponseWriter, r *http.Request){
    w.WriteHeader(http.StatusOK)
}

