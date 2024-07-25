package server

import (
	"context"

	"github.com/suri312006/term2term/v2/internal/db"
	m "github.com/suri312006/term2term/v2/pkg/middleware"
	v2 "github.com/suri312006/term2term/v2/proto-gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type MsgServer struct {
	v2.UnimplementedMsgServiceServer
}

func (s MsgServer) Send(ctx context.Context, msg *v2.Msg) (*v2.MsgSendRes, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}
	return nil, status.Errorf(codes.Unimplemented, "method Send not implemented")
}
func (s MsgServer) Search(ctx context.Context, req *v2.MsgFetchReq) (*v2.MsgList, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}
	return nil, status.Errorf(codes.Unimplemented, "method Search not implemented")
}
