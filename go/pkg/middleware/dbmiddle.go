package middleware

import (
	"context"

	grpc_middleware "github.com/grpc-ecosystem/go-grpc-middleware"
	"github.com/suri312006/term2term/v2/internal/db"
	"google.golang.org/grpc"
)

type ContextKey string

const (
	DBSession ContextKey = "dbSession"
)

func DBUnaryServerInterceptor(session *db.Dbm) grpc.UnaryServerInterceptor {
	return func(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
		return handler(context.WithValue(ctx, DBSession, session), req)
	}
}

func DBStreamServerInterceptor(session *db.Dbm) grpc.StreamServerInterceptor {
	return func(srv interface{}, stream grpc.ServerStream, info *grpc.StreamServerInfo, handler grpc.StreamHandler) error {
		wrapped := grpc_middleware.WrapServerStream(stream)
		wrapped.WrappedContext = context.WithValue(stream.Context(), DBSession, session)
		return handler(srv, wrapped)
	}
}
