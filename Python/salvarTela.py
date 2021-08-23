#!/usr/bin/python3

import sys
import signal
import cv2
import numpy as np
from queue import Queue, Empty
import pyscreenshot as ImageGrab
import time

end = False

# --------------------------------------cc
def signal_handler(signal, frame):
    global end
    end = True

# --------------------------------------

arquivo2 = './Videos/tela_' + sys.argv[1] 

fps = 1.0
tempo = float(1/fps)
frame_size = (1920,1200)
video = []

fourcc2 = cv2.VideoWriter_fourcc(*'DIVX')

out2 = cv2.VideoWriter(arquivo2, fourcc2, fps, frame_size)

# Captura o sinal de CTRL+C no terminal
signal.signal(signal.SIGINT, signal_handler)
#print('Capturando o vídeo da webcam -- pressione Ctrl+C para encerrar...')

# Processa enquanto o usuário não encerrar (com CTRL+C)
while(not end):
    video.append(ImageGrab.grab(bbox=(0,0,1920,1200)))


# Encerra tudo

for imagem in video:
    frame = np.array(imagem) 
    frame = frame[:, :, ::-1].copy() 

    out2.write(frame)


out2.release()
