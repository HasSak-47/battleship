package main

import (
    "fmt"
    "net/http"

    "github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
}

type Player struct{
	id int
};

type Gaem struct{
	player_turn int
	players int
}
 
func main() {
	gaem := Gaem{player_turn:  0, players: 0};
    http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
        conn, _ := upgrader.Upgrade(w, r, nil) // error ignored for sake of simplicity
		id := gaem.players;
		gaem.players += 1;
        for {
            // Read message from browser
			var msg = []byte(fmt.Sprintf("SetId;%d", id));
			conn.WriteMessage(websocket.TextMessage, msg);
            _, msg, err := conn.ReadMessage()
            if err != nil {
                return
            }

            // Print the message to the console
            // fmt.Printf("%s sent: %s\n", conn.RemoteAddr(), string(msg))
        }
    })

    fs := http.FileServer(http.Dir("static/"))
    http.Handle("/", http.StripPrefix("/", fs))

    http.ListenAndServe(":8080", nil)
}
