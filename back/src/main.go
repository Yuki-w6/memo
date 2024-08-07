package main

import (
    "fmt"
    "net/http"
    "os"
)

func handler(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "Hello, %s!", r.URL.Path[1:])
}

func main() {
    http.HandleFunc("/", handler)

    err := http.ListenAndServe(":18080", nil)
    if err != nil {
        fmt.Printf("failed to terminate server: %v", err)
        os.Exit(1)
    }
}