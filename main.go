package main

import (
	"fmt"
	"net/http"
)

func main(){
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request){
		fmt.Fprintf(w, "Hello you have requet %s\n", r.URL.Path)
	})

	var e = http.ListenAndServe(":8000", nil);
	if e != nil {
		fmt.Printf("%s\n", e.Error());
	}
}
