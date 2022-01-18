FROM golang:1.17-buster  as builder

#RUN go get github.com/romana98/NTP/data
#RUN go get github.com/romana98/NTP/routing
WORKDIR /app
#COPY go.mod .
#COPY go.sum .
COPY go.* ./
RUN go mod download

COPY . ./
RUN go build -v -o server


FROM debian:buster-slim
RUN set -x && apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary to the production image from the builder stage.
COPY --from=builder /app/server /server
EXPOSE 8080
# Run the web service on container startup.
CMD ["/server"]