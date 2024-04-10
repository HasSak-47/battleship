let input = document.getElementById("input");
let output = document.getElementById("output");
let socket = new WebSocket("ws://localhost:8080/ws");

socket.onopen = function () { };

function send() {
	let data = input.value;
    console.log("sending:", data);
    socket.send(data);
}

socket.onmessage = function (event) {
	console.log("received:", event.data);
	if(output.innerHTML != '')
		output.innerHTML += '\n'

	output.innerHTML += event.data;

};

