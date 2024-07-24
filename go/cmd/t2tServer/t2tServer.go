package main

import (
	"context"
	"net"
	"os"

	log "github.com/sirupsen/logrus"
	v2 "github.com/suri312006/term2term/v2/proto-gen"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type userServer struct {
	v2.UnimplementedUserServiceServer
}

func (s userServer) VerifyUser(ctx context.Context, req *v2.VerifyUserReq) (*v2.VerifyUserRes, error) {
	return nil, status.Errorf(codes.Unimplemented, "method VerifyUser not implemented")
}
func (s userServer) SearchUser(ctx context.Context, user *v2.User) (*v2.UserList, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SearchUser not implemented")
}
func (s userServer) Create(ctx context.Context, req *v2.NewUserReq) (*v2.User, error) {
	return nil, status.Errorf(codes.Unimplemented, "lmao creating new user")
}

func init() {
	log.SetFormatter(&log.JSONFormatter{})
	log.SetOutput(os.Stdout)
	log.SetLevel(log.InfoLevel)
}

func main() {

	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen on port 50051: %v", err)
	}

	s := grpc.NewServer()

	v2.RegisterUserServiceServer(s, &userServer{})

	log.Printf("gRPC server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
