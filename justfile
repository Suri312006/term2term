set dotenv-load
alias rcc:=run_client_cli
alias rct:=run_client_tui
alias rs:=run_server

run_client_cli:
    cd ./t2t-cli/ && cargo run
run_client_tui:
    cd ./t2t/ && cargo run

run_server:
    cd ./hermes && cargo run

deploy:
    cp proto ./t2t-server/ -r
    cd ./t2t-server && fly deploy
    rm -rf ./t2t-server/proto

migrate CMD="":
     cd ./t2t-server/migration/ && DATABASE_URL=$DB_URL cargo run -- {{CMD}}

migen NAME="":
     cd ./t2t-server/migration/ && DATABASE_URL=$DB_URL cargo run -- generate {{NAME}}

entitygen:
    cd ./t2t-server/ && sea-orm-cli generate entity -u $DB_URL -o src/entities \
    --with-serde both --model-extra-derives Default


