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

type ConvoServer struct {
	v2.UnimplementedConvoServiceServer
}

func (s ConvoServer) Create(ctx context.Context, req *v2.NewConvoReq) (*v2.Convo, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		log.Fatal("database connection not provided")
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	return nil, status.Errorf(codes.Unimplemented, "method Create not implemented")
}
