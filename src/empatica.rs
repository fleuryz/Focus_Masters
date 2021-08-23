use std::fs;
use std::error::Error;
use crate::db_structures::LoLData;
use crate::variavel::Variavel;

extern crate xml_rpc;
use std::sync::mpsc;

use std::process::{Command, Stdio};
use std::thread;
use xml_rpc::{Fault, Client, Url, into_params, Params, Value};
use serde::{Serialize, Deserialize};

#[derive(Clone,Debug)]
pub struct Signal_Data{
    pub start_timestamp: f64,
    pub frequency: u64,
    pub data_vec: Vec<f64>,
}

impl Signal_Data{
    pub fn new(file_location: &str) -> Result<Signal_Data, Box<dyn Error>>{
        let data = fs::read_to_string(file_location)?;

        let mut data = data.lines();

        let data_string = data.next().unwrap();

        let start_timestamp: f64 = data_string.parse()?;
        let data_string = data.next().unwrap();
        let frequency: u64 = data_string.parse::<f64>()? as u64;

        let mut data_vec: Vec<f64> = Vec::new();
        for line in data{
            if line.len() > 1{
                data_vec.push(line.parse()?);   
            }
        }

        Ok(Signal_Data{start_timestamp, frequency, data_vec})
    }

    pub fn new_three_axis(file_location: &str) -> Result<Vec<Signal_Data>, Box<dyn Error>>{
        let data = fs::read_to_string(file_location)?;


        let mut data = data.lines();

        let timestamps: Vec<&str> = data.next().unwrap().split(",").collect();
        let start_timestamp_x: f64 = timestamps[0].trim().parse()?;
        let start_timestamp_y: f64 = timestamps[1].trim().parse()?;
        let start_timestamp_z: f64 = timestamps[2].trim().parse()?;

        let frequencies: Vec<&str> = data.next().unwrap().split(",").collect();
        let frequency_x: u64 = frequencies[0].trim().parse::<f64>()? as u64;
        let frequency_y: u64 = frequencies[1].trim().parse::<f64>()? as u64;
        let frequency_z: u64 = frequencies[2].trim().parse::<f64>()? as u64;

        let mut data_vec_x: Vec<f64> = Vec::new();
        let mut data_vec_y: Vec<f64> = Vec::new();
        let mut data_vec_z: Vec<f64> = Vec::new();
        for line in data{
            if line.len() > 1{
                let data_vecs: Vec<&str> = line.split(",").collect(); 
                data_vec_x.push(data_vecs[0].parse()?);
                data_vec_y.push(data_vecs[1].parse()?);
                data_vec_z.push(data_vecs[2].parse()?); 
            }
        }

        let mut return_vec: Vec<Signal_Data> = Vec::new();
        return_vec.push(Signal_Data{start_timestamp: start_timestamp_x, frequency: frequency_x, data_vec: data_vec_x});
        return_vec.push(Signal_Data{start_timestamp: start_timestamp_y, frequency: frequency_y, data_vec: data_vec_y});
        return_vec.push(Signal_Data{start_timestamp: start_timestamp_z, frequency: frequency_z, data_vec: data_vec_z});
        Ok(return_vec)
    }

    pub fn get_lol_data(&self, name: &str, start_time: f64, end_time: f64) -> Result<Vec<LoLData>, Box<dyn Error>>{
        let mut lol_data_vec = Vec::new();

        let time_step: f64 = (1.0/(self.frequency as f64));
        let mut i = 0;
        
        for data in &self.data_vec{
            let time = (self.start_timestamp + time_step*(i as f64))*1000.0;
            i += 1;
            if time < start_time || time > end_time{
                continue;
            }
            lol_data_vec.push(LoLData::new_timestamp(time as u64, name.into(), Variavel::Float(*data)));
        }

        Ok(lol_data_vec)
    }

    pub fn get_eda_data(&self, start_time: f64, end_time: f64) -> Result<Vec<LoLData>,Box<dyn Error>>{
        let mut client = Client::new()?;
        let url = Url::parse("http://localhost:9000")?;

        let mut params_vec: Params = Vec::new();
        let params_1 = Value::Array(into_params(&self.data_vec)?);
        let params_2 = Value::Int(self.frequency as i32);
        let params_3 = Value::Double(self.start_timestamp);
        let params_4 = Value::Double(start_time);
        let params_5 = Value::Double(end_time);

        params_vec.push(params_1);
        params_vec.push(params_2);
        params_vec.push(params_3);
        params_vec.push(params_4);
        params_vec.push(params_5);

        let result = client.call_value(&url, "processEDA", params_vec)?.expect("Could not convert xml return");


        let mut lol_data_vec = Vec::new();

        let time_step: f64 = 1.0/(self.frequency as f64);
        let mut i = 0;
        
        for data in result{
            let time = (self.start_timestamp + time_step*(i as f64))*1000.0;
            i += 1;
            if time < start_time || time > end_time{
                continue;
            }
            let value;
            if let Value::Double(xml_value) = data{
                value = Variavel::Float(xml_value);
            }else{
                value = Variavel::Float(0.0);
            }
            lol_data_vec.push(LoLData::new_timestamp(time as u64, "eda".into(), value));
        }

        Ok(lol_data_vec)
    }

    pub fn get_hrv_data(&self, start_time: f64, end_time: f64) -> Result<(Vec<LoLData>, Vec<LoLData>), Box<dyn Error>>{
        let mut client = Client::new()?;
        let url = Url::parse("http://localhost:9000")?;

        let mut params_vec: Params = Vec::new();
        let params_1 = Value::Array(into_params(&self.data_vec)?);
        let params_2 = Value::Int(self.frequency as i32);
        let params_3 = Value::Double(self.start_timestamp + 10800.0);
        let params_4 = Value::Double(start_time);
        let params_5 = Value::Double(end_time);

        params_vec.push(params_1);
        //params_vec.push(params_2);
        params_vec.push(params_3);
        params_vec.push(params_4);
        params_vec.push(params_5);

        let result = client.call_value(&url, "processHR", params_vec)?.expect("Could not convert xml return");

        let mut hr_data_vec = Vec::new();
        let mut hrv_data_vec = Vec::new();

        let time_step: f64 = 1.0/(self.frequency as f64);
        let mut i = 0;

        let heart_rate;
        let heart_rate_timestamp;
        let heart_rate_variance;
        let heart_rate_variance_timestamp;
        if let Value::Array(array) = &result[0]{
            if let Value::Array(hr_array) = &array[0]{
                heart_rate = Some(hr_array);
            }else{
                heart_rate = None;
            }
            if let Value::Array(hr_timestamp_array) = &array[1]{
                heart_rate_timestamp = Some(hr_timestamp_array);
            }else{
                heart_rate_timestamp = None;
            }
            if let Value::Array(hrv_array) = &array[2]{
                heart_rate_variance = Some(hrv_array);
            }else{
                heart_rate_variance = None;
            }
            if let Value::Array(hrv_timestamp_array) = &array[3]{
                heart_rate_variance_timestamp = Some(hrv_timestamp_array);
            }else{
                heart_rate_variance_timestamp = None;
            }

        }else{
            heart_rate = None;
            heart_rate_timestamp = None;
            heart_rate_variance = None;
            heart_rate_variance_timestamp = None;
        }

        let heart_rate = heart_rate.unwrap();
        let heart_rate_timestamp = heart_rate_timestamp.unwrap();
        let heart_rate_variance = heart_rate_variance.unwrap();
        let heart_rate_variance_timestamp = heart_rate_variance_timestamp.unwrap();

        for i in 0..heart_rate.len()
        /*for data in heart_rate.unwrap()*/{
            let time;
            if let Value::Double(xml_time) = heart_rate_timestamp[i]{
                time = xml_time*1000.0;
            }else{
                continue;
            }
            
            //let time = (self.start_timestamp + time_step*(i as f64))*1000.0;
            //i += 1;
            if time < start_time || time > end_time{
                continue;
            }
            let value;
 
            if let Value::Double(xml_value) = heart_rate[i]{
                value = Variavel::Float(xml_value);
            }else{
                value = Variavel::Float(0.0);
            }
            hr_data_vec.push(LoLData::new_timestamp(time as u64, "hr".into(), value));
        }

        for i in 0..heart_rate_variance.len()
        /*for data in heart_rate_variance.unwrap()*/{
            let time;
            if let Value::Double(xml_time) = heart_rate_variance_timestamp[i]{
                time = (xml_time - 10800.0)*1000.0;
            }else{
                continue;
            }
            
            //let time = (self.start_timestamp + time_step*(i as f64))*1000.0;
            //i += 1;
            if time < start_time || time > end_time{
                continue;
            }
            let value;
 
            if let Value::Double(xml_value) = heart_rate_variance[i]{
                value = Variavel::Float(xml_value);
            }else{
                value = Variavel::Float(0.0);
            }
            hrv_data_vec.push(LoLData::new_timestamp(time as u64, "hrv".into(), value));
        }

        Ok((hr_data_vec, hrv_data_vec))
    }
}

#[derive(Clone,Debug)]
pub struct IBI_Signal{
    pub beat: f64,
    pub duration: f64,
}

#[derive(Clone,Debug)]
pub struct IBI_Data{
    pub start_timestamp: f64,
    pub data_vec: Vec<IBI_Signal>,
}
impl IBI_Data{
    pub fn new(file_location: &str) ->Result<IBI_Data, Box<Error>>{
        let data = fs::read_to_string(file_location)?;


        let mut data = data.lines();

        let start_timestamp: f64 = data.next().unwrap().split(',').collect::<Vec<&str>>()[0].parse()?;

        let mut data_vec: Vec<IBI_Signal> = Vec::new();
        for line in data{
            let ibi_signal_string: Vec<&str> = line.split(',').collect();
            data_vec.push(IBI_Signal{
                beat: ibi_signal_string[0].parse()?,
                duration: ibi_signal_string[1].parse()?,
            });
        }

        Ok(IBI_Data{start_timestamp, data_vec})
    }

    pub fn get_lol_data(&self, name: &str, start_time: f64, end_time: f64) -> Result<Vec<LoLData>, Box<dyn Error>>{
        let mut lol_data_vec = Vec::new();

        let mut i = 0;
        for data in &self.data_vec{
            if data.beat < start_time || data.beat > end_time{
                break;
            }
            lol_data_vec.push(LoLData::new_timestamp(data.beat as u64, name.into(), Variavel::Float(data.duration)));

            i += 1;
        }

        Ok(lol_data_vec)
    }
}

#[derive(Clone,Debug)]
pub struct E4_Data{
    pub bvp: Signal_Data,
    pub eda: Signal_Data,
    pub acc: Vec<Signal_Data>,
    pub ibi: IBI_Data,
    pub hr: Signal_Data,
    pub temp: Signal_Data,
    pub tags: Vec<f64>,

    pub sender: Option<std::sync::mpsc::Sender<bool>>,
}

impl E4_Data{
    pub fn new(directory: &str) -> Result<E4_Data, Box<dyn Error>>{
        let bvp = Signal_Data::new(&format!("{}/BVP.csv", directory))?;
        let eda = Signal_Data::new(&format!("{}/EDA.csv", directory))?;
        let acc = Signal_Data::new_three_axis(&format!("{}/ACC.csv", directory))?;
        let ibi = IBI_Data::new(&format!("{}/IBI.csv", directory))?;
        let hr = Signal_Data::new(&format!("{}/HR.csv", directory))?;
        let tags = E4_Data::get_tags(&format!("{}/tags.csv", directory))?;
        let temp = Signal_Data::new(&format!("{}/temp.csv", directory))?;

        let sender = None;
        Ok(E4_Data{bvp,eda,acc,ibi,hr,tags,temp, sender})
    }

    pub fn get_tags(file_location: &str)-> Result<Vec<f64>, Box<dyn Error>>{
        let data = fs::read_to_string(file_location)?;
        
        let data = data.lines();

        let mut tags: Vec<f64> = Vec::new();
        for line in data{
            tags.push((line.parse::<f64>()?)*1000.0);
        }

        Ok(tags)
    }

    pub fn get_data_vec(&self, tag_number: usize) -> Result<Vec<LoLData>, Box<dyn Error>>{
        let mut data_vec = Vec::new();
        
        data_vec.append(&mut self.bvp.get_lol_data("bvp", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.eda.get_lol_data("eda", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.acc[0].get_lol_data("acc_x", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.acc[1].get_lol_data("acc_y", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.acc[2].get_lol_data("acc_z", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.ibi.get_lol_data("ibi", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.hr.get_lol_data("hr", self.tags[tag_number], self.tags[tag_number+1])?);
        data_vec.append(&mut self.temp.get_lol_data("temp", self.tags[tag_number], self.tags[tag_number+1])?);

        //println!("Vetor: {:?}", data_vec);

        Ok(data_vec)
    }

    pub fn start_python_rpc(&mut self) -> Result<(), Box<dyn Error>>{
        let (s_rpc, r_rpc) = mpsc::channel();
        self.sender = Some(s_rpc);
        let python_rpc_thread = thread::Builder::new().name("RPC_Server_Thread".to_string()).spawn(move || {
            let mut start_server = Command::new("python")
                .arg("./Python/getBioSignals.py")
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
                .expect("Error starting python with RPC server")
            ;
            println!("Rodando");
            loop {
                if r_rpc.try_recv().is_ok() {

                    start_server.kill();
                    break;
                }
            }
            start_server.wait().expect("Error on starting RPC server");
        })?;

        Ok(())
    }

    pub fn end_python_rpc(&self) -> Result<(), Box<dyn Error>>{
        /*let mut client = Client::new()?;
        let url = Url::parse("http://localhost:9000")?;
        let result = client.call_value(&url, "endServer", Vec::new())?;*/
        
        if let Some(sender) = &self.sender{
            sender.send(true).expect("Error sending ender of RPC server.");
        }
        
        Ok(())
    }
}

pub fn check_has_files(directory: &str) -> bool {
    let mut has_bvp = false;
    let mut has_eda = false;
    let mut has_acc = false;
    let mut has_ibi = false;
    let mut has_hr = false;
    let mut has_tags = false;
    let mut has_temp = false;
    //let mut sessions: Vec<LoLSession> = Vec::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                match entry.file_name().to_str().unwrap(){
                    "BVP.csv" => has_bvp = true,
                    "EDA.csv" => has_eda = true,
                    "ACC.csv" => has_acc = true,
                    "IBI.csv" => has_ibi = true,
                    "HR.csv" => has_hr = true,
                    "tags.csv" => has_tags = true,
                    "TEMP.csv" => has_temp = true,
                    _ => (),
                }
            }
        }
    }
    has_bvp && has_acc && has_ibi && has_hr && has_tags && has_temp && has_eda
}