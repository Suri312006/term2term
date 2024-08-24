set dotenv-load

default:
    echo 'Hello, world!'

migrate CMD="":
     cd ./t2t-server/migration/ && DATABASE_URL=$DB_URL cargo run -- {{CMD}}

