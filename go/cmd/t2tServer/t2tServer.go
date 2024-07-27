package main

import (
	"net"
	"os"

	log "github.com/sirupsen/logrus"
	"github.com/suri312006/term2term/v2/internal/config"
	"github.com/suri312006/term2term/v2/internal/db"
	api "github.com/suri312006/term2term/v2/pkg/server"
	"google.golang.org/grpc"

	"github.com/grpc-ecosystem/go-grpc-middleware/v2/interceptors/recovery"
	m "github.com/suri312006/term2term/v2/pkg/middleware"
)

func init() {
	log.SetFormatter(&log.JSONFormatter{})
	log.SetOutput(os.Stdout)
	log.SetLevel(log.TraceLevel)
}

func main() {

	cfg := config.Source()

	dbSession := db.Init(cfg)

	lis, err := net.Listen("tcp", cfg.Port)
	if err != nil {
		log.Fatalf("failed to listen on port %s: %v",cfg.Port, err)
	}
	// Shared options for the logger, with a custom gRPC code to log level function.
	opts := []recovery.Option{
		recovery.WithRecoveryHandler(m.RecoverPanic),
	}

	s := grpc.NewServer(
		grpc.ChainStreamInterceptor(
			m.DBStreamServerInterceptor(&dbSession),
			recovery.StreamServerInterceptor(opts...),
		),
		grpc.ChainUnaryInterceptor(
			m.DBUnaryServerInterceptor(&dbSession),
			m.TraceUnaryServerInterceptor(),
			// recovery.UnaryServerInterceptor(opts...),
		),
	)

	api.AttachServers(s)

	log.Printf("gRPC server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
