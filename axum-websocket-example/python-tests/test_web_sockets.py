from websocket import create_connection

text_to_send = "Hello, websocket"
expected_response = f"You said: {text_to_send}"

ws = create_connection("ws://127.0.0.1:3000/integration-testable")
print(f"Sending '{text_to_send}'")
ws.send(text_to_send)
result =  ws.recv()
print("Received '%s'" % result)
assert result == expected_response
ws.close()
