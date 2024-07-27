package server

import (
	"context"
	"fmt"

	log "github.com/sirupsen/logrus"
	"github.com/suri312006/term2term/v2/internal/convert"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
	m "github.com/suri312006/term2term/v2/pkg/middleware"
	v2 "github.com/suri312006/term2term/v2/proto-gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type MsgServer struct {
	v2.UnimplementedMsgServiceServer
}

func (s MsgServer) Send(ctx context.Context, msg *v2.MsgSendReq) (*v2.Msg, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	log.Tracef("Preparing to save message")

	var other *v2.User
	fmt.Printf("num of users: %v", len(msg.Convo.Participants.Users))

	for i, v := range msg.Convo.Participants.Users {
		fmt.Printf("user %d : %v|n", i, v)
		if v.Id != msg.Author.Id {
			other = v
		}

	}

	log.Printf("author: %v|n", msg.Author)
	log.Printf("other: %v|n", other)

	dbMsg := db.Message{
		PubId:       id.Must(),
		AuthorId:    msg.Author.Id,
		Author:      convert.User2dbUser(msg.Author),
		RecipientId: other.Id,
		Recipient:   convert.User2dbUser(other),
		ConvoId:     msg.Convo.Id,
		Convo:       convert.Convo2dbConvo(msg.Convo),
		Body:        msg.Body,
		Read:        false,
	}

	if err := dbSesh.Save(&dbMsg); err != nil {
		log.Errorf("Db save failed %v", err)

		return nil, status.Error(codes.Internal, "db fail")
	}

	return convert.DbMsg2Msg(dbMsg), nil

	// return nil, status.Errorf(codes.Unimplemented, "method Send not implemented")
}
func (s MsgServer) Search(ctx context.Context, req *v2.MsgFetchReq) (*v2.MsgList, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}
	return nil, status.Errorf(codes.Unimplemented, "method Search not implemented")
}
