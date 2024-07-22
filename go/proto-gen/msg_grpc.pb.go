// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v5.27.0
// source: msg.proto

package v2

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	MsgHandler_SendMsg_FullMethodName = "/t2t.MsgHandler/SendMsg"
)

// MsgHandlerClient is the client API for MsgHandler service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type MsgHandlerClient interface {
	SendMsg(ctx context.Context, in *Msg, opts ...grpc.CallOption) (*MsgSendRes, error)
}

type msgHandlerClient struct {
	cc grpc.ClientConnInterface
}

func NewMsgHandlerClient(cc grpc.ClientConnInterface) MsgHandlerClient {
	return &msgHandlerClient{cc}
}

func (c *msgHandlerClient) SendMsg(ctx context.Context, in *Msg, opts ...grpc.CallOption) (*MsgSendRes, error) {
	out := new(MsgSendRes)
	err := c.cc.Invoke(ctx, MsgHandler_SendMsg_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MsgHandlerServer is the server API for MsgHandler service.
// All implementations must embed UnimplementedMsgHandlerServer
// for forward compatibility
type MsgHandlerServer interface {
	SendMsg(context.Context, *Msg) (*MsgSendRes, error)
	mustEmbedUnimplementedMsgHandlerServer()
}

// UnimplementedMsgHandlerServer must be embedded to have forward compatible implementations.
type UnimplementedMsgHandlerServer struct {
}

func (UnimplementedMsgHandlerServer) SendMsg(context.Context, *Msg) (*MsgSendRes, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SendMsg not implemented")
}
func (UnimplementedMsgHandlerServer) mustEmbedUnimplementedMsgHandlerServer() {}

// UnsafeMsgHandlerServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to MsgHandlerServer will
// result in compilation errors.
type UnsafeMsgHandlerServer interface {
	mustEmbedUnimplementedMsgHandlerServer()
}

func RegisterMsgHandlerServer(s grpc.ServiceRegistrar, srv MsgHandlerServer) {
	s.RegisterService(&MsgHandler_ServiceDesc, srv)
}

func _MsgHandler_SendMsg_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Msg)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgHandlerServer).SendMsg(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: MsgHandler_SendMsg_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgHandlerServer).SendMsg(ctx, req.(*Msg))
	}
	return interceptor(ctx, in, info, handler)
}

// MsgHandler_ServiceDesc is the grpc.ServiceDesc for MsgHandler service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var MsgHandler_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "t2t.MsgHandler",
	HandlerType: (*MsgHandlerServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "SendMsg",
			Handler:    _MsgHandler_SendMsg_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "msg.proto",
}
