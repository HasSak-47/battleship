"use strict";
let input = document.getElementById("input");
let output = document.getElementById("output");
let socket = new WebSocket("ws://localhost:8080/ws");
var GaemLoop;
(function (GaemLoop) {
    GaemLoop[GaemLoop["None"] = 0] = "None";
    GaemLoop[GaemLoop["Start"] = 0] = "Start";
    GaemLoop[GaemLoop["Hit"] = 1] = "Hit";
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
                this.loop = this.id == 0 ? GaemLoop.Receive : GaemLoop.Hit;
            case GaemLoop.Hit:
                let x = this.attack();
                socket.send(`Hit;${x[0]} ${x[1]}`);
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
    attack() {
        return [Math.random(), Math.random()];
    }
}
let gm = new Gaem();
socket.onopen = function () {
    socket.send("Ok");
};
socket.onmessage = function (e) {
    gm.tick(e.data);
};
function send() {
    socket.send(input.value);
}
