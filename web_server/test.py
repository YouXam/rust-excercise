import socket
import threading
import time


def job(path, duration, tid):
    begin = time.time()
    print(f"[{tid}] start to connect {path}")
    s = socket.socket()
    s.connect(("127.0.0.1", 7878))
    s.send(b'GET ' + path.encode())
    time.sleep(duration)
    s.send(b' HTTP/1.1\r\n\r\n')
    print(f"[{tid}] recv", s.recv(1024).decode().split('\r\n')[0], f" cost {round(time.time() - begin, 2)}s")


begin = time.time()
t = []
duration = 0
for i in range(10):
    t.append(threading.Thread(target=job, args=('/sleep', duration, i)))
    t[-1].start()

for i in t:
    i.join()

print(f"end with {round(time.time() - begin, 2)}s")
