let input = document.getElementById("input") as HTMLTextAreaElement;
let output = document.getElementById("output");
let socket = new WebSocket("ws://localhost:8080/ws");

enum GaemLoop{
	None,
    Start   = 0x00,
    Attack  = 0x01,
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
				this.loop = this.id != 0 ? GaemLoop.Receive : GaemLoop.Attack;
				break;
			case GaemLoop.Attack:
				let x = this.attack();
				let m = `Hit;${x[0]};${x[1]}`
				send(m);
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

	rand_int(){
		return Math.round( Math.random() * 10 )
	}

	attack() {
		return [this.rand_int(), this.rand_int()]
	}
}
let gaem = new Gaem();
socket.onopen = function () {
    socket.send("Ok");
};

socket.onmessage = function (e) {
    gaem.tick(e.data);
}

function send(data: string) {
	console.log("sending:", data);
    socket.send(data);
}

export {Gaem, gaem, send}
