FROM golang:1.22 

# Set the working directory inside the container
WORKDIR /app

# Copy the entire project directory into the container
COPY . ./

RUN pwd

RUN ls
# Enable Go modules
ENV GO111MODULE=on
ENV GOPROXY=https://proxy.golang.org

WORKDIR /app
# Run go mod download to download dependencies
RUN go mod download

RUN ls /app/internal/apiserver

RUN ls /app/cmd/t2tServer

# Change working directory to the directory containing the main package
WORKDIR /app/cmd/t2tServer

# Build the Go binary
RUN CGO_ENABLED=0 GOOS=linux go build -o /t2tServer

# Expose port 8080 for incoming traffic
EXPOSE 8080 5432

#TODO: need to do the following for env vars

# Specify the variable you need
ARG  DBSTRING
# Use the varible
RUN export DBSTRING=$DBSTRING

# Specify the variable you need
ARG  PORT
# Use the varible
RUN export PORT=$PORT

# Command to run the executable when the container starts
CMD ["/t2tServer"]
