protoc --go_out=../go/proto --go_opt=paths=source_relative \                                       ─╯
--go-grpc_out=../go/proto --go-grpc_opt=paths=source_relative \
helloworld.proto
