#!/usr/bin/python3
#
# This file is part of the Emotions project. The complete source code is
# available at https://github.com/luigivieira/emotions.
#
# Copyright (c) 2016-2017, Luiz Carlos Vieira (http://www.luiz.vieira.nom.br)
#
# MIT License
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

import time
import sys
import argparse
import cv2
import numpy as np
from collections import OrderedDict
from datetime import datetime, timedelta

from faces import FaceDetector
from data import FaceData
from gabor import GaborBank
from emotions import EmotionsDetector

from collections import deque

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
    def detect(self, frame):
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

        ret, face = self._faceDet.detect(frame)
        if ret:
            self._face = face

            # Crop just the face region
            frame, face = face.crop(frame)

            # Filter it with the Gabor bank
            responses = self._bank.filter(frame)

            # Detect the prototypic emotions based on the filter responses
            self._emotions = self._emotionsDet.detect(face, responses)

            return True
        else:
            self._face = None
            return False

    #---------------------------------------------
    def imprimir_tempo(self, tempo, frame, lab, val):
    
        [ano, mes, dia, hora, minuto, segundo, nano] = tempo.split('/')
        
        nano = int(nano)
        segundo = int(segundo)
        minuto = int(minuto)
        hora = int(hora)
        dia = int(dia)
        mes = int(mes)
        ano = int(ano)
    
        tempoPassado = frame*50000000
        
        nano += tempoPassado%1000000000

        tempoPassado = int(tempoPassado/1000000000)
        if nano >= 1000000000:
            nano -= 1000000000
            tempoPassado += 1
            
        segundo += tempoPassado%60
        tempoPassado = int(tempoPassado/60)
        if segundo >= 60:
            segundo -= 60
            tempoPassado += 1
        
        minuto += tempoPassado%60
        tempoPassado = int(tempoPassado/60)
        if minuto >= 60:
            minuto -= 60
            tempoPassado += 1
        
        hora += tempoPassado%60
        tempoPassado = int(tempoPassado/24)
        if hora >= 24:
            hora -= 24
            tempoPassado += 1
        
        dia += tempoPassado%24
        if hora >= 24:
            hora -= 24
        
        saida = str(ano) + '/' + str(mes) + '/' + str(dia) + '/' + str(hora) + '/' + str(minuto) + '/' + str(segundo) + '/' + str(nano) + '-' + lab + '-' + val
        
        print(saida)
    #-----------------------------------------

    def drawFrame(self, frame, labels):
        atual = -1
        cor = (0,0,0)
        amarelo = (0,255,255)
        soft = 1
        font = cv2.FONT_HERSHEY_SIMPLEX
        cv2.line(frame, (0,465), (620, 465), cor, soft)
        y = 0
        for l in labels:
            atual += 1
            lab = '{}:'.format(l)

            x = 10
            y = 435 - atual*30
            #size, _ = cv2.getTextSize(lab, font, 1, soft)
            #maior label tem tamanho de (164,22)
            #print (size)
            cv2.putText(frame, lab, (x, y+26), font, 1, amarelo, soft)
            cv2.line(frame, (0,y), (620, y), cor, soft)

        cv2.line(frame, (180, y), (180,465), cor, soft)
        cv2.line(frame, (620, y), (620,465), cor, soft)
        #cv2.line(frame, (600, y), (600,465), cor, soft)

        return frame


    #-----------------------------------------
    def draw(self, frame, tempo, frameNum, vals):
        """
        Draws the detected data of the given frame image.

        Parameters
        ----------
        frame: numpy.ndarray
            Image where to draw the information to.
        """

        # Font settings
        font = cv2.FONT_HERSHEY_SIMPLEX
        scale = 0.5
        thick = 1
        glow = 3 * thick

        # Color settings
        black = (0, 0, 0)
        yellow = (0, 255, 255)

        empty = True

        print("teste")
        try:
            face = self._face
            empty = face.isEmpty()
        except:
            pass

        # Plot the emotion probabilities
        try:
            emotions = self._emotions
            atual = 0
            if empty:
                labels = []
                values = []
            else:
                
                labels = list(emotions.keys())
                values = list(emotions.values())
                bigger = labels[values.index(max(values))]

            frame = self.drawFrame(frame, labels)
            for l, v in zip(labels, values):
                lab = '{}'.format(l)
                val = '{:.2f}'.format(v)
                vals[atual].rotate(-1)
                vals[atual].pop()
                vals[atual].append(v)
                for i in range(199):
                    valor1 = int(465 - vals[atual][i]*30 - atual*30)
                    valor2 = int(465 - vals[atual][i+1]*30 - atual*30)
                    cv2.line(frame, (200+2*i, valor1), (200+2*(i+1), valor2 ), (0,255,255), 1)
                #cv2.putText(frame, val, (5, 20 + atual*25), font, 1, yellow, 1)
                #cv2.putText(frame, '{}'.format(vals[atual][199]), (320, 20 + atual*25), font, 1, yellow, 1)
                self.imprimir_tempo(tempo, frameNum, lab, val)
                atual += 1
            
            return frame, vals
        except Exception as e:
            print(e)
            pass
            
#---------------------------------------------
def main(argv):

    """
    Main entry of this script.

    Parameters
    ------
    argv: list of str
        Arguments received from the command line.
    """
    
    nome_arquivo = './Videos/' + sys.argv[1]
    nome_saida = './Saida/Videos/saida_{}'.format(sys.argv[1])

    # Define the codec and create VideoWriter object
    fourcc = cv2.VideoWriter_fourcc(*'XVID')
    out = cv2.VideoWriter(nome_saida,fourcc, 11.0, (640,480))

    video = cv2.VideoCapture(nome_arquivo)
    if not video.isOpened():
        print('Error opening video file {}'.format(nome_arquivo))
        sys.exit(-1)

    fps = int(video.get(cv2.CAP_PROP_FPS))
    frameCount = int(video.get(cv2.CAP_PROP_FRAME_COUNT))
    sourceName = argv[0]
    tempo = argv[2]

    # Create the helper class
    data = VideoData()

    paused = False
    frameNum = 0
    valsTemp = []
    vals = []
    for i in range(7):
        for j in range(200):
            valsTemp.append(0)
        vals.append(deque(valsTemp))
        valsTemp = []

    # Process the video input
    while True:

        ret, img = video.read()

        if ret:
            frame = img.copy()
            frameNum += 1
        else:
            break

        data.detect(frame)
        saida, vals = data.draw(frame, tempo, frameNum, vals)
        out.write(saida)
        
        

    video.release()
    out.release()
    cv2.destroyAllWindows()

#---------------------------------------------
# namespace verification for invoking main
#---------------------------------------------
if __name__ == '__main__':
    main(sys.argv)
 
