package main

import (
	"context"
	"github.com/joho/godotenv"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/logging"
	"github.com/romana98/NTP/routing"
	"github.com/rs/cors"
	"net/http"
	"os"
	"os/signal"
	"time"
)


func main() {
	// load .env file
	err := godotenv.Load(".env")

	logging.Logger()

	if err != nil {
		logging.ErrorLogger.Println(err)
	}

	data.InitDatabase()

	r := routing.NewRouter()

	cf := cors.New(cors.Options{
		AllowedOrigins:   []string{"http://localhost:4200"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Authorization", "Page", "PerPage", "Content-Type"},
		AllowCredentials: true,
		Debug:            true,
	})

	s := http.Server{
		Addr:         ":8080",           // configure the bind address
		Handler:      cf.Handler(r),     // set the default handler
		ErrorLog:     nil,               // set the logger for the server
		ReadTimeout:  10 * time.Second,  // max time to read request from the client
		WriteTimeout: 20 * time.Second,  // max time to write response to the client
		IdleTimeout:  120 * time.Second, // max time for connections using TCP Keep-Alive
	}

	go func() {
		logging.InfoLogger.Println("Serving on port 8080")
		err := s.ListenAndServe()
		if err != nil {
			logging.ErrorLogger.Println(err)
			return
		}
	}()

	// trap sigterm or interrupt and gracefully shutdown the server
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	signal.Notify(c, os.Kill)

	// Block until a signal is received.
	sig := <-c
	logging.InfoLogger.Println("Got signal:", sig, " - shutting down")

	// gracefully shutdown the server, waiting max 30 seconds for current operations to complete
	ctx, _ := context.WithTimeout(context.Background(), 30*time.Second)
	err = s.Shutdown(ctx)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return
	}
}
