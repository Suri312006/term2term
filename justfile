set dotenv-load
alias rc:=run_client
alias rs:=run_server

run_client:
    cd ./t2t-client/ && cargo run

run_server:
    cd ./t2t-server/ && cargo run 

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



