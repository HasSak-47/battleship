package main

import (
	"fmt"
	"net/http"
	// "html/template"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
}

func send_msg(w *websocket.Conn, msg  string){
	w.WriteMessage(websocket.TextMessage, []byte(msg));
}

func read_msg(w *websocket.Conn) (string){
	_, msg, _ := w.ReadMessage();
	str := string(msg[:])
	return str;
}

type Connection struct {
	conA *websocket.Conn
	conB *websocket.Conn
}

func process(msg string){
}

func echoCon(cons *Connection){
	echo := func(A **websocket.Conn, B **websocket.Conn){
		for{
			if *A == nil || *B == nil{
				continue
			}
			msg := read_msg(*A)
			send_msg(*B, msg)
		}
	}

	go echo(&cons.conA, &cons.conB)
	go echo(&cons.conB, &cons.conA)
}

func main() {
	// temps := template.Must(template.ParseGlob("static/*.html"))
	count := 0;
	connection := Connection{ conA: nil, conB: nil};
    http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
        conn, _ := upgrader.Upgrade(w, r, nil)
		send_msg(conn, fmt.Sprintf("Id: %d", count))
		count++
		if connection.conA == nil{
			connection.conA = conn
		}else if connection.conB == nil{
			connection.conB = conn	
		}
    })

	go echoCon(&connection)

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request){
		fmt.Printf("test!!\n")
		w.Write([]byte("test string"))
	})

    http.ListenAndServe(":8080", nil)
}
