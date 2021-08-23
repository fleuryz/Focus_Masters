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
from pymongo import MongoClient
from bson.son import SON

from faces import FaceDetector
from data import FaceData
from gabor import GaborBank
from emotions import EmotionsDetector

from collections import deque

POINTS = 200
OFFSETPOINT = 478
RESOL_LINE_Y = 68
OFFSETLINE = 840
RESOL_LINE_X = 5
SIZE_LINE = 2

START_X = 640
END_X = 1920
START_Y = 0
END_Y = 478

DIVIDER_X = 820
SIZE_CHAR = 68
OFFSETCHAR_FRAME = 412
OFFSETCHAR_Y = 30
OFFSETCHAR_X = 650

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
    def imprimir_tempo(self, time, frame, lab, val, fps):
    
        [year, mounth, day, hour, minute, second, nano] = time.split('/')
        
        nano = int(nano)
        segundo = int(segundo)
        minuto = int(minuto)
        hora = int(hora)
        dia = int(dia)
        mes = int(mes)
        ano = int(ano)
    
        tempoPassado = frame*int(1000000000/fps)
        
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
        current = -1
        black = (0,0,0)
        yellow = (0,255,255)
        soft = SIZE_LINE
        font = cv2.FONT_HERSHEY_SIMPLEX

        cv2.line(frame, (START_X,END_Y), (END_X, END_Y), black, soft)
        y = START_Y
        for l in labels:
            current += 1
            lab = '{}:'.format(l)

            x = OFFSETCHAR_X
            y = OFFSETCHAR_FRAME - current*SIZE_CHAR
            #size, _ = cv2.getTextSize(lab, font, 1, soft)
            #maior label tem tamanho de (164,22)
            #print (size)
            cv2.putText(frame, lab, (x, y+OFFSETCHAR_Y), font, 1, yellow, soft)
            cv2.line(frame, (START_X,y), (END_X, y), black, soft)

        cv2.line(frame, (DIVIDER_X, y), (DIVIDER_X,END_Y), black, soft)
        cv2.line(frame, (END_X, y), (END_X,END_Y), black, soft)
        #cv2.line(frame, (600, y), (600,465), cor, soft)

        return frame


    #-----------------------------------------
    def draw(self, frame, time, frameNum, vals, fps):
        """
        Draws the detected data of the given frame image.

        Parameters
        ----------
        frame: numpy.ndarray
            Image where to draw the information to.
        """

        yellow = (0, 255, 255)

        empty = True

        try:
            face = self._face
            empty = face.isEmpty()
        except:
            pass

        # Plot the emotion probabilities
        try:
            emotions = self._emotions
            current = 0
            labels = ['Neutral', 'Happiness', 'Sadness', 'Anger', 'Fear', 'Surprise', 'Disgust']

            frame = self.drawFrame(frame, labels)
            if frameNum % 30 == 0:
                if empty:
                    values = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
                else:
                    values = list(emotions.values())
                    bigger = labels[values.index(max(values))]

                for l, v in zip(labels, values):
                    lab = '{}'.format(l)
                    val = '{:.2f}'.format(v)
                    vals[current].rotate(-1)
                    vals[current].pop()
                    vals[current].append(v)
                    for i in range(POINTS-1):
                        value1 = int(OFFSETPOINT - vals[current][i]*RESOL_LINE_Y - current*RESOL_LINE_Y)
                        value2 = int(OFFSETPOINT - vals[current][i+1]*RESOL_LINE_Y - current*RESOL_LINE_Y)
                        cv2.line(frame, (OFFSETLINE+RESOL_LINE_X*i, value1), (OFFSETLINE+RESOL_LINE_X*(i+1), value2), yellow, SIZE_LINE)
                    #cv2.putText(frame, val, (5, 20 + atual*25), font, 1, yellow, 1)
                    #cv2.putText(frame, '{}'.format(vals[atual][199]), (320, 20 + atual*25), font, 1, yellow, 1)
                    current += 1
            else:
                for l in labels:
                    lab = '{}'.format(l)
                    for i in range(POINTS-1):
                        value1 = int(OFFSETPOINT - vals[current][i]*RESOL_LINE_Y - current*RESOL_LINE_Y)
                        value2 = int(OFFSETPOINT - vals[current][i+1]*RESOL_LINE_Y - current*RESOL_LINE_Y)
                        cv2.line(frame, (OFFSETLINE+RESOL_LINE_X*i, value1), (OFFSETLINE+RESOL_LINE_X*(i+1), value2), yellow, SIZE_LINE)
                    current += 1
            
            return frame, vals
        except Exception as e:
            print(e)
            pass
            
#---------------------------------------------
def check_time(time, clip_times, clip_time_miliseconds, half_clip_time):
    add = True
    for clip in clip_times:
        if clip[0] < time and time < clip[1]:
            add = False
            if clip[1] - time < 0.1*clip_time_miliseconds:
                clip[1] += 0.2*clip_time_miliseconds
            break
    if add:
        clip_times.append([time - half_clip_time, time + half_clip_time])

def main(argv):

    """
    Main entry of this script.

    Parameters
    ------
    argv: list of str
        Arguments received from the command line.
    """
    
    username = argv[1]
    session_number = sys.argv[2]
    current_time = float(argv[3])
    time_difference = float(argv[4])
    id = argv[5]

    clip_time_miliseconds = 60000
    half_clip_time = clip_time_miliseconds/2
    client = MongoClient(port=27017)
    db = client.Focus
    timeline = db.Timeline
    search_result = timeline.find_one({'_id': id})
    #hr = search_result['hr']
    #current_hr = 0
    #size_hr = len(hr)
    #eda = search_result['eda']
    #current_eda = 0
    #size_eda = len(eda)
    #keys = search_result['keys']
    #filtered_keys = filter(lambda item: item['value']['Texto'] == 'R', keys)
    #frames = search_result['frames']
    #filtered_frames = filter(lambda frame: len(filter(lambda event: event['type'] == 'CHAMPION_KILL' and ((event['killerId'] == frame['player_frame']['participantId']) or (event['victimId'] == frame['player_frame']['participantId'])), frame['events'])),frames)

    #clip_times = []
    #for key in filtered_keys:
    #    time = key['time']
    #    check_time(time, clip_times, clip_time_miliseconds, half_clip_time)

    #for frame in filtered_frames:
    #    time = frame['timestamp'] + current_time
    #    check_time(time, clip_times, clip_time_miliseconds, half_clip_time)

    #print(clip_times)

    base_name = ".\\Data\\LoL\\Sessions\\" + username + "\\" + session_number + "\\"

    face_video_name = base_name + "face.mp4"
    screen_video_name = base_name + "screen.mp4"

    face_video = cv2.VideoCapture(face_video_name)
    screen_video = cv2.VideoCapture(screen_video_name)
    if not face_video.isOpened():
        print('Error opening video file {}'.format(face_video_name))
        sys.exit(-1)
    if not screen_video.isOpened():
        print('Error opening video file {}'.format(screen_video_name))
        sys.exit(-1)

    fourcc = cv2.VideoWriter_fourcc(*'mp4v')
    fps_face = int(face_video.get(cv2.CAP_PROP_FPS))
    frameCount_face = int(face_video.get(cv2.CAP_PROP_FRAME_COUNT))

    fourcc2 = int(screen_video.get(cv2.CAP_PROP_FOURCC))
    fps_screen = int(screen_video.get(cv2.CAP_PROP_FPS))
    frameCount_screen = int(screen_video.get(cv2.CAP_PROP_FRAME_COUNT))

    frame_step = 1000/fps_screen

    frame_face_step = fps_face/fps_screen
    #print( str(fps_face) + '/' + str(fps_screen) + '=' + str(frame_face_step))
    
    #out = cv2.VideoWriter(nome_saida,fourcc, 20.0, (640,480))
    out_name = base_name + "export_video.mp4"
    out = cv2.VideoWriter(out_name,fourcc, fps_screen, (1920,1080))

    #clips = []
    #for i in range(len(clip_times)):
    #    write_name = i + ".mp4"
    #    clips.append(cv2.VideoWriter(write_name, fourcc, fps_screen, (1920,1080)))

    # Create the helper class
    data = VideoData()

    frameNum = 0
    frameFaceNum = -1
    frameFaceStepCount = 0.0
    frame_face = []
    valsTemp = []
    vals = []
    for i in range(7):
        for j in range(200):
            valsTemp.append(0)
        vals.append(deque(valsTemp))
        valsTemp = []

    #while ((frameCount_screen - frameCount_face) - 11) > int(screen_video.get(cv2.CAP_PROP_POS_FRAMES)):
        #screen_video.read()

    # Process the video input
    while True:
        ret2, frame_screen = screen_video.read()
        frameNum += 1
        current_time += frame_step
        
        if not ret2:
            break

        if current_time > time_difference + 2400:
            frameFaceStepCount += frame_face_step
            #print(str(frameFaceNum) + ' - ' + str(frameFaceStepCount))
            if frameFaceNum < int(frameFaceStepCount):
                ret, frame_face = face_video.read()
                frameFaceNum += 1
                if not ret:
                    break
            
            if frameFaceNum%fps_face == 0:
                data.detect(frame_face)
            output, vals = data.draw(frame_screen, time, frameNum, vals, fps_face)
            output[0:frame_face.shape[0], 0:frame_face.shape[1]] = frame_face
            out.write(output)

            #for i in range(len(clip_times)):
            #    if clip_times[i][0] < current_time and current_time < clip_times[i][1]:
            #        clips[i].write(output)
            #fazer a checagem dos clipes aqui
        else:
            out.write(frame_screen)

        
        
        
    face_video.release()
    screen_video.release()
    out.release()
    #for clip in clips:
    #    clip.release()
    cv2.destroyAllWindows()

        

#---------------------------------------------
# namespace verification for invoking main
#---------------------------------------------
if __name__ == '__main__':
    main(sys.argv)
 
