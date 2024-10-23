This is a simple websocket project that allows you to send and receive messages.

Enter the following command to run the project:<br>

```cargo run```

The server will start on port 8080.

```wscat -c ws://127.0.0.1:8080/ws```<br>

This will connect to the server and allow you to send and receive messages. Before that, remember to install wscat using the following command:<br>

```npm install -g wscat```

After that, you can send and receive messages by typing them in the terminal.

Enter the foollowing command after running the websocket:<br>

```curl -X POST http://127.0.0.1:8080/send -H "Content-Type: application/json" -d '{"body": {"message": "Hello, world!","status": 200}}'```