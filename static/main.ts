let input = document.getElementById("input") as HTMLTextAreaElement;
let output = document.getElementById("output");
let socket = new WebSocket("ws://localhost:8080/ws");

enum GaemLoop{
	None,
    Start   = 0x00,
    Hit     = 0x01,
    Receive = 0x02,
    End     = 0x10,
	Win     = 0x01,
	Lose    = 0x02,
}

class Gaem {
	loop : GaemLoop
	id : Number | null
    constructor() {
        this.loop = GaemLoop.Start;
        this.id = null;
    }

    tick(msg: string) {
		console.log(this);
		console.log(msg);
        let cols = msg.split(';');

		switch(this.loop){
			case GaemLoop.Start:
				this.id = parseInt(cols[1]);
				this.loop = this.id == 0? GaemLoop.Receive : GaemLoop.Hit;
			case GaemLoop.Hit:
				let x = this.attack();
				socket.send(`Hit;${x[0]} ${x[1]}`);
				break;
			case GaemLoop.Receive:
				break;
			case GaemLoop.End | GaemLoop.Win:
				break;
			case GaemLoop.End |GaemLoop.Lose:
				break;
			default:
				break;
		}
    }

	attack() {
		return [Math.random(), Math.random()]
	}
}
let gm = new Gaem();
socket.onopen = function () {
    socket.send("Ok");
};
socket.onmessage = function (e) {
    gm.tick(e.data);
}

function send() {
    socket.send(input.value);
}
