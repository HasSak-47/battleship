package main

import (
	"fmt"
	"html/template"
	"io"

	"github.com/gorilla/websocket"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

var upgrader = websocket.Upgrader{}

func write_msg(w *websocket.Conn, msg  string){
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

func process(msg string) (string, string){
	echo_back := msg + " back"
	echo_forward := msg

	return echo_back, echo_forward
}

func echoCon(cons *Connection){
	echo := func(A **websocket.Conn, B **websocket.Conn){
		for{
			if *A == nil || *B == nil{
				continue
			}
			msg := read_msg(*A)
			msgA, msgB := process(msg)
			write_msg(*A, msgA)
			write_msg(*B, msgB)
		}
	}

	go echo(&cons.conA, &cons.conB)
	go echo(&cons.conB, &cons.conA)
}

type Templates struct {
	templates *template.Template
}

func newTemplates() Templates {
	return Templates{
		templates: template.Must(template.ParseGlob("static/*.html")),
	}
}

func (t *Templates) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return t.templates.ExecuteTemplate(w, name, data)
}

type Cell string

const (
	EMPTY Cell = "."
	SHIP       = "o"
	MISS       = "-"
	HIT        = "x"
)

func newCell() Cell{
	return EMPTY
}

type Row struct {
	Cells [10]Cell }

func newRow() Row{
	row := Row{ Cells : [10]Cell{}, }
	for i := range row.Cells {
		row.Cells[i] = newCell()
	}
	return row
}

type Board struct { Rows [10]Row }

func new_board() Board{
	board := Board { Rows : [10]Row{}, }
	for i := range board.Rows{
		board.Rows[i] = newRow()
	}
	return board 
}

func (b *Board) set(x int, y int, val Cell){
	b.Rows[x].Cells[y] = val;
}

type Player struct{
	name string
}

func newPlayer(name string) Player{
	return Player{name: name}
}

type PlayerGame struct{
	Name string
	Eboard Board
	Pboard Board
}

func main() {
	temps := newTemplates();
	connection := Connection{ conA: nil, conB: nil};
	e := echo.New()
	count := 0
	e.Use(middleware.Recover())
	e.Renderer = &temps;

	e.Static("/", "static/")
	e.GET("/", func(c echo.Context) error {
		return c.Render(200, "index", nil);
	});


	e.POST("/play", func(c echo.Context) error {
		game := PlayerGame{
			Name: c.FormValue("name"),
			Eboard: new_board(),
			Pboard: new_board(),
		}
		count++;

		err := c.Render(200, "game", game);
		if err != nil{
			fmt.Println(err);
		}

		return err
	})

	e.GET("/ws", func(c echo.Context) error {
		conn, err := upgrader.Upgrade(c.Response(), c.Request(), nil)
		if err != nil{
			fmt.Println(err);
			return err
		}
		if connection.conA == nil{
			connection.conA = conn
		}else{
			connection.conB = conn
		}
		return nil
	})


	go echoCon(&connection)

	e.Logger.Fatal(e.Start(":8080"));
}
