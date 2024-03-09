let input = document.getElementById("input");
let output = document.getElementById("output");
let socket = new WebSocket("ws://localhost:8080/ws");
var GaemLoop;
(function (GaemLoop) {
    GaemLoop[GaemLoop["None"] = 0] = "None";
    GaemLoop[GaemLoop["Start"] = 0] = "Start";
    GaemLoop[GaemLoop["Attack"] = 1] = "Attack";
    GaemLoop[GaemLoop["Receive"] = 2] = "Receive";
    GaemLoop[GaemLoop["End"] = 16] = "End";
    GaemLoop[GaemLoop["Win"] = 1] = "Win";
    GaemLoop[GaemLoop["Lose"] = 2] = "Lose";
})(GaemLoop || (GaemLoop = {}));
class Gaem {
    constructor() {
        this.loop = GaemLoop.Start;
        this.id = null;
    }
    tick(msg) {
        console.log(this);
        console.log(msg);
        let cols = msg.split(';');
        switch (this.loop) {
            case GaemLoop.Start:
                this.id = parseInt(cols[1]);
                this.loop = this.id != 0 ? GaemLoop.Receive : GaemLoop.Attack;
                break;
            case GaemLoop.Attack:
                let x = this.attack();
                let m = `Hit;${x[0]};${x[1]}`;
                send(m);
                break;
            case GaemLoop.Receive:
                break;
            case GaemLoop.End | GaemLoop.Win:
                break;
            case GaemLoop.End | GaemLoop.Lose:
                break;
            default:
                break;
        }
    }
    rand_int() {
        return Math.round(Math.random() * 10);
    }
    attack() {
        return [this.rand_int(), this.rand_int()];
    }
}
let gaem = new Gaem();
socket.onopen = function () {
    socket.send("Ok");
};
socket.onmessage = function (e) {
    gaem.tick(e.data);
};
function send(data) {
    console.log("sending:", data);
    socket.send(data);
}
export { Gaem, gaem, send };
