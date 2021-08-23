#!/usr/bin/python3
#

import sys
import argparse
import cv2
import numpy as np
import math

def normalizar(tipo, valor):
    tipos = []
    maximos = []

    for x in range(len(tipo)):
        if not (tipo[x] in tipos):
            tipos.append(tipo[x])
            maximos.append(0)
        if maximos[tipos.index(tipo[x])] < valor[x]:
            maximos[tipos.index(tipo[x])] = valor[x]
    
    for x in range(len(tipo)):
        if maximos[tipos.index(tipo[x])] > 0:
            valor[x] = valor[x]/maximos[tipos.index(tipo[x])]
    
    return valor


def ler_saidas(linha):
    [data, label, valor_txt] = linha.split('-')
    [ano, mes, dia, hora, minuto, segundo, nano] = data.split('/')
        
    nano = int(nano)
    segundo = int(segundo)
    minuto = int(minuto)
    hora = int(hora)
    dia = int(dia)
    mes = int(mes)
    ano = int(ano)
    valor_txt = valor_txt.strip("\n")
    if valor_txt == "true":
        valor = 1.0
    elif valor_txt == "false":
        valor = 0.0
    else:
        valor = float(valor_txt)
    return ((nano,segundo,minuto,hora,dia,mes,ano), label, valor)
            
#---------------------------------------------
def main(argv):

    """
    Main entry of this script.

    Parameters
    ------
    argv: list of str
        Arguments received from the command line.
    """

    nome_arquivo = './Testes/{}.kans'.format(sys.argv[1])
    numero_sessao = sys.argv[2]
    saidas = int(sys.argv[3])

    labels = ['Neutral', 'Felicidade', 'Tristeza', 'Raiva', 'Medo', 'Surpresa', 'Desgosto']
    labels_saida = []
    nomes_saidas = []

    for x in range(saidas):
        label = sys.argv[4+x]
        if not (label in labels):
            nomes_saidas.append(sys.argv[4+x])
        else:
            labels_saida.append(sys.argv[4+x])

    pontos = 0
    tempos = []
    tipos = []
    valores = []
    saida = []

    num_emoc = len(labels_saida)

    arquivo = open(nome_arquivo,"r")
    linha = arquivo.readline()

    while True:
        while ".Sessao:" not in linha:
            linha = arquivo.readline()
        linha = arquivo.readline()
        if linha.strip("\n") == numero_sessao:
            break


    while "Processado" not in linha:
        linha = arquivo.readline()
    
    linha = arquivo.readline().strip("\n")

    while "." != linha and ".Sessao:" != linha:
        (tempo, tipo, valor) = ler_saidas(linha)
        if tipo in labels_saida:
            pontos += 1
        elif not (tipo in nomes_saidas):
            linha = arquivo.readline().strip("\n")
            continue

        tempos.append(tempo)
        tipos.append(tipo)
        valores.append(valor)

        linha = arquivo.readline().strip("\n")
    arquivo.close()

    valores = normalizar(tipos, valores)
    

    for i in range(len(nomes_saidas)):
        labels_saida.append(nomes_saidas[i])
    
     #Create a black image
    img = np.zeros((1080,1920,3), np.uint8)
    font = cv2.FONT_HERSHEY_SIMPLEX
    branco = (255,255,255)
    amarelo = (0,255,255)
    tamanho = 19

    num_saidas_final = len(labels_saida)
    tam_linha = int(950/num_saidas_final)
    multiplicador = int(8/num_saidas_final)


    #Desenhar frame
    cv2.putText(img, '{} sessao {}'.format(sys.argv[1], numero_sessao), (10,30), font, 1, branco, 2)
    for i in range(num_saidas_final):
        cv2.putText(img,labels_saida[i], (10, 40 + int(tam_linha/2) + i*tam_linha), font, 1, branco, 3)
        #cv2.putText(img,'0',(185,160 + tam_linha + i*tam_linha), font, 0.5, branco)
        cv2.putText(img,'0',(185,40 + tam_linha + i*tam_linha), font, 0.5, branco)
        #cv2.putText(img,'0.5',(170,108 + int(tam_linha/2) + i*tam_linha), font, 0.5, branco)
        cv2.putText(img,'0.5',(170,40 + int(tam_linha/2) + i*tam_linha), font, 0.5, branco)
        cv2.putText(img,'1',(185,65 + i*tam_linha), font, 0.5, branco)
        #Linha vertical:
        cv2.line(img,(200,40 + i*tam_linha),(200,40 + tam_linha + i*tam_linha), branco, 2)
        #Linha Horizontal:
        cv2.line(img, (200, 40 + tam_linha + i*tam_linha), (1910, 40 + tam_linha + i*tam_linha), branco, 3)
    cv2.putText(img,'minuto:segundo:decimo de segundo',(950,1070), font, 0.5, branco)
    
    # #Desenhar divisao no eixo X
    # for i in range(tamanho):
    #     indice = int(7*i*((len(saida)/7)/tamanho))
    #     ((nano,segundo,minuto,hora,dia,mes,ano), label, valor) = saida[indice]
    #     cv2.putText(img,'{}:{}:{}'.format(minuto, segundo, int(nano/10000000)), (165+ i*int(1720/tamanho), 1030), font, 0.5, branco, 1)
    #     for j in range(7):
    #         cv2.line(img,(200 + i*int(1720/tamanho), 40 + j*140), (200+ i*int(1720/tamanho), 170 + j*140), branco, 1)
    
    #Desenhar valores
    anterior = -1
    contagem = 0
    cont_variaveis = 0
    contador = -1
    
    
    for i in range(len(valores) - num_saidas_final):
        if num_emoc>0:
            x = math.floor(contador/num_emoc)
        if tipos[0] in nomes_saidas:
            emocao = num_emoc + nomes_saidas.index(tipos[0])
        else:
            contador += 1
            emocao = contador%num_emoc
        tempo = tempos[0]
        tipo = tipos[0]
        valor1 = valores[0]
        tempos.remove(tempo)
        tipos.remove(tipo)
        valores.remove(valor1)
        if not labels_saida[emocao] == tipo:
            texto = labels_saida[emocao] + "==" + tipo
            print(texto)

        if tipo in tipos:
            valor2 = valores[tipos.index(tipo)]
        else:
            valor2 = valor1

        #(tempo2, lixo2, valor12) = saida[0]
        #(lixo1, lixo2, valor22) = saida[i+7]



        ponto1 = (200 + int(x*1710/(pontos/num_emoc)), 40 + tam_linha + emocao*tam_linha - int(valor1*(tam_linha - 5)))
        ponto2 = (200 + int((x+1)*1710/(pontos/num_emoc)), 40 + tam_linha + emocao*tam_linha - int(valor2*(tam_linha - 5)))
        cv2.line(img, ponto1, ponto2, amarelo, 2)
        if contagem <= tamanho and contagem*math.floor(pontos/tamanho) == contador or contador == (pontos-num_emoc+1):
            contagem +=1
            (nano,segundo,minuto,hora,dia,mes,ano) = tempo
            cv2.putText(img,'{}:{}:{}'.format(minuto, segundo, int(nano/10000000)), (150 + int(x*1710/(pontos/num_emoc)), 1015), font, 0.5, branco, 1)
            for j in range(num_saidas_final):
                cv2.line(img,(200 + int(x*1710/(pontos/num_emoc)), 40 + j*tam_linha), (200 + int(x*1710/(pontos/num_emoc)), 40 + tam_linha + j*tam_linha), branco, 1)

    nome_saida = './Saida/Graficos/{}_{}.png'.format(sys.argv[1], numero_sessao)

    cv2.imwrite(nome_saida, img)
    cv2.destroyAllWindows()

#---------------------------------------------
# namespace verification for invoking main
#---------------------------------------------
if __name__ == '__main__':
    main(sys.argv)
 
