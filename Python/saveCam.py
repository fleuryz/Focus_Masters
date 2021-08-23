#!/usr/bin/python3

import sys
import signal
import cv2
import numpy as np
from queue import Queue, Empty
import pyscreenshot as ImageGrab
from threading  import Thread


def readStdin(stdin, q):
    while (True):
        command = stdin.read(1)
        q.put(command)

# --------------------------------------cc
def signal_handler(signal, frame):
    global end
    end = True

# --------------------------------------
cap = cv2.VideoCapture(0)

arquivo =  sys.argv[1]

# Tenta abrir a webcam, e ja falha se nao conseguir

if not cap.isOpened():
    sys.stdout.write("PYTHON:Nao foi possivel abrir a web cam.")
    sys.exit(-1)

frame_width = int(cap.get(3))
frame_height = int(cap.get(4))
fps = cap.get(cv2.CAP_PROP_FPS)
fourcc2 = int(cap.get(cv2.CAP_PROP_FOURCC))

end = False

q = Queue()
t = Thread(target=readStdin, args=(sys.stdin, q))
t.daemon = True # thread dies with the program
t.start()

# Cria o arquivo de video de saída
fourcc = cv2.VideoWriter_fourcc(*'mp4v')
out = cv2.VideoWriter(arquivo, fourcc, 30, (frame_width,frame_height))

# Captura o sinal de CTRL+C no terminal
signal.signal(signal.SIGINT, signal_handler)

#print('Capturando o vídeo da webcam -- pressione Ctrl+C para encerrar...')

# Processa enquanto o usuário não encerrar (com CTRL+C)
contador = 0
lastTime = 0
while(not end):
    ret, frame = cap.read()

    if ret:
        out.write(frame)
    else:
        sys.stdout.write('PYTHON: Oops! A captura falhou.')
        break

    try:  command = q.get_nowait() # or q.get(timeout=.1)
    except Empty:
        end = False
    else: # got line
        print(command)
        if(command == 'q'):
            end = True

# Encerra tudo
sys.stdout.write('PYTHON: Encerrando.')
cap.release()
out.release()
sys.exit(0)