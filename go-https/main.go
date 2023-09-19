package main

import (
	"crypto/tls"
	"fmt"
	"log"
	"net/http"
	"io"
)

func echoHandler(w http.ResponseWriter, r *http.Request) {
	// Read the request body
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Error reading request body", http.StatusInternalServerError)
		return
	}

	// Set the response content type
	w.Header().Set("Content-Type", "text/plain")

	// Write the request body to the response
	fmt.Fprint(w, string(body))
}

func main() {
	// Load the certificate and private key
	cert, err := tls.LoadX509KeyPair("../server.crt", "../server.key")
	if err != nil {
		log.Fatal("Failed to load certificate:", err)
	}

	// Create a TLS configuration with the certificate and private key
	tlsConfig := &tls.Config{
		Certificates: []tls.Certificate{cert},
	}

	// Create an HTTP server with the TLS configuration
	server := &http.Server{
		Addr:      ":8443",
		TLSConfig: tlsConfig,
	}

	http.HandleFunc("/", echoHandler)

	// Start the server
	fmt.Println("Server listening on port 8443...")
	err = server.ListenAndServeTLS("", "")
	if err != nil {
		log.Fatal("Failed to start server:", err)
	}
}

