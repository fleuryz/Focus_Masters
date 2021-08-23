extern crate time;

use crate::variavel::Variavel;
use crate::sessao::Sessao;

use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct Data {
    pub time: time::Tm,
    pub name: String,
    pub value: Variavel,
}

impl Ord for Data {
    fn cmp(&self, other: &Data) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Data) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Data) -> bool {
        self.time == other.time
    }
}

impl Data {
    pub fn new(time: time::Tm, name: String, value: Variavel) -> Data {
        Data{
            time,
            name,
            value,
        }
    }

    pub fn get_dados(linha: &str) -> Data{
        let values:Vec<_> = linha.split('-').collect();
        let variable_values = match values[2].parse(){
            Ok(valor) => valor,
            Err(error) => panic!(error),
        };
        Data::new(Sessao::to_tm(values[0]), String::from(values[1]), variable_values)
        
    }

    pub fn escrever(&self, arquivo:&mut File){
        Sessao::escrever_data(arquivo, self.time);
        arquivo.write(b"-").unwrap();
        arquivo.write(self.name.as_bytes()).unwrap();
        arquivo.write(b"-").unwrap();
        self.value.escrever(arquivo);
        arquivo.write(b"\n").unwrap();
    }

    pub fn copiar(&self) -> Data {
        Data{
            time: self.time,
            name: format!("{}",self.name),
            value: match self.value {
                Variavel::Int(ref value) => Variavel::Int(*value),
                Variavel::Float(ref value) => Variavel::Float(*value),
                Variavel::Booleano(ref value) => Variavel::Booleano(*value),
                Variavel::Texto(ref value) => Variavel::Texto(format!("{}",value)),
            },
        }
    }

}

#[derive(Debug)]
pub enum TipoDados {
    DadosProcessados (Vec<Data>),
    DadosBrutos (Vec<Data>),
}

impl TipoDados{
    pub fn escrever(&self, arquivo:&mut File){
        match self{
            &TipoDados::DadosProcessados(ref dados) => {
                arquivo.write(b"Processado\n").unwrap();
                for dado in dados.iter(){
                    dado.escrever(arquivo);
                }
            },

            &TipoDados::DadosBrutos(ref dados) => {
                arquivo.write(b"Bruto\n").unwrap();
                for dado in dados.iter(){
                    dado.escrever(arquivo);
                }
            },
        }
    }

	pub fn nomes_variaveis(&self)-> Vec<String>{
		let mut nomes:Vec<String> = Vec::new();
		match self{
            &TipoDados::DadosProcessados(ref dados) => {
                for dado in dados.iter(){
					if !nomes.contains(&dado.name) {
						nomes.push(dado.name.clone());					
					}                    
                }
            },

            &TipoDados::DadosBrutos(ref dados) => {
                for dado in dados.iter(){
					if !nomes.contains(&dado.name) {
						nomes.push(dado.name.clone());					
					}                    
                }
            },
        }
		nomes
	}

}