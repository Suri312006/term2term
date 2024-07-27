package server

import (
	"context"

	log "github.com/sirupsen/logrus"
	"github.com/suri312006/term2term/v2/internal/convert"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
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

	convo := db.Conversation{
		PubId:   id.Must(),
		User1Id: req.Participants.Users[0].Id,
		User2Id: req.Participants.Users[1].Id,
		User1:   convert.User2dbUser(req.Participants.Users[0]),
		User2:   convert.User2dbUser(req.Participants.Users[1]),
	}

	if err := dbSesh.Save(&convo); err != nil {
		log.Errorf("Unable to execute db save on conversation: %v", err)
		return nil, status.Error(codes.Internal, "dont worry about it fr")
	}

	return convert.DbConvo2Convo(convo), nil
}
