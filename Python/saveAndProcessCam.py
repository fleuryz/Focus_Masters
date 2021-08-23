#!/usr/bin/python3

import sys
import signal
import cv2
import numpy as np
from queue import Queue, Empty
import pyscreenshot as ImageGrab
from threading  import Thread

import xmlrpc.client
import logging
import time
from collections import OrderedDict
from datetime import datetime, timedelta

from faces import FaceDetector
from data import FaceData
from gabor import GaborBank
from emotions import EmotionsDetector

#---------------------------------------------
class VideoData:
    """
    Helper class to present the detected face region, landmarks and emotions.
    """

    #-----------------------------------------
    def __init__(self):
        """
        Class constructor.
        """

        self._faceDet = FaceDetector()
        '''
        The instance of the face detector.
        '''

        self._bank = GaborBank()
        '''
        The instance of the bank of Gabor filters.
        '''

        self._emotionsDet = EmotionsDetector()
        '''
        The instance of the emotions detector.
        '''

        self._face = FaceData()
        '''
        Data of the last face detected.
        '''

        self._emotions = OrderedDict()
        '''
        Data of the last emotions detected.
        '''

    #-----------------------------------------
    def detect(self, frame, frameTime, proxy):
        """
        Detects a face and the prototypic emotions on the given frame image.

        Parameters
        ----------
        frame: numpy.ndarray
            Image where to perform the detections from.

        Returns
        -------
        ret: bool
            Indication of success or failure.
        """
        logging.info("Teste")
        ret, face = self._faceDet.detect(frame)
        empty = False
        if ret:
            self._face = face

            # Crop just the face region
            frame, face = face.crop(frame)

            # Filter it with the Gabor bank
            responses = self._bank.filter(frame)

            # Detect the prototypic emotions based on the filter responses
            self._emotions = self._emotionsDet.detect(face, responses)

            empty = empty = face.isEmpty()
        
            try:
                emotions = self._emotions
                if empty:
                    labels = []
                    values = []
                else:
                    
                    labels = list(emotions.keys())
                    values = list(emotions.values())
                    #bigger = labels[values.index(max(values))]

                for l, v in zip(labels, values):
                    lab = '{}'.format(l)
                    val = '{:.2f}'.format(v)
                    
                    saida = str(frameTime) + '-' + lab + '-' + val
                    proxy.send_data(saida)
                    #print(saida)

            except Exception as e:
                print(e)
                pass

                return True
        else:
            self._face = None
            return False
    
frameNum = 0
frame = None
data = None
threadRun = True
proxy = xmlrpc.client.ServerProxy("http://127.0.0.1:8080")

def threadFunc():
    last_frame = frameNum
    while threadRun:
        if frameNum%fps == 0 and frameNum != last_frame:
                #currentTime = time.time()
                currentTime = datetime.now().timestamp()#.strftime("%Y/%m/%d/%H/%M/%S/%f")
                data.detect(frame, int((currentTime)*1000), proxy) #data.detect(frame, int((currentTime - 10800)*1000))
                last_frame = frameNum

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
fps = 30
fourcc = cv2.VideoWriter_fourcc(*'mp4v')
out = cv2.VideoWriter(arquivo, fourcc, fps, (frame_width,frame_height))

time_between_frame = 1/fps

# Captura o sinal de CTRL+C no terminal
signal.signal(signal.SIGINT, signal_handler)

#print('Capturando o vídeo da webcam -- pressione Ctrl+C para encerrar...')

data = VideoData()
paused = False
frameNum = 0
last_frame = 0

ret, frame = cap.read()
currentTime = datetime.now().timestamp()#.strftime("%Y/%m/%d/%H/%M/%S/%f")
data.detect(frame, int((currentTime)*1000), proxy)
#data.detect(frame, 0)
out.write(frame)
frameNum += 1


# Processa enquanto o usuário não encerrar (com CTRL+C)
contador = 0
t = time.localtime()
current_time = datetime.now().timestamp() #.strftime("%Y/%m/%d/%H/%M/%S/%f")
saida = str(int((current_time)*1000)) + '-face-start\r\n' #-10800
next_frame_time = current_time + time_between_frame

faceInfering = Thread(target=threadFunc, args=())
faceInfering.start()

ten_seconds_frame = current_time + 10
expected = 300

while(not end):
    current_time = datetime.now().timestamp()
    while current_time < next_frame_time:
        timeout = next_frame_time - current_time
        time.sleep(timeout)
        current_time = datetime.now().timestamp()
    
    if current_time > next_frame_time + time_between_frame:
        frames_ahead = (int((current_time-next_frame_time)/time_between_frame))
        for i in range(frames_ahead):
            #print('Maior que deveria: ' + str(frameNum))
            #print('Delay: ' + str(delay))
            out.write(frame)
            frameNum += 1
            next_frame_time += time_between_frame

    ret, frame = cap.read()
    #ret = cap.grab()
    #if not ret:
        #print('Não pegou aqui não')
    #ret, frame = cap.retrieve()
    if ret:
        out.write(frame)
        frameNum += 1
        next_frame_time += time_between_frame
    else:
        sys.stdout.write('PYTHON: Oops! A captura falhou.\r\n')
        break
    
    if current_time > ten_seconds_frame:
            #print('Total: ' + str(frameNum) + '\nExpected: ' + str(expected) + '\nMiss: ' + str(100-(frameNum*100/expected)) + '% (' + str(expected-frameNum) + ')')
            ten_seconds_frame += 10
            expected += 300
    
    try: command = q.get_nowait() #q.get_nowait() # or q.get(timeout=.1)
    except Empty:
        end = False
    else: # got line
        if(command == 'q'):
            end = True

    #if frameNum%fps == 0:
            #currentTime = time.time()
            #currentTime = datetime.now().timestamp()#.strftime("%Y/%m/%d/%H/%M/%S/%f")
            #faceInfering = Thread(target=data.detect, args=(frame,int(currentTime*1000),))
            #faceInfering.start()
    #print(str(time_between_frame) + ' - ' + str(delay) + ' = ' + str(time_between_frame-delay))
    

# Encerra tudo
#sys.stdout.write('PYTHON: Encerrando.')
proxy.end_data(0)
threadRun = False
cap.release()
out.release()
sys.exit(0)