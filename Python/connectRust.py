import zmq

context = zmq.Context()

socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:5555")

string_to_rev = input("enter string ")
print(f"sending {string_to_rev}")

socket.send(string_to_rev.encode())
message = socket.recv()
print(f"reversed: {message}")