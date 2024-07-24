# PROTO_DIR := ./proto
# PROTO_FILES := $(PROTO_DIR)/*.proto
# OUT_DIR := server/
#
# .PHONY: all proto
#
# all: proto
#
# proto:
# 	cd proto
# 	protoc --go_out=$(OUT_DIR) --go_opt=paths=source_relative \
# 		--go-grpc_out=$(OUT_DIR) --go-grpc_opt=paths=source_relative \
# 		$(PROTO_FILES)

PROTO_DIR := ./proto
OUT_DIR := ./go/proto-gen

# List all proto files
PROTO_FILES := $(wildcard $(PROTO_DIR)/*.proto)

.PHONY: all
all: gen

.PHONY: gen
gen:
	protoc -I=$(PROTO_DIR) --go_out=$(OUT_DIR) --go_opt=paths=source_relative \
		--go-grpc_out=$(OUT_DIR) --go-grpc_opt=paths=source_relative \
		$(PROTO_FILES)
	cd rs/ && cargo build
	cd ../
