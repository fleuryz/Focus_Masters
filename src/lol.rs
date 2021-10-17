#![recursion_limit="1024"]

use std::sync::mpsc;
use std::thread;
use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use time;
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::sessao::Sessao;
use crate::lol_structs::{ChampionMasteryDTO, ChampionList, MatchTimelineDto, MatchDto, MetadataDto};
use crate::gui::{SearchData, DisplayData};
use crate::empatica;
use crate::db_structures::*;

use mongodb::{Client, options::ClientOptions};
use mongodb::error::Result as mongoResult;
use mongodb::bson::{Document, doc};
use mongodb::options::FindOptions;
use futures::executor::block_on;
use futures::stream::StreamExt;
use serde::Deserialize;
use winapi::um::winuser;
//use reqwest::Result;
use conrod::backend::glium::glium;
use xml_rpc::{Fault, Server};

#[derive(Debug)]
pub enum LoLState{
    WAITING,
    COLLECTING,
    FINISHED,
    NONE,
}

#[derive(Debug)]
pub struct GameRole{}

#[derive(Debug)]
pub struct Goal{}

#[derive(Debug)]
pub struct Champion{}

#[derive(Debug)]
pub struct Settings{
    pub exe_location: String,
    pub champion_list: HashMap<u32, String>,
}

impl Settings{
    pub fn new(location: &str) -> Settings{
        let mut arquivo = fs::File::open("./assets/lol/champions.txt").expect("Erro ao abrir arquivo de teste.");
        let mut champions = String::new();
        arquivo.read_to_string(&mut champions).unwrap();
    
        let champion_list: HashMap<u32, String>= serde_json::from_str(&champions).unwrap();
        Settings{
            exe_location: location.to_string(),
            champion_list,
        }
    }

    pub fn get_champion_image<'a>(&self, champion_id: u32, display_data: &'a mut DisplayData)-> Option<&'a conrod::image::Id>{
        if display_data.champions_image_map.contains_key(&champion_id){
            return Some(display_data.champions_image_map.get(&champion_id).unwrap());
        }

        let assets = std::env::current_dir().unwrap();

        let mut image_path_str = assets.join("assets\\lol\\images\\champions\\");
        let image_path = std::path::Path::new(&image_path_str);
        for entry in fs::read_dir(image_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            let file_name = path.file_name();
            //println!("{:?}", file_name);
        }
        let champion_name = self.champion_list.get(&champion_id);
        if champion_name.is_none(){
            println!("Champion image was not fount: {}", champion_id);
            return None;
        }

        let champion_name = champion_name.unwrap();

        image_path_str.push(format!("{}_0.jpg", champion_name.as_str()));
        
        let rgba_image = image::open(&std::path::Path::new(&image_path_str)).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &rgba_image.into_raw(),
        image_dimensions,
        );
        let texture = glium::texture::Texture2d::new(&display_data.display, raw_image).unwrap();

        display_data.champions_image_map.insert(champion_id, display_data.image_map.insert(texture));
        let champion_image = display_data.champions_image_map.get(&champion_id);
        
        return Some(champion_image.unwrap());
    }

    pub fn get_item_image<'a>(&self, item_id: u32, display_data: &'a mut DisplayData)-> &'a conrod::image::Id{
        if display_data.itens_image_map.contains_key(&item_id){
            return display_data.itens_image_map.get(&item_id).unwrap();
        }

        let assets = std::env::current_dir().unwrap();

        let mut image_path_str = assets.join("assets\\lol\\images\\item\\");
        let image_path = std::path::Path::new(&image_path_str);
        for entry in fs::read_dir(image_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            let file_name = path.file_name();
            //println!("{:?}", file_name);
        }
        image_path_str.push(format!("{}.png", item_id));
        let rgba_image = image::open(&std::path::Path::new(&image_path_str)).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &rgba_image.into_raw(),
        image_dimensions,
        );
        let texture = glium::texture::Texture2d::new(&display_data.display, raw_image).unwrap();

        display_data.champions_image_map.insert(item_id, display_data.image_map.insert(texture));
        let champion_image = display_data.champions_image_map.get(&item_id);
        
        return champion_image.unwrap();
    }

    pub fn get_champion_name(&mut self, champion_id: u32) -> Option<String>{

        match self.champion_list.get(&champion_id){
            Some(name)=> Some(name.clone()),
            None=> None,
        }
    }
}

#[derive(Debug)]
pub struct LoLSession{
    pub id: String,
    start_time: time::Tm,
    end_time: time::Tm,
    session_number: usize,
    keys: Vec<LoLData>,
    affections: Vec<LoLData>,
    processed: bool,
    loaded: bool,
}

impl LoLSession{
    pub fn new(username: &str, start_time: time::Tm, end_time: time::Tm, session_number: usize, keys: Vec<LoLData>, affections: Vec<LoLData>) -> LoLSession{
        let session = LoLSession{
            id: format!("0"),
            start_time,
            end_time,
            session_number,
            keys,
            affections,
            processed: false,
            loaded: true,
        };

        session.save(username);
        
        let keys_file_location = format!("./Data/LoL/Sessions/{}/{}/keys.lkans", username, session_number);
        let affections_file_location = format!("./Data/LoL/Sessions/{}/{}/affections.lkans", username, session_number);
        
        let mut keys_file = File::create(&keys_file_location).expect(format!("Error creating file {}.", &keys_file_location).as_str());
        let mut affections_file = File::create(&affections_file_location).expect(format!("Error creating file {}.", &affections_file_location).as_str());
        
        for key in &session.keys{
            key.write(&mut keys_file);
        }

        for affection in &session.affections{
            affection.write(&mut affections_file);
        }

        keys_file.flush().expect("Error while saving LoL session data file.");
        affections_file.flush().expect("Error while saving LoL session data file.");
    
        session
    }

    pub fn save(&self, username: &str) -> Result<(), Box<dyn Error>>{
        let info_file_location = format!("./Data/LoL/Sessions/{}/{}/info.lkans", username, self.session_number);
        let mut info_file = File::create(&info_file_location)?;
        
        write!(info_file, "{}\n", time::strftime("%Y/%m/%d/%H/%M/%S/%f", &self.start_time)?)?;
        write!(info_file, "{}\n", time::strftime("%Y/%m/%d/%H/%M/%S/%f", &self.end_time)?)?;
        write!(info_file, "{}\n", self.session_number)?;
        write!(info_file, "{}\n", self.processed)?;
        write!(info_file, "{}\n", self.id)?;
        
        info_file.flush()?;

        Ok(())
    }

    pub fn delete_data(&mut self, username: &str) -> Result<(), Box<dyn Error>>{
        let keys_file_location = format!("./Data/LoL/Sessions/{}/{}/keys.lkans", username, self.session_number);
        let affections_file_location = format!("./Data/LoL/Sessions/{}/{}/affections.lkans", username, self.session_number);

        std::fs::remove_file(keys_file_location)?;
        std::fs::remove_file(affections_file_location)?;

        //É PARA TIRAR ISSO, EM?

        let keys_file_location = format!("./Data/LoL/Sessions/{}/{}/keys.lkans", username, self.session_number);
        let affections_file_location = format!("./Data/LoL/Sessions/{}/{}/affections.lkans", username, self.session_number);
        
        let mut keys_file = File::create(&keys_file_location).expect(format!("Error creating file {}.", &keys_file_location).as_str());
        let mut affections_file = File::create(&affections_file_location).expect(format!("Error creating file {}.", &affections_file_location).as_str());
        
        for key in &self.keys{
            key.write(&mut keys_file);
        }

        for affection in &self.affections{
            affection.write(&mut affections_file);
        }

        keys_file.flush().expect("Error while saving LoL session data file.");
        affections_file.flush().expect("Error while saving LoL session data file.");

        //ATÉ AQUI

        self.affections = Vec::new();
        self.keys = Vec::new();

        Ok(())
    }

    pub fn load_session(username: &str, session: i64) -> Result<LoLSession, Box<dyn Error>>{
        let file_location = format!("./Data/LoL/Sessions/{}/{}/info.lkans", username, session);
        let mut file = File::open(file_location)?;
        
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut data = data.lines();

        let start_time = time::strptime(data.next().unwrap(), "%Y/%m/%d/%H/%M/%S/%f")?;
        let end_time = time::strptime(data.next().unwrap(), "%Y/%m/%d/%H/%M/%S/%f")?;
        let session_number: usize = data.next().unwrap().parse()?;
        let processed: bool = data.next().unwrap().parse()?;
        let id = format!("{}", data.next().unwrap());
        
        Ok(LoLSession{id, start_time,end_time,session_number,processed, keys: Vec::new(), affections: Vec::new(), loaded: false})
    }

    pub fn load(username: &str, last_processed_session: usize) -> Vec<LoLSession>{
        let mut sessions: Vec<LoLSession> = Vec::new();
        if let Ok(entries) = fs::read_dir(format!("./Data/LoL/Sessions/{}/", username)) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    if let Ok(file_type) = entry.file_type() {
                        // Now let's show our entry's file type!
                        //println!("{:?}: {:?} ,is_dir: {:?}", entry.path(), entry.path().file_stem(), file_type.is_dir());
                        let session_number: i64 = entry.path().file_stem().expect("Error in stem.").to_str().expect("Error in stem as str").parse().expect("Error parsing stem.");
                        if let Ok(session) = LoLSession::load_session(username, session_number){
                            sessions.push(session);
                        }
                        /*
                        Remover a parte de carregar os dados da sessão. Estão aqui só para eu testar uma coisa
                        
                        println!("Vou começar a processar aqui os arquivos");
                        sessions.last_mut().expect("Error getting last session").process(username);*/
                        
                    } else {
                        println!("Couldn't get file type for {:?}", entry.path());
                    }
                }
            }
        }

        sessions
    }

    pub fn load_data(&mut self, username: &str) -> Result<(), Box<dyn Error>>{
        if !self.loaded{
            let keys_file_location = format!("./Data/LoL/Sessions/{}/{}/keys.lkans", username, self.session_number);
            let affections_file_location = format!("./Data/LoL/Sessions/{}/{}/affections.lkans", username, self.session_number);
            let mut keys_file = File::open(keys_file_location).expect("Error opening info file.");
            let mut affections_file = File::open(affections_file_location).expect("Error opening info file.");
            
            let mut keys_string = String::new();
            keys_file.read_to_string(&mut keys_string).unwrap();
            let mut affections_string = String::new();
            affections_file.read_to_string(&mut affections_string).unwrap();
            /*
            let keys_string = io::BufReader::new(keys_file).lines();
            let affections_string = io::BufReader::new(affections_file).lines();
            //file.read_to_string(&mut data_string).unwrap();*/

            for keys_each_string in keys_string.lines(){
                //if let Ok(keys_each_string) = keys_each_string{
                    //println!("key string: {:?}", keys_each_string);
                    self.keys.push(LoLData::get_data(&keys_each_string));
                    //println!("last key: {:?}", self.keys.last());
                //}
            }

            for affections_each_string in affections_string.lines(){
                //if let Ok(affections_each_string) = affections_each_string{
                    //println!("affection string: {:?}", affections_each_string);
                    self.affections.push(LoLData::get_data(&affections_each_string));
                //}
            }
            //println!("depois de dar o load: {}", self.affections.len());
            self.loaded = true;

            self.save(username);
        }
        
        Ok(())
    }

    pub fn get_keys(&mut self)-> Vec<LoLData>{
        let mut return_keys = Vec::new();
        for _i in 0..self.keys.len(){
            //return_keys.push(self.keys.remove(0));
            return_keys.push(self.keys[_i].clone());
        }
        return_keys
    }

    pub fn get_affections(&mut self)-> Vec<LoLData>{
        let mut return_affections = Vec::new();
        for _i in 0..self.affections.len(){
            //return_affections.push(self.affections.remove(0));
            return_affections.push(self.affections[_i].clone());
        }
        return_affections
    }

    pub fn update_id(&mut self, username: &str, id: &str){
        self.id = format!("{}", id);
        self.save(username);
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub accountId: String,
    pub puuid: String,
    pub name: String,
    pub profileIconId: usize,
    pub revisionDate: usize,
    pub summonerLevel: usize,
}

impl User{
    pub fn new() -> User{
        User{
            id: "0".to_string(),
            accountId: "0".to_string(),
            puuid: "0".to_string(),
            name: "0".to_string(),
            profileIconId: 0,
            revisionDate: 0,
            summonerLevel: 0,
        }
    }

    pub fn save(&self, mut file: &File){
        writeln!(file, "{}", self.id);
        writeln!(file, "{}", self.accountId);
        writeln!(file, "{}", self.puuid);
        writeln!(file, "{}", self.name);
        writeln!(file, "{}", self.profileIconId);
        writeln!(file, "{}", self.revisionDate);
        writeln!(file, "{}", self.summonerLevel);
    }

    pub fn load(lines: &mut std::str::Lines) -> User{
        let id = String::from(lines.next().unwrap());
        let accountId = String::from(lines.next().unwrap());
        let puuid = String::from(lines.next().unwrap());
        let name = String::from(lines.next().unwrap());
        let profileIconId: usize = lines.next().unwrap().parse().unwrap(); 
        let revisionDate: usize = lines.next().unwrap().parse().unwrap();
        let summonerLevel: usize = lines.next().unwrap().parse().unwrap();

        User{
            id,
            accountId,
            puuid,
            name,
            profileIconId,
            revisionDate,
            summonerLevel,
        }
    }
}

#[derive(Debug)]
pub struct LoLU {
    pub user: User,
    pub lanes: Vec<usize>,
    pub champion_ids: Vec<isize>,
    pub state: LoLState, 
    pub settings : Settings,
    pub sessions : Vec<LoLSession>,
    pub last_process_time: i64,
    pub last_processed_session: usize,
    pub db : Option<mongodb::Database>,
    pub matches: Vec<MatchData>,
    pub matches_stats: Vec<MatchStats>,
    pub champion_stats: Vec<ChampionStats>,
    pub lane_stats: Vec<LaneStats>,
    pub timelines: Vec<MatchTimeline>,

}

impl LoLU {
    pub fn new(user: User, lanes: Vec<usize>, champion_ids: Vec<isize>, settings: Settings) -> LoLU {
        LoLU {
            user,
            lanes,
            champion_ids,
            state: LoLState::NONE,
            settings,
            sessions: Vec::new(),
            last_process_time: 0,
            last_processed_session: 0,
            db: None,
            matches: Vec::new(),
            matches_stats: Vec::new(),
            champion_stats: Vec::new(),
            lane_stats: Vec::new(),
            timelines: Vec::new(),
        }
    }

    pub fn update_data(&mut self, lanes: Vec<usize>, champion_ids: Vec<isize>){
        self.lanes = lanes;
        self.champion_ids = champion_ids;
    }

    //Saves the user file
    pub fn save(&self){
        let user_dir = format!("./Data/LoL/Sessions/{}", self.user.name);
        if !Path::new(&user_dir).is_dir(){
            fs::create_dir(user_dir).unwrap();
        }
            

        let nome = format!("./Data/LoL/Users/{}.luser", self.user.name);
        let mut file = File::create(nome).expect("Error creating file.");

        self.user.save(&mut file);
        
        write!(file, "{}", &self.lanes[0]).expect("Error while writing lanes");
        if self.lanes.len() > 0{
            for i in 1..(self.lanes.len()){
                write!(&file, ",{}", self.lanes[i]).expect("Error while writing lanes");
            }
            file.write(b"\n").unwrap();
        }
        

        write!(file, "{}", &self.champion_ids[0]).expect("Error while writing champion ids");
        if self.champion_ids.len() > 0{
            for i in 1..(self.champion_ids.len()){
                write!(&file, ",{}", &self.champion_ids[i]).expect("Error while writing champion ids");
            }
        }

        write!(file, "\n{}\n{}\n", self.last_process_time, self.last_processed_session);


        file.flush().expect("Error while saving lol user file.");
    }

    //Loads de user file
    pub fn load(username: &str) -> LoLU {
        let file_location = format!("./Data/LoL/Users/{}.luser", username);
        let mut file = File::open(file_location).expect("Error opening luser file");
        
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let mut data = data.lines();
        
        let user = User::load(&mut data);
        let lanes:Vec<usize> = data.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let champion_ids:Vec<isize> = data.next().unwrap().split(',').map(|x| x.parse::<isize>().unwrap()).collect();
        let info = data.next().unwrap();
        let last_process_time: i64 = info.parse::<i64>().unwrap();
        let info = data.next().unwrap();
        let last_processed_session: usize = info.parse::<usize>().unwrap();
        let sessions = LoLSession::load(username, last_processed_session);

        //Checar no diretório da sessões quantas sessões tem então carregar uma por uma
        /*for x in 0..sessions_number{
            let session_file_location = format!("./Data/LoL/Sessions/{}/{}/", username, x);
            let mut sessions_file = File::open(session_file_location).expect("Error opening sessions");
            let mut session_data = String::new();

        }*/
        
        let mut user = LoLU{
            user,
            lanes,
            champion_ids,
            state: LoLState::NONE,
            settings: Settings::new("C:\\Riot Games\\Riot Client\\RiotClientServices.exe"),
            sessions,
            last_process_time,
            last_processed_session,
            db: None,
            matches: Vec::new(),
            matches_stats: Vec::new(),
            champion_stats: Vec::new(),
            lane_stats: Vec::new(),
            timelines: Vec::new(),
        };
        
        //Vou colocar aqui uns testes com o mongoDB mas é para apagar depois. O mongodb vai ser criado junto com o usuário e os dados vão ser adicionados quando a sessão for processada
        let future = user.connect_to_db();
        user.db = Some(block_on(future).unwrap());

        block_on(user.get_matches_db(20));
        block_on(user.get_champions_db(10));
        block_on(user.get_lanes_db());

        user.process_matches();

        user.champion_stats.sort_by(|x, y| y.stats.matches.cmp(&x.stats.matches));
        user.lane_stats.sort_by(|x, y| y.stats.matches.cmp(&x.stats.matches));

        
        user
    }

    //Starts the game and recording threads and saves them in a file
    pub fn start(&mut self, s_gui: std::sync::mpsc::Sender<std::result::Result<(),usize>>, s_session: std::sync::mpsc::Sender<LoLSession>) {
        let game_location = self.settings.exe_location.to_string();

        let username = self.user.name.clone();

        let session_number: usize = self.sessions.len() + 1;

        fs::create_dir(format!("./Data/LoL/Sessions/{}/{}", username, session_number)).unwrap();
        self.save();

        let video_screen_name: String = format!("./Data/LoL/Sessions/{}/{}/screen.mp4", username, session_number);

        let face_video: String = format!("Data/LoL/Sessions/{}/{}/face.mp4", username, session_number);

        let _thread_start = thread::Builder::new().name("Session_Thread".to_string()).spawn(move || {
            let mensagem_erro = format!("Erro ao abrir o League of Legends: ({}).", game_location);
            Command::new(game_location)
                .arg("--launch-product=league_of_legends")
                .arg("--launch-patchline=live")
                .current_dir("C:/Riot Games/Riot Client")
                .spawn()
                .expect(&mensagem_erro)
            ;

            let mut first;
            let mut second;
            let mut state;
            loop {
                state = unsafe { winuser::GetAsyncKeyState(0x4C) };
                first = state == -32768 || state == -32767;
                state = unsafe { winuser::GetAsyncKeyState(winuser::VK_CONTROL) };
                second = state == -32768 || state == -32767;
                
                if first && second {
                    s_gui.send(Ok(())).unwrap();
                    break;
                }
                unsafe { winapi::um::synchapi::SleepEx(1,1); }
                //thread::sleep(std::time::Duration::from_millis(10));
            }

            let (s_keys_1, r_keys_1) = mpsc::channel();
            //let (s_keys_2, r_keys_2) = mpsc::channel();
            let (s_screen_1, r_screen_1) = mpsc::channel();
            let (s_screen_2, r_screen_2) = mpsc::channel();
            let (s_face_1, r_face_1) = mpsc::channel();
            let (s_face_2, r_face_2) = mpsc::channel();

            let start_time = time::now();

            let thread_keys = thread::Builder::new().name("Keys_Thread".to_string()).spawn(move || {
                let mut ctrl_pressed = false;
                s_keys_1.send(format!("{}-keys-start", Sessao::data_string(time::now()))).expect("Error sending time to s_keys pipe.");
                loop{
                    for i in 1..190 {
                        if unsafe { winuser::GetAsyncKeyState(i) } == -32767 {
                            let key: String  = match i as i32 {
                                32 => "SPACE".into(),
                                8 => "BACKSPACE".into(),
                                13 => "[RETURN]".into(),
                                winuser::VK_TAB => "TAB".into(),
                                winuser::VK_SHIFT => "SHIFT".into(),
                                winuser::VK_CONTROL => continue,
                                winuser::VK_LCONTROL => "LCTRL".into(),
                                winuser::VK_MENU => continue,
                                winuser::VK_LMENU => "LALT".into(),
                                winuser::VK_ESCAPE => "[ESCAPE]".into(),
                                winuser::VK_END => "END".into(),
                                winuser::VK_HOME => "HOME".into(),
                                winuser::VK_LEFT => "LEFT".into(),
                                winuser::VK_UP => "UP".into(),
                                winuser::VK_RIGHT => "RIGHT".into(),
                                winuser::VK_DOWN => "DOWN".into(),
                                winuser::VK_LBUTTON => "LEFT MOUSE".into(),
                                winuser::VK_RBUTTON => "RIGHT MOUSE".into(),
                                190|110 => ".".into(),
                                _ => {
                                    (i as u8 as char).to_string()
                                },
                            };
                            s_keys_1.send(format!("{}-Key-{}", Sessao::data_string(time::now()), key)).expect("Error sending key to gui pipe.");
                        }
                    }

                    let mut state = unsafe { winuser::GetAsyncKeyState(0x53) };
                    let first = state == -32768 || state == -32767;
                    state = unsafe { winuser::GetAsyncKeyState(winuser::VK_CONTROL) };
                    let second = state == -32768 || state == -32767;
                    
                    if first && second {
                        s_gui.send(Ok(())).unwrap();
                        s_keys_1.send(format!("{}-keys-end", Sessao::data_string(time::now()))).expect("Error sending time to s_keys pipe.");
                        //LoLSession::new(session_number ,&mut values);
                        s_face_1.send("end").unwrap();
                        s_screen_1.send("end").unwrap();
                        s_keys_1.send(format!("end")).expect("Error sending time to s_keys pipe.");
                        return;
                    }
                    unsafe { winapi::um::synchapi::SleepEx(1,1); }
                    //thread::sleep(std::time::Duration::from_millis(10));
                }
            }).expect("Error starting thread to record keys");

            let screen_thread = thread::Builder::new().name("Screen_Thread".to_string()).spawn(move || {
                let mut screen_recording = Command::new("ffmpeg")
                    //.args(&["-video_size", "1920x1080", "-framerate", "30", "-f", "x11grab", "-i", ":0.0", "-f", "alsa", "-ac", "2", "-i", "hw:0",
                    //  "-c:v", "libx264", "-crf", "0", "-preset", "ultrafast", &nome_video])
                    .args(&[
                        "-f",
                        "gdigrab",
                        "-framerate",
                        "30",
                        "-video_size",
                        "1920x1080",
                        "-i",
                        "desktop",
                        "-c:v",
                        "h264_nvenc",
                        "-crf",
                        "0",
                        &video_screen_name,
                        //"video_screen_name.avi",
                    ])
                    .stdout(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Screen was not recorded");

                s_screen_2.send(format!("{}-screen-start", Sessao::data_string(time::now()))).expect("Error sending start time to s_screen_2 pipe.");

                loop {
                    if r_screen_1.try_recv().is_ok() {

                        {
                            let screen_stdin = screen_recording.stdin.as_mut().expect("Failed to open stdin on screen thread");
                            let mut writer = std::io::BufWriter::new(screen_stdin);
                            writer.write_all(b"q").unwrap();
                        }

                        screen_recording.wait().expect("The process to record screen was not finished");
                        //screen_recording.kill().expect("Could not finish screen video recording.");
                        s_screen_2.send(format!("{}-screen-end", Sessao::data_string(time::now()))).expect("Error sending end time to s_screen_2 pipe.");
                        s_screen_2.send(format!("end")).expect("Error sending closing.");
                        break;
                    }
                }
            }).expect("Error starting thread to record screen.");
            
            let face_thread = thread::Builder::new().name("Face_Thread".to_string()).spawn(move || {
                static mut teste_str: String = String::new();
                static mut pode: bool = false;
                static mut finalizar: bool = false;
                fn teste(data_string: String) -> Result<(), Fault>{
                    unsafe {
                        teste_str = data_string.clone();
                        pode = true;
                    }
                    Ok(())
                }
                fn fechando(nada: i64) -> Result<(), Fault>{
                    unsafe {
                        finalizar = true;
                    }
                    Ok(())
                }

                let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
                let mut server = Server::new();
                
                server.register_simple("send_data", &teste);
                server.register_simple("end_data", &fechando);    

                let bound_server = server.bind(&socket).unwrap();

                thread::spawn(move || {bound_server.run();});

                let mut face_recording = Command::new("python")
                    .args(&[
                            "./Python/saveAndProcessCam.py",
                            &face_video,
                        ])
                    .stdout(Stdio::piped())
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Camera was not recorded")
                ;

                /*let mut stdout = face_recording.stdout.take().unwrap();
                let mut output = String::new();*/
                let timeout = Duration::from_millis(10);

                //s_face_2.send(format!("{}-face-start", Sessao::data_string(time::now()))).expect("Error sending start time to s_face_2 pipe.");
                //println!("Continuou depois de chamar a função");

                thread::spawn(move || unsafe{
                    let hundred_millis = std::time::Duration::from_millis(100);
                    while !finalizar{
                        if pode{
                            s_face_2.send(teste_str.clone()).expect("Error sending face detection lines");
                            pode = false;
                        }
                        //thread::sleep(hundred_millis);
                    }
                /*if let Ok(size) = stdout.read_to_string(&mut output){
                    //let line = String::from_utf8(output.to_vec());
                    
                }*/
                    s_face_2.send(format!("end")).expect("Error sending face detection lines");
                });

                loop {
                    if r_face_1.recv_timeout(timeout).is_ok() {
                        {
                            let screen_stdin = face_recording.stdin.as_mut().expect("Failed to open stdin on screen thread");
                            let mut writer = std::io::BufWriter::new(screen_stdin);
                            writer.write_all(b"q").unwrap();
                        }

                        //s_face_2.send(format!("{}-face-end", LoLU::tm_to_milisec(time::now()))).expect("Error sending end time to s_face_2 pipe.");

                        return;
                    }

                    /*
                    if r_face_1.try_recv().is_ok() {
                        {
                            println!("Vai enviar o q");
                            let mut screen_stdin = face_recording.stdin.as_mut().expect("Failed to open stdin on screen thread");
                            let mut writer = std::io::BufWriter::new(screen_stdin);
                            writer.write_all(b"q").unwrap();
                        }


                        //face_recording.wait().expect("The process to record screen was not finished");
                        //face_recording.kill().expect("Webcam was not killed or was already dead.");
                        //face_recording.wait().expect("Error while waiting");
                        let output = face_recording.wait_with_output().expect("Could not wait with output");

                        println!("Vai pegar aqui os sinais");
                        for line in String::from_utf8(output.stdout).unwrap().lines() {
                            //println!("{}", line);
                            s_face_2.send(line.into()).expect("Error sending face detection lines");
                        }

                        /*let status_code = match face_recording.try_wait().unwrap() {
                            Some(status) => status.code(),
                            None => {
                                // child hasn't exited yet
                                println!("Vai matar");
                                face_recording.kill().unwrap();
                                println!("Matou");
                                face_recording.wait().unwrap().code()
                            }
                        };*/
                        
                        s_face_2.send(format!("{}-face-end", LoLU::tm_to_milisec(time::now()))).expect("Error sending end time to s_face_2 pipe.");
                        return;
                    }*/
                }
            }).expect("Error starting thread to record face from webcam.");

            /*thread_keys.join().unwrap();
            screen_thread.join().unwrap();
            face_thread.join().unwrap();*/

            let mut keys: Vec<LoLData> = Vec::new();
            let mut affections: Vec<LoLData> = Vec::new();
            let end_time = time::now();
            let mut keys_running = true;
            let mut screen_running = true;
            let mut face_running = true;

            let hundred_millis = std::time::Duration::from_millis(100);
            while keys_running || screen_running || face_running {
                if keys_running{
                    let mut message = r_keys_1.try_recv();
                    while message.is_ok(){
                        let message_str = message.as_ref().unwrap();
                        if message_str == "end"{
                            keys_running = false;
                            /*let mut iter = r_keys_2.iter();
                            keys.push(LoLData::get_data_str_time(&iter.next().unwrap()));
                            keys.push(LoLData::get_data_str_time(&iter.next().unwrap()));*/
                        }else{
                            keys.push(LoLData::get_data_str_time(message_str));
                        } 
                        message = r_keys_1.try_recv();
                    }
                }
                if screen_running{
                    let mut message = r_screen_2.try_recv();
                    while message.is_ok(){
                        let message_str = message.as_ref().unwrap();
                        if message_str == "end"{
                            screen_running = false;
                        }else{
                            keys.push(LoLData::get_data_str_time(message_str));
                        }
                        message = r_screen_2.try_recv();
                    }
                }
                if face_running{
                    let mut message = r_face_2.try_recv();
                    while message.is_ok(){
                        let message_str = message.as_ref().unwrap();
                        if message_str == "end"{
                            face_running = false;
                        }else if message_str.len() > 5{
                            affections.push(LoLData::get_data(message_str));
                        }
                        message = r_face_2.try_recv();
                    }
                }
                thread::sleep(hundred_millis);
            }
            /*for line in r_keys_1 {
                keys.push(LoLData::get_data_str_time(&line));
            }

            let mut iter = r_keys_2.iter();
            keys.push(LoLData::get_data_str_time(&iter.next().unwrap()));
            keys.push(LoLData::get_data_str_time(&iter.next().unwrap()));

            let mut iter = r_screen_2.iter();
            keys.push(LoLData::get_data_str_time(&iter.next().unwrap()));
            keys.push(LoLData::get_data_str_time(&iter.next().unwrap()));

            for line in r_face_2{
                if line.len() > 5{
                    //println!("{:?}", line);
                    //affections.push(LoLData::get_data_str_time(&line));
                    affections.push(LoLData::get_data(&line));
                }
            }
            let mut iter = r_face_2.iter();
            values.push(LoLData::get_data(&iter.next().unwrap()));
            values.push(LoLData::get_data(&iter.next().unwrap()));*/
            keys.sort();

            s_session.send(LoLSession::new(&username, start_time, end_time, session_number, keys, affections)).expect("Could not send the session data");
        });
    }

    /*
    pub fn play(&self, sender: std::sync::mpsc::Sender<std::result::Result<(),usize>>){
        let (t1, r1) = mpsc::channel();
        let (t2, r2) = mpsc::channel();
        //let (t3, r3) = mpsc::channel();

        let session_number: usize = self.sessions_number;
        let thread_game = thread::Builder::new().name("Game_Thread".to_string()).spawn(move || {
            let mut values: Vec<String> = Vec::new();
            let mut ctrl_pressed = false;
            loop{
                for i in 1..190 {
                    if unsafe { winuser::GetAsyncKeyState(i) } == -32767 {
                        let key: String  = match i as i32 {
                            32 => " ".into(),
                            8 => "Backspace".into(),
                            13 => "\n".into(),
                            winuser::VK_TAB => "TAB".into(),
                            winuser::VK_SHIFT => "SHIFT".into(),
                            winuser::VK_CONTROL => {
                                                    ctrl_pressed = true;
                                                    "CTRL".into()
                                                },
                            winuser::VK_ESCAPE => "[ESCAPE]".into(),
                            winuser::VK_END => "END".into(),
                            winuser::VK_HOME => "HOME".into(),
                            winuser::VK_LEFT => "LEFT".into(),
                            winuser::VK_UP => "UP".into(),
                            winuser::VK_RIGHT => "RIGHT".into(),
                            winuser::VK_DOWN => "DOWN".into(),
                            winuser::VK_LBUTTON => "LEFT MOUSE".into(),
                            winuser::VK_RBUTTON => "RIGHT MOUSE".into(),
                            190|110 => ".".into(),
                            _ => (i as i64 as char).to_string()
                        };
                        values.push(format!("{}-KB-{}", LoLU::from_tm(time::now()), key));
                    }
                }
                let mut state = unsafe { winuser::GetAsyncKeyState(0x53) };
                let first = state == -32768 || state == -32767;
                state = unsafe { winuser::GetAsyncKeyState(winuser::VK_CONTROL) };
                let second = state == -32768 || state == -32767;

                println!("first: {} \n second: {}\n", first, second);
                
                if first && second {
                    println!("Vai sair");
                    sender.send(Ok(())).unwrap();
                    //LoLSession::new(session_number ,&mut values);
                    t1.send("end").unwrap();
                    t2.send("end").unwrap();
                    return;
                }
                unsafe { winapi::um::synchapi::SleepEx(1,1); }
            }
        });

        let video_screen_name: String = format!("./Data/LoL/Sessions/{}/screen.avi", session_number);

        let screen_thread = thread::Builder::new().name("Screen_Thread".to_string()).spawn(move || {
            let mut screen_recording = Command::new("ffmpeg")
                //.args(&["-video_size", "1920x1080", "-framerate", "30", "-f", "x11grab", "-i", ":0.0", "-f", "alsa", "-ac", "2", "-i", "hw:0",
                //  "-c:v", "libx264", "-crf", "0", "-preset", "ultrafast", &nome_video])
                .args(&[
                    "-f",
                    "gdigrab",
                    "-framerate",
                    "30",
                    "-video_size",
                    "1920x1080",
                    "-i",
                    "desktop",
                    "-c:v",
                    "h264_nvenc",
                    "-crf",
                    "0",
                    &video_screen_name,
                    //"video_screen_name.avi",
                ])
                .spawn()
                .expect("Screen was not recorded");

            loop {
                if r1.try_recv().is_ok() {
                    println!("Vai terminar a gravação da tela");
                    //let mut screen_stdin = screen_recording.stdin.unwrap();
                    //let mut writer = std::io::BufWriter::new(&mut screen_stdin);
                    //writer.write_all(b"q").unwrap();
                    screen_recording.kill().expect("Could not finish screen video recording.");
                    break;
                }
            }
            println!("Vai finalizar o programa de gravação de tela");
        });

        let face_video: String = format!("Data/LoL/Sessions/{}/face.avi", session_number);
        //let face_video: String = format!("face.avi");

        let face_thread = thread::Builder::new().name("Face_Thread".to_string()).spawn(move || {
            let mut face_recording = Command::new("py")
                .args(&[
                        "./Python/saveCam.py",
                        &face_video,
                    ])
                .spawn()
                .expect("Camera was not recorded");

            loop {
                if r2.try_recv().is_ok() {
                    //nix::sys::signal::kill(Pid::from_raw(filmagem.id() as i32), Signal::SIGINT)
                    //    .unwrap();
                    face_recording.kill().expect("Webcam recording could not be killed.");
                    return;
                }
            }
        });
    }*/

    pub fn process_e4_files(&mut self, directory: &str) -> std::result::Result<(), Box<dyn std::error::Error>>{

        let mut e4_data = empatica::E4_Data::new(directory)?;
        e4_data.start_python_rpc()?;
        
        let mut tag_number = 2;

        while tag_number < e4_data.tags.len(){
            let start_time = e4_data.tags[tag_number];
            let end_time = e4_data.tags[tag_number+1];
            //println!("sessions: {:?}", self.sessions);
            
            for session in &mut self.sessions{
                if LoLU::is_close_tm(LoLU::tm_to_milisec(session.start_time), start_time as i64, 300) && LoLU::is_close_tm(LoLU::tm_to_milisec(session.end_time), end_time as i64, 300){
                    if session.processed{
                        println!("This session was already processed. Nothing was done.");
                    }else{
                        session.load_data(&self.user.name)?;
                        session.affections.append(&mut e4_data.get_data_vec(tag_number)?);
                        let timeline_collection = self.db.as_ref().unwrap().collection("Timeline");

                        let mut cursor = block_on(timeline_collection.find(doc! {"_id": session.id.as_str()}, None))?;
                        let mut match_data = None;
                        while let Some(result) = block_on(cursor.next()){
                            //println!("result: {:?}", result);
                            match result {
                                Ok(document) => {
                                    let mut match_timeline = MatchTimeline::from_document(document);
                                    match_timeline.bvp = e4_data.bvp.get_lol_data("bvp", start_time, end_time)?;
                                    match_timeline.eda = e4_data.eda.get_eda_data(start_time, end_time)?;
                                    let heart_rate_response = e4_data.bvp.get_hrv_data(start_time, end_time)?;
                                    match_timeline.hr = heart_rate_response.0;
                                    match_timeline.hrv = heart_rate_response.1;
                                    //match_timeline.eda = e4_data.eda.get_lol_data("eda", start_time, end_time)?;
                                    match_timeline.ibi = e4_data.ibi.get_lol_data("ibi", start_time, end_time)?;
                                    match_timeline.hr = e4_data.hr.get_lol_data("hr", start_time, end_time)?;
                                    
                                    match_timeline.temp = e4_data.temp.get_lol_data("temp", start_time, end_time)?;
                                    match_data = Some(match_timeline);
                                }
                                Err(e) => return Err(e.into()),
                            }
                        }
                        
                        if match_data.is_some(){
                            let document = match_data.unwrap().get_bson().as_document().unwrap().clone();
                            //println!("document: {}", document);
                            block_on(timeline_collection.find_one_and_replace(doc! {"_id": session.id.as_str()}, document, None))?;
                            session.processed = true;
                            session.save(&self.user.name)?;
                            session.delete_data(&self.user.name)?;
                        }
                    }
                }
            }

            tag_number += 2;
        }
        
        e4_data.end_python_rpc()?;
        Ok(())
    }

    pub async fn connect_to_db(&mut self) ->  mongoResult<mongodb::Database>{
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        // Manually set an option.
        client_options.app_name = Some("Focus".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;

        // Get a handle to a database.
        let db = client.database("Focus");

        
        // Get a handle to a collection in the database.
        let db_collection = db.collection("General");

        return mongoResult::Ok(db);
    }

    pub fn process_matches(&mut self){
        //Filtrar partidas que não são do summoner's rift
        let match_ids = LoLU::get_matches_id(&self.user.puuid, 0, 20).unwrap();

        let mut new_last_process_time = self.last_process_time;

        for each_match_id in match_ids{
            let mut each_match = None;
            if self.matches.iter().any(|x| x._id.eq(&each_match_id)){
                continue;
            }
            for _error_count in 0..5 {
                each_match = LoLU::get_match(&each_match_id, self.user.name.as_str(), &self.user.puuid);
    
                if each_match.is_some(){
                    break;
                }
            }
            //if (each_match.role == "NONE" && each_match.role == "NONE") || each_match.lane == "NONE"{
              //  continue;
            //}
            if each_match.is_none(){
                continue;
            }
                
            let (each_match_data, each_match_stats): (MatchData, MatchStats) = each_match.unwrap();
            
            let session_to_process = self.last_processed_session + 1;
            let last_session_time = match self.last_processed_session{
                0 => 0,
                _ => LoLU::tm_to_milisec(self.sessions[self.last_processed_session-1].end_time),
            };
            let this_session_time;
            if session_to_process > self.sessions.len(){
                this_session_time = 0;
            }else{
                this_session_time = LoLU::tm_to_milisec(self.sessions[session_to_process-1].start_time);
            }
            new_last_process_time = each_match_data.start_date as i64;
            if session_to_process <= self.sessions.len() &&
                LoLU::is_close_tm(this_session_time, each_match_data.start_date as i64, 600) &&
                //LoLU::tm_to_milisec(self.sessions[session_to_process].start_time) <= each_match.timestamp &&
                last_session_time <= each_match_data.start_date as i64
            {
                //println!("Essa partida faz parte de sessão");
                let match_id = each_match_data._id.clone();
                block_on(self.add_general_to_db(each_match_data, each_match_stats, session_to_process, this_session_time as i64, LoLU::tm_to_milisec(self.sessions[session_to_process-1].end_time) as i64));
                block_on(self.add_timeline_to_db(match_id.as_str(), session_to_process));
                
                self.last_processed_session = session_to_process;
            }else{
                //println!("Essa partida não faz parte de sessão");
                let match_id = each_match_data._id.clone();
                block_on(self.add_general_to_db(each_match_data, each_match_stats, 0, 0, 0));
                block_on(self.add_timeline_to_db(match_id.as_str(), 0));                
            }
        }
        block_on(self.add_stats_to_db());

        self.last_process_time = new_last_process_time;
        self.save();
    }

    pub async fn get_matches_db(&mut self, number_of_matches: usize) -> mongoResult<()>{
        let db_collection = self.db.as_ref().unwrap().collection("General");
        let options = FindOptions::builder()
            .sort(doc!{"start_date" : -1})
            .limit(number_of_matches as i64)
            .batch_size(5)
            .build();
        let mut cursor = db_collection.find(doc!{"username": self.user.name.as_str()}, Some(options)).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let match_data = MatchData::from_document(document);
                    self.matches.push(match_data);
                    //println!("{:?}", match_data);
                }
                Err(e) => return Err(e.into()),
            }
        }
    
        return mongoResult::Ok(());
    }

    pub async fn get_champions_db(&mut self, number_of_champions: usize) -> mongoResult<()>{
        let db_collection = self.db.as_ref().unwrap().collection("Champion_Stats");
        let options = FindOptions::builder()
            .sort(doc!{"stats.matches" : -1})
            .limit(number_of_champions as i64)
            .batch_size(5)
            .build();
        let mut cursor = db_collection.find(doc!{"username": self.user.name.as_str()}, Some(options)).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let champion_stats = ChampionStats::from_document(document);
                    //println!("{:?}", champion_stats);
                    self.champion_stats.push(champion_stats);
                }
                Err(e) => return Err(e.into()),
            }
        }
    
        return mongoResult::Ok(());
    }

    pub async fn get_lanes_db(&mut self) -> mongoResult<()>{
        let db_collection = self.db.as_ref().unwrap().collection("Lane_Stats");
        let options = FindOptions::builder()
            .sort(doc!{"stats.matches" : -1})
            .batch_size(5)
            .build();
        let mut cursor = db_collection.find(doc!{"username": self.user.name.as_str()}, Some(options)).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let lane_stats = LaneStats::from_document(document);
                    //println!("{:?}", lane_stats);
                    self.lane_stats.push(lane_stats);
                }
                Err(e) => return Err(e.into()),
            }
        }
    
        return mongoResult::Ok(());
    }

    pub fn create_champion_stats(&mut self, current_match: &MatchData, current_match_stats: &MatchStats){
        let champion_stats_index = self.champion_stats.iter().position(|x| x.champion == current_match.champion);
        let mut champion_stats: &mut ChampionStats;
        let total_champion_stats = self.champion_stats.len();
        if let Some(index) = champion_stats_index {
            champion_stats = &mut self.champion_stats[index];
        }else{
            self.champion_stats.push(ChampionStats::new(current_match.champion, current_match.username.as_str()));
            champion_stats = &mut self.champion_stats[total_champion_stats];
        }

        champion_stats.stats.fill_stats(current_match.win, current_match.participant_id, current_match_stats);
        
    }

    pub fn create_lane_stats(&mut self, current_match: &MatchData, current_match_stats: &MatchStats){
        let match_stats_index = self.lane_stats.iter().position(|x| x.role == current_match.role);
        let mut lane_stats: &mut LaneStats;
        let total_lane_stats = self.lane_stats.len();
        if let Some(index) = match_stats_index {
            lane_stats = &mut self.lane_stats[index];
        }else{
            self.lane_stats.push(LaneStats::new(&current_match.role, current_match.username.as_str()));
            lane_stats = &mut self.lane_stats[total_lane_stats];
        }

        
        let participant_index = current_match_stats.participant_stats.iter().position(|x| x.participantId.eq(&current_match.participant_id)).unwrap();
        //let participant_team = current_match.team_stats[participant_index].teamId;

        if !lane_stats.champions.contains_key(&(current_match_stats.participant_stats[participant_index].championId as u32)) {
            lane_stats.champions.insert(current_match_stats.participant_stats[participant_index].championId as u32, 1);
        }else{
            if let Some(appearances) = lane_stats.champions.get_mut(&(current_match_stats.participant_stats[participant_index].championId as u32)) {
                *appearances += 1;
            }
        }

        lane_stats.stats.fill_stats(current_match.win, current_match.participant_id, current_match_stats);

        /*let mut team_kills = 0;
        current_match.team_stats.iter().filter(|x| x.teamId == participant_team).for_each(|x| team_kills += x.stats.kills);
        let mut team_gold = 0;
        current_match.team_stats.iter().filter(|x| x.teamId == participant_team).for_each(|x| team_gold += x.stats.goldEarned);

        lane_stats.stats.matches += 1;
        if current_match.win{
            lane_stats.stats.wins += 1;
        }else{
            lane_stats.stats.losses += 1;
        }

        let mut matches_division: f64 = (lane_stats.stats.matches-1) as f64/(lane_stats.stats.matches as f64);
        if matches_division <= 0.0{
            matches_division = 1.0;
        }
        let ka = (current_match.team_stats[participant_index].stats.kills + current_match.team_stats[participant_index].stats.assists) as f64;
        let kda = ka/current_match.team_stats[participant_index].stats.deaths as f64;
        let kp = ka/team_kills as f64;
        let kda_div = kda/(lane_stats.stats.matches as f64);
        let kp_div = kp/(lane_stats.stats.matches as f64);
        let last_kda_div = lane_stats.stats.kda_per_match/matches_division;
        let last_kp_div = lane_stats.stats.kp_per_match/matches_division;

        let kda_per_match = last_kda_div + kda_div;
        let kp_per_match = last_kp_div + kp_div;

        let gold = current_match.team_stats[participant_index].stats.goldEarned as f64/team_gold as f64;
        let gold_div = gold/(lane_stats.stats.matches as f64);
        let last_gold_div = lane_stats.stats.kp_per_match/matches_division;
        let gold_percentage_per_match = last_gold_div + gold_div;


        lane_stats.stats.kills += current_match.team_stats[participant_index].stats.kills;
        lane_stats.stats.assists += current_match.team_stats[participant_index].stats.assists ;
        lane_stats.stats.deaths += current_match.team_stats[participant_index].stats.kills;
        lane_stats.stats.kda_per_match = kda_per_match;
        lane_stats.stats.kp_per_match = kp_per_match;
        lane_stats.stats.gold_percentage_per_match = gold_percentage_per_match;*/
    }

    pub fn tm_to_string(time: time::Tm) -> String{
        format!("{:04}/{:02}/{:02}/{:02}/{:02}/{:02}/{:08}", (time.tm_year + 1900), (time.tm_mon + 1), time.tm_mday, time.tm_hour, time.tm_min, time.tm_sec, time.tm_nsec )
    }

    pub fn tm_to_milisec(time: time::Tm) -> i64{
        let timespec = time.to_timespec();
        //10,800 seconds (3 hours) is added to the time to convert to Brasília time
        let milisec_time = (timespec.sec + 10800) * 1000 + (timespec.nsec as i64)/1000000;
        return milisec_time;
    }

    pub fn mili_timestamp_to_tm(timestamp: i64) -> time::Tm{
        let sec = timestamp/1000;
        let nsec = (timestamp - sec*1000)*1000;
        time::at(time::Timespec::new(sec,nsec as i32))
    }

    pub fn is_close_tm(time_1: i64, time_2: i64, closeness_in_sec: i64) -> bool{
        let closeness = closeness_in_sec*1000;
        /*println!("closenes: {}", closeness);
        println!("comparison: {}", time_2);
        println!("minus: {}\n reference: {}\nplus: {}", (time_2 - closeness), time_1, (time_2 + closeness));
        println!("return: {}", (time_1 <= (time_2 + closeness) && time_1 >= (time_2 - closeness)));*/
        time_1 <= (time_2 + closeness) && time_1 >= (time_2 - closeness)
    }

    pub fn duration_to_string(duration: i64) -> String{
        let seconds = (duration/1000)%60;
        let minutes = (duration/60000)%60;
        let hours = duration/3600000;

        format!("{}h {}min {}sec", hours, minutes, seconds)
    }

    pub fn check_username(username: &str) -> reqwest::Result<User>{
        let mut arquivo = fs::File::open("./assets/lol/API.txt").expect("Erro ao abrir arquivo de teste.");
        let mut api_key = String::new();
        arquivo.read_to_string(&mut api_key).unwrap();
        let request_url = format!("https://br1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{summonerName}?api_key={api_key}",
            summonerName = username,
            api_key = api_key);
        
       //println!("{}", request_url);
        let response  = reqwest::blocking::get(&request_url)?.json::<User>();
        //let response  = reqwest::blocking::get(&request_url)?.text()?;
        //println!("{:?}", response);
        response
    }

    pub fn get_champions(summoner_id: &str) -> reqwest::Result<Vec<ChampionMasteryDTO>>{
        let mut arquivo = fs::File::open("./assets/lol/API.txt").expect("Erro ao abrir arquivo de teste.");
        let mut api_key = String::new();
        arquivo.read_to_string(&mut api_key).unwrap();
        let request_url = format!("https://br1.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/{encryptedSummonerId}?api_key={api_key}",
        encryptedSummonerId = summoner_id,
        api_key = api_key);


        let response: reqwest::Result<Vec<ChampionMasteryDTO>> = reqwest::blocking::get(&request_url)?.json::<Vec<ChampionMasteryDTO>>();
        //let response  = reqwest::blocking::get(&request_url)?.text()?;
        response
    }

    pub fn get_champions_images(image_map: &mut conrod::image::Map<conrod::glium::Texture2d>, display: &glium::Display) -> HashMap<String, (conrod::image::Id, String)>{
        let mut arquivo = fs::File::open("./assets/lol/champions.txt").expect("Erro ao abrir arquivo de teste.");
        let mut champions = String::new();
        arquivo.read_to_string(&mut champions).unwrap();

        let champions: HashMap<u32, String>  = serde_json::from_str(&champions).unwrap();

        let assets = std::env::current_dir().unwrap();

        let mut champion_images: HashMap<String, (conrod::image::Id, String)> = HashMap::new();

        for champion in champions{
            let mut image_path_str = assets.join("assets\\lol\\images\\champions\\");
            let image_path = std::path::Path::new(&image_path_str);
            for entry in fs::read_dir(image_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                
                let file_name = path.file_name();
                //println!("{:?}", file_name);
                
            }

            image_path_str.push(format!("{}_0.jpg", champion.1.as_str()));
            
            let rgba_image = image::open(&std::path::Path::new(&image_path_str)).unwrap().to_rgba();
            let image_dimensions = rgba_image.dimensions();
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &rgba_image.into_raw(),
            image_dimensions,
            );
            let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
            champion_images.insert((&champion.0).to_string(), (image_map.insert(texture), champion.1.clone()) ) ;
        }


        champion_images
    }

    pub fn get_lanes_images(image_map: &mut conrod::image::Map<conrod::glium::Texture2d>, display: &glium::Display) ->Vec<conrod::image::Id>{
        let image_path = std::env::current_dir().unwrap().join("assets\\lol\\images\\lanes\\");
        let mut image_file_names: Vec<std::path::PathBuf> = Vec::new();
        image_file_names.push(image_path.join("jungle.png"));
        image_file_names.push(image_path.join("top.png"));
        image_file_names.push(image_path.join("mid.png"));
        image_file_names.push(image_path.join("adc.png"));
        image_file_names.push(image_path.join("sup.png"));
        let mut ids: Vec<conrod::image::Id> = Vec::new();
        
        for file_name in image_file_names{
            let image = image::open(&std::path::Path::new(&file_name));
            let rgba_image = image.unwrap().to_rgba();
            let image_dimensions = rgba_image.dimensions();
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &rgba_image.into_raw(),
                image_dimensions,
                );
            let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
            ids.push(image_map.insert(texture));
        }
        

        ids
    }

    pub fn get_matches_id(puuid: &str, first: i64, count: i64) ->reqwest::Result<Vec<String>>{
        let mut arquivo = fs::File::open("./assets/lol/API.txt").expect("Erro ao abrir arquivo de teste.");
        let mut api_key = String::new();
        arquivo.read_to_string(&mut api_key).unwrap();
        let request_url = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids?start={start}&count={count}&api_key={api_key}",
            puuid = puuid,
            start = first,
            count = count,
            api_key = api_key);

        //println!("request_url: {:?}", request_url);

        //let response: reqwest::Result<MatchlistDto> = reqwest::blocking::get(&request_url)?.json::<MatchlistDto>();
        let response = reqwest::blocking::get(&request_url)?;
        //println!("response: {:?}", response);

        //let end_time = time::now();

        /*println!("response: {:?}", response);
        println!("time: {:?}", end_time);
        println!("timespec: {:?}", end_time.to_timespec());*/

        response.json::<Vec<String>>()
    }

    pub fn get_match(game_id: &str, username: &str, account_puuid: &str) -> Option<(MatchData, MatchStats)>{
        let mut arquivo = fs::File::open("./assets/lol/API.txt").expect("Erro ao abrir arquivo de teste.");
        let mut api_key = String::new();
        arquivo.read_to_string(&mut api_key).unwrap();

        let request_url = format!("https://americas.api.riotgames.com/lol/match/v5/matches/{game_id}?api_key={api_key}",
            game_id = game_id,
            api_key = api_key);

        //println!("request: {}", request_url);

        //let response: reqwest::Result<MatchTimelineDto> = reqwest::blocking::get(&request_url)?.json::<MatchTimelineDto>();

        let response = reqwest::blocking::get(&request_url).expect("Error getting match data");
        
        
        let response_text: String = response.text().expect("Error converting response to string.");

        //println!("response: {:?}", reqwest::blocking::get(&request_url).unwrap().text().unwrap().get(2922..3002) );
        //println!("\n\nresponse: {:?}\n\n", response_text);
        //response

        if response_text.contains("TUTORIAL_MODULE"){

            println!("Should not fetch: {:?}", response_text);
            return None
        }

        let match_data: std::result::Result<MatchDto, serde_json::error::Error> = serde_json::from_str(&response_text);

        match match_data{
            Ok(match_data) => Some((MatchData::new(&match_data, username, account_puuid), MatchStats::new(&match_data, username, account_puuid))),
            Err(err) => {
                println!("Error: {:?}", err);
                None
            }
        }

        /*match match_data{
            Ok(match_data) => Some((MatchData::new(&match_data, username, account_id, session as u32), MatchStats::new(&match_data, username, account_id))),
            Err(err) => {
                println!("Error: {:?}", err);
                None
            }
        }
        Option<(MatchData, MatchStats)>*/

        
        //Some(MatchData::new(&reqwest::blocking::get(&request_url).expect("Error getting match data").json::<MatchDto>().unwrap(), session, account_id))
    }

    pub fn get_match_timeline(game_id: &str, username: &str, session: u32, participant_id: i64, team_id: i64, colleagues_ids:Vec<i64>, keys: Vec<LoLData>, facial_inferings: Vec<LoLData>) -> Option<MatchTimeline>{
        let mut arquivo = fs::File::open("./assets/lol/API.txt").expect("Erro ao abrir arquivo de teste.");
        let mut api_key = String::new();
        arquivo.read_to_string(&mut api_key).unwrap();

        let request_url = format!("https://americas.api.riotgames.com/lol/match/v5/matches/{matchId}/timeline?api_key={api_key}",
            matchId = game_id,
            api_key = api_key
        );

        //println!("request: {}", request_url);

        let response = reqwest::blocking::get(&request_url).expect("Error getting match data");
        
        
        let response_text: String = response.text().expect("Error converting response to string.");

        //println!("response: {:?}", reqwest::blocking::get(&request_url).unwrap().text().unwrap().get(2922..3002) );
        //println!("response: {:?}",response_text );

        //response

        /*if response_text.contains("TUTORIAL_MODULE"){
            println!("{:?}", response_text);
            return None
        }*/

        let match_data: std::result::Result<MatchTimelineDto, serde_json::error::Error> = serde_json::from_str(&response_text);
        if match_data.is_ok(){
            //TODO arrumar o 0 com o "participant_puuid"
            Some(MatchTimeline::new(match_data.unwrap().info, username, participant_id, team_id, colleagues_ids, game_id, keys, facial_inferings))
        }else{
            println!("Error: {:?}", match_data.unwrap_err());
            //println!("{:?}", response_text);
            None
        }
        
        //reqwest::blocking::get(&request_url)?.json::<MatchTimelineDto>()
    }

    pub fn get_champion_from_id (champion_id: u32, champions: &HashMap<u32, String>) -> Option<String>{
        match champions.iter().find(|x| *x.0 == champion_id){
            Some(champion) => Some(champion.1.clone()),
            None => None,
        }
    }

    pub fn get_id_from_champion (champion_name: String, champions: &HashMap<u32, String>) -> Option<i64>{ 
        match champions.iter().find(|x| x.1.to_ascii_lowercase().contains(&champion_name.to_ascii_lowercase())){
            Some(champion) => Some(*champion.0 as i64),
            None => None,
        }
    }

    pub async fn add_stats_to_db(&self){
        let champion_collection = self.db.as_ref().unwrap().collection("Champion_Stats");
        let lane_collection = self.db.as_ref().unwrap().collection("Lane_Stats");
        
        for each_stat in &self.champion_stats{
            if each_stat._id.oid.eq(&format!("")) {
                champion_collection.insert_one((*each_stat.get_bson().as_document().unwrap()).clone(), None).await;
            }else{
                champion_collection.update_one(doc! {"champion": each_stat.champion}, (*each_stat.get_bson().as_document().unwrap()).clone(), None).await;
            }
        }
        
        for each_stat in &self.lane_stats{
            if each_stat._id.oid.eq(&format!("")) {
                lane_collection.insert_one((*each_stat.get_bson().as_document().unwrap()).clone(), None).await;
            }else{
                lane_collection.update_one(doc! {"role": &each_stat.role}, (*each_stat.get_bson().as_document().unwrap()).clone(), None).await;
            }
        }
        
    }

    pub async fn add_timeline_to_db(&mut self, match_id: &str, session: usize){
        let current_match = self.matches.last().unwrap();
        let current_match_stats = self.matches_stats.last().unwrap();
        let mut colleagues_ids: Vec<i64> = Vec::new();
        for stat in &current_match_stats.participant_stats{
            if stat.teamId == current_match.team_id as i64{
                colleagues_ids.push(stat.participantId as i64);
            }
        }
        let participant_id = current_match.participant_id;
        let team_id = current_match.team_id;
        let match_data;
        if session > 0 {
            self.sessions[session-1].update_id(&self.user.name, match_id);
            self.sessions[session-1].load_data(&self.user.name).unwrap();
            match_data = LoLU::get_match_timeline(match_id, self.user.name.as_str(), session as u32, participant_id, team_id, colleagues_ids, self.sessions[session-1].get_keys(), self.sessions[session-1].get_affections());
        }else{
            match_data = LoLU::get_match_timeline(match_id, self.user.name.as_str(), session as u32, participant_id, team_id, colleagues_ids, Vec::new(), Vec::new());
        }
        // Get a handle to a collection in the database.
        let collection = self.db.as_ref().unwrap().collection("Timeline");

        if match_data.is_some(){
            let match_data = match_data.unwrap();
            collection.insert_one((*match_data.get_bson().as_document().unwrap()).clone(), None).await;
            self.timelines.push(match_data);
        }
    }

    pub async fn add_general_to_db(&mut self, mut match_data: MatchData, match_stats_data: MatchStats, session: usize, start_time: i64, end_time: i64){
        //let match_reference = &matches.matches[matches.endIndex-1];
        
        // Get a handle to a collection in the database.
        let general_collection = self.db.as_ref().unwrap().collection("General");
        let stats_collection = self.db.as_ref().unwrap().collection("Match_Stats");

        match_data.session = session as u32;

        if start_time != 0 && match_data.start_date > start_time{
            match_data.start_date = start_time;
        }
        if end_time > 0{
            let match_time = end_time - match_data.start_date;
            if match_data.match_time < match_time{
                match_data.match_time = match_time;
            }else{
                match_data.match_time = match_data.match_time;
            }
        }else{
            match_data.match_time = match_data.match_time;
        }

        general_collection.insert_one((*match_data.get_bson().as_document().unwrap()).clone(), None).await;
        stats_collection.insert_one((*match_stats_data.get_bson().as_document().unwrap()).clone(), None).await;
        self.create_champion_stats(&match_data, &match_stats_data);
        self.create_lane_stats(&match_data, &match_stats_data);
        self.matches.push(match_data);
        self.matches_stats.push(match_stats_data);
    }

    pub fn find_matches(&mut self, search_data: &SearchData) -> Result<Vec<usize>, Box<dyn Error>>{
        let collection = self.db.as_ref().unwrap().collection("General");

        let mut found_matches_indexes: Vec<usize> = Vec::new();

        let mut search_document = Vec::new();//Document::new();
        search_document.push(doc!{"username": self.user.name.as_str()});

        if search_data.champion.is_some(){
            let champion_names: Vec<&str> = search_data.champion.as_ref().unwrap().trim().split('|').collect();
            let mut champion_ids = Vec::new();
            for champion in champion_names{
                if let Some(champion_id) = LoLU::get_id_from_champion(champion.trim().to_string(), &self.settings.champion_list){
                    champion_ids.push(doc!{"champion": champion_id});
                }
            }
            search_document.push(doc!{"$or": champion_ids});
        }
        if search_data.team.is_some(){
            let champion_names: Vec<&str> = search_data.team.as_ref().unwrap().trim().split('|').collect();
            let mut champion_ids = Vec::new();
            for champion in champion_names{
                if let Some(champion_id) = LoLU::get_id_from_champion(champion.trim().to_string(), &self.settings.champion_list){
                    champion_ids.push(doc!{"team": champion_id});
                }
            }
            search_document.push(doc!{"$or": champion_ids});
        }
        if search_data.oppontents.is_some(){
            let champion_names: Vec<&str> = search_data.oppontents.as_ref().unwrap().trim().split('|').collect();
            let mut champion_ids = Vec::new();
            for champion in champion_names{
                if let Some(champion_id) = LoLU::get_id_from_champion(champion.trim().to_string(), &self.settings.champion_list){
                    champion_ids.push(doc!{"oppontents": champion_id});
                }
            }
            search_document.push(doc!{"$or": champion_ids});
        }
        if search_data.lane != -1{
            let mut lanes_search = Vec::new();
            lanes_search.push(doc!{"role": MatchData::get_role_from_num(5)});
            lanes_search.push(doc!{"role": MatchData::get_role_from_num(search_data.lane as usize)});
            search_document.push(doc!{"$or": lanes_search});
        }
        if search_data.won.is_some(){
            search_document.push(doc!{"win": true});
        }
        if search_data.lost.is_some(){
            search_document.push(doc!{"win": false});
        }
        if search_data.session.is_some(){
            search_document.push(doc!{"session": { "$ne": 0 }});
        }

        //println!("search_document: {:?}", search_document);

        let mut cursor = if search_document.len() > 0 {
            block_on(collection.find(doc!{"$and": search_document}, None))?
        }else{
            block_on(collection.find(doc!{}, None))?
        };

        while let Some(result) = block_on(cursor.next()){
            match result {
                Ok(document) => {
                    let match_data = MatchData::from_document(document);
                    //println!("match_data: {:?}", match_data);
                    match self.matches.iter().position(|each_match| each_match._id == match_data._id){
                        Some(match_index) =>{
                            found_matches_indexes.insert(0, match_index);
                        },
                        None =>{
                            self.matches.push(match_data);
                            found_matches_indexes.insert(0, (self.matches.len() - 1) as usize);
                        },
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(found_matches_indexes)
    }

    pub fn get_match_stats_db(&mut self, match_id: &str) -> Result<(), Box<dyn Error>>{
        
        let collection = self.db.as_ref().unwrap().collection("Match_Stats");
        let mut cursor = block_on(collection.find(doc!{"_id": match_id}, None))?;

        while let Some(result) = block_on(cursor.next()){
            match result {
                Ok(document) => {
                    let match_data = MatchStats::from_document(document);
                    self.matches_stats.push(match_data);
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            }
    
        }

        Err("Could not find Match Stats".into())
    }

    pub fn get_match_timeline_db(&mut self, match_id: &str) -> Result<(), Box<dyn Error>>{
        
        let collection = self.db.as_ref().unwrap().collection("Timeline");
        let mut cursor = block_on(collection.find(doc!{"_id": match_id}, None))?;

        while let Some(result) = block_on(cursor.next()){
            match result {
                Ok(document) => {
                    let match_data = MatchTimeline::from_document(document);
                    self.timelines.push(match_data);
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            }
    
        }

        Err("Could not find Match Timeline".into())
    } 

    pub fn export_video(user_name: &str, session: u32, start_time: i64, match_time: i64, id: &str){

        let mut export_video = Command::new("python")
            .args(&[
                    "./Python/exportVideo.py",
                    user_name,
                    &session.to_string(),
                    &start_time.to_string(),
                    &match_time.to_string(),
                    id,
                ])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect("Video not exported")
        ;

        export_video.wait().expect("Could not wait for video export.");

    }

}