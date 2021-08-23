#!/usr/bin/python3

import sys
import signal
import cv2
import numpy as np
from queue import Queue, Empty
import pyscreenshot as ImageGrab
import datetime

end = False

# --------------------------------------cc
def signal_handler(signal, frame):
    global end
    end = True

# --------------------------------------
cap = cv2.VideoCapture(0)

arquivo = './Videos/' + sys.argv[1]
arquivo2 = 'tempo.txt'

# Tenta abrir a webcam, e ja falha se nao conseguir

if not cap.isOpened():
    print("Nao foi possivel abrir a web cam.")
    sys.exit(-1)

frame_width = int(cap.get(3))
frame_height = int(cap.get(4))
fps = cap.get(cv2.CAP_PROP_FPS)
fourcc2 = int(cap.get(cv2.CAP_PROP_FOURCC))


# Cria o arquivo de video de saída
fourcc = cv2.VideoWriter_fourcc(*'XVID')
out = cv2.VideoWriter(arquivo, fourcc2, 11, (frame_width,frame_height))



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
        print('Oops! A captura falhou.')
        break

# Encerra tudo
cap.release()
out.release()