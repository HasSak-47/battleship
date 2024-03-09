package main

import (
	"fmt"
	"net/http"
	"strconv"
	"strings"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
}

type Cell uint8
const (
	Empty Cell = 0
	Ship  Cell = 1
	Hit   Cell = 2
)

type GaemLoop uint8
const (
    Start   GaemLoop = 0x00
    Attack  GaemLoop = 0x01
    Receive GaemLoop = 0x02
    End     GaemLoop = 0x10
	Win     GaemLoop = 0x01
	Lose    GaemLoop = 0x02
	None    GaemLoop = 0xff
)

type Player struct{
	id int
	loop GaemLoop
	board [10][10]Cell
}

type Gaem struct{
	player_turn int
	player_count int
	players [2]Player
}

var hit_msgs = []string{
	"Hit",
	"Miss",
};

func send_msg(w *websocket.Conn, msg  string){
	fmt.Printf("sending msg %s\n", msg);
	w.WriteMessage(websocket.TextMessage, []byte(msg));
}

func read_msg(w *websocket.Conn) (string){
	_, msg, _ := w.ReadMessage();
	str := string(msg[:])
	fmt.Printf("Received message: %s\n", str);
	return str;
}

func main() {
	gaem := Gaem{player_turn:  0, player_count: 0};
    http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
        conn, _ := upgrader.Upgrade(w, r, nil) // error ignored for sake of simplicity

		// set up player state
		id := gaem.player_count;
		player := gaem.players[id];
		player.id = id;
		if id == 0{
			player.loop = Attack
		} else {
			player.loop = Receive
		}

		gaem.player_count += 1;
		send_msg(conn, fmt.Sprintf("SetId;%d", id));
		read_msg(conn); // ok msg
        for {
			if player.loop == Attack{
				send_msg(conn, "Attack");
				msg := read_msg(conn);
				println(msg)
				cols := strings.Split(msg, ";");

				x, _ := strconv.Atoi(cols[1]);
				y, _ := strconv.Atoi(cols[2]);

				other_id := (^id) & 1;
				ok_hit := gaem.players[other_id].board[x][y];
				gaem.players[other_id].board[x][y] |= Hit;
				send_msg(conn, hit_msgs[ok_hit])
				player.loop = Receive;
				continue;
			}

			if player.loop == Receive{

			}
        }
    })

    fs := http.FileServer(http.Dir("static/"))
    http.Handle("/", http.StripPrefix("/", fs))

    http.ListenAndServe(":8080", nil)
}
