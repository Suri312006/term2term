package server

import (
	"context"

	log "github.com/sirupsen/logrus"
	"github.com/suri312006/term2term/v2/internal/db"
	m "github.com/suri312006/term2term/v2/pkg/middleware"
	v2 "github.com/suri312006/term2term/v2/proto-gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type UserServer struct {
	v2.UnimplementedUserServiceServer
}

func (s UserServer) VerifyUser(ctx context.Context, req *v2.VerifyUserReq) (*v2.VerifyUserRes, error) {

	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		log.Fatal("database connection not provided")
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	return nil, status.Errorf(codes.Unimplemented, "method VerifyUser not implemented")
}

func (s UserServer) SearchUser(ctx context.Context, user *v2.User) (*v2.UserList, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	return nil, status.Errorf(codes.Unimplemented, "method SearchUser not implemented")
}

func (s UserServer) Create(ctx context.Context, req *v2.NewUserReq) (*v2.User, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	// log.Fatal("cerating new user")
	log.Panic("error creating new user")
	return nil, status.Errorf(codes.Unimplemented, "lmao creating new user")
}