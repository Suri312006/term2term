PROTO_DIR := ./proto
PROTO_FILES := $(PROTO_DIR)/t2t.proto
OUT_DIR := server/

.PHONY: all proto

all: proto

proto:
	protoc --go_out=$(OUT_DIR) --go_opt=paths=source_relative \
		--go-grpc_out=$(OUT_DIR) --go-grpc_opt=paths=source_relative \
		$(PROTO_FILES)
