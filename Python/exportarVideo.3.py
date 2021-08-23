import numpy as np
import cv2

nome_video_jogo = './output.mkv'
video_jogo = cv2.VideoCapture(nome_video_jogo)
fourcc2 = int(video_jogo.get(cv2.CAP_PROP_FOURCC))
width = int(video_jogo.get(cv2.CAP_PROP_FRAME_WIDTH))
height = int(video_jogo.get(cv2.CAP_PROP_FRAME_HEIGHT))
fps = int(video_jogo.get(cv2.CAP_PROP_FPS))
framecount = int(video_jogo.get(cv2.CAP_PROP_FRAME_COUNT))

video_saida = cv2.VideoWriter('teste1.avi',fourcc2, fps, (width,height))
tamanho = 0

while(video_jogo.isOpened()):
    ret, frame = video_jogo.read()

    if ret:
        tamanho += 1
        frame_jogo = frame.copy()
        video_saida.write(frame_jogo)

    else:
        break

print(tamanho)
video_saida.release()
video_jogo.release()
cv2.destroyAllWindows()