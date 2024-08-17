package server

import (
	v2 "github.com/suri312006/term2term/v2/proto-gen"
	"google.golang.org/grpc"
)

func AttachServers(s *grpc.Server) {
	v2.RegisterUserServiceServer(s, &UserServer{})
	v2.RegisterMsgServiceServer(s, &MsgServer{})
	v2.RegisterConvoServiceServer(s, &ConvoServer{})
}
