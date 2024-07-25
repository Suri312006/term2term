package middleware

import (
	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func RecoverPanic(p any) (err error) {
	log.Warn("Recovering from panic")
	return status.Errorf(codes.Internal, "Internal Server Error")
}
