package logging

import (
	"log"
	"os"
	"path/filepath"
)

var (
	InfoLogger    *log.Logger
	ErrorLogger   *log.Logger
)

func Logger() {
	absPath, _ := filepath.Abs(os.Getenv("LOG_STORAGE"))
	file, err := os.OpenFile(absPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0666)
	if err != nil {
		log.Fatal(err)
	}

	InfoLogger = log.New(file, "INFO: ", log.Ldate|log.Ltime|log.Lshortfile)
	ErrorLogger = log.New(file, "ERROR: ", log.Ldate|log.Ltime|log.Lshortfile)
}
