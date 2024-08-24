set dotenv-load

default:
    echo 'Hello, world!'

migrate CMD="":
     cd ./t2t-server/migration/ && DATABASE_URL=$DB_URL cargo run -- {{CMD}}

migen NAME="":
     cd ./t2t-server/migration/ && DATABASE_URL=$DB_URL cargo run -- generate {{NAME}}

entitygen:
    cd ./t2t-server/ && sea-orm-cli generate entity -u $DB_URL -o src/entities

