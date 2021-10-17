use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::iter::{once, Once, Chain};
use std::slice::Iter;

use crate::variavel::Variavel;
use crate::lol_structs::{MatchDto, ParticipantDto, TeamDto, TimelineInfoDto, MatchParticipantFrameDto, MatchEventDto};
use crate::sessao::Sessao;
use crate::lol::LoLU;

use mongodb::bson::{bson, Document, Bson, doc};
use serde::Deserialize;

#[derive(Clone,Debug, Deserialize)]
pub struct Id{
    pub oid: String,
}
impl Id{
    pub fn new(oid: String) -> Id{
        Id{
            oid,
        }
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchData{
    pub _id: String,
    pub username: String,
    pub match_time : i64,
    pub start_date : i64,
    pub session: u32,
    pub win: bool,
    pub role: String,
    pub champion: u32,
    pub team: Vec<u32>,
    pub opponents: Vec<u32>,
    pub participant_id: i64,
    pub team_id: i64,
}

impl MatchData{

    pub fn new_empty() -> MatchData{
        MatchData{
            _id: format!("NONE"),
            username: format!("NONE"),
            match_time: 0,
            start_date: 0,
            session: 0,
            win: false,
            role: format!("UNKNOWN"),
            champion: 0,
            team: Vec::new(),
            opponents: Vec::new(),
            participant_id: 0,
            team_id: 0,
        }
    }

    pub fn new(match_info: &MatchDto, username: &str, account_puuid: &str) -> MatchData{
        let player_participant_id =  match_info.info.participants.iter().find(|x| x.puuid.eq(account_puuid)).unwrap().participantId;
        let team_id = match_info.info.participants.iter().find(|x| x.puuid.eq(account_puuid)).unwrap().teamId;
        let mut player = None;
        let mut opponents = Vec::new();
        let mut team = Vec::new();
        for participant in &match_info.info.participants{
            if participant.puuid.eq(account_puuid){
                player = Some(participant);
            }else{
                if participant.teamId == team_id{
                    team.push(participant.championId as u32);
                }else{
                    opponents.push(participant.championId as u32);
                }
            }
        }
        //let player = match_info.participants.iter().find(|participant| participant.participantId == player_participant_id).unwrap();

        MatchData{
            _id : match_info.metadata.matchId.clone(),
            username: format!("{}", username),
            match_time: match_info.info.gameDuration,
            start_date: match_info.info.gameCreation,
            session: 0,
            win: player.unwrap().win,
            role: player.unwrap().teamPosition.clone(),
            champion: (player.unwrap().championId as u32),
            team,
            opponents,
            participant_id: player_participant_id,
            team_id: team_id as i64,

        }
    }

    pub fn from_document(document: Document) -> MatchData{
        
        let document_string = serde_json::to_string(&document).unwrap().replace("$", "");

        serde_json::from_str(&document_string).unwrap()
        /*
        let match_time: i64 = if let Some(match_time) = document.get("match_time").and_then(Bson::as_i64){match_time as i64}else{0};
        let start_date: i64 = if let Some(start_date) = document.get("start_date").and_then(Bson::as_i64){start_date as i64}else{0};
        let season: i64 = if let Some(season) = document.get("season").and_then(Bson::as_i64){season as i64}else{0};
        let lane = if let Some(lane) = document.get("lane").and_then(Bson::as_str){format!("{}",lane)}else{format!("")};
        let champion: u32 = if let Some(champion) = document.get("champion").and_then(Bson::as_i64){champion as u32}else{0};
        let participant_id: i64 = if let Some(participant_id) = document.get("participant_id").and_then(Bson::as_i64){participant_id as i64}else{0};
        let stats: Vec<TeamStatsDto> = if let Some(stats) = document.get("stats").and_then(Bson::as_array){MatchData::from_teams_bson(stats)}else{Vec::new()};
        let team_stats: Vec<ParticipantDto> = if let Some(team_stats) = document.get("team_stats").and_then(Bson::as_array){MatchData::from_participants_bson(team_stats)}else{Vec::new()};
        let session: u32 = if let Some(session) = document.get("session").and_then(Bson::as_i64){session as u32}else{0};

        MatchData{
            match_time,
            start_date,
            season,
            lane,
            champion,
            participant_id,
            stats,
            team_stats,
            session,
        }*/
    }

    pub fn get_bson(&self) -> Bson{
        bson! ({
            "_id" : self._id.as_str(),
            "username" : self.username.clone(),
            "match_time" : self.match_time,
            "start_date" : self.start_date,
            "session" : self.session,
            "win": self.win,
            "role": self.role.clone(),
            "champion": self.champion,
            "team": self.team.clone(),
            "opponents": self.opponents.clone(),
            "participant_id": self.participant_id,
            "team_id": self.team_id as i64,
        })
    }

    pub fn role_num(&self) -> usize{
        match self.role.as_str(){
            "JUNGLE" => 0,
            "TOP" => 1,
            "MIDDLE" => 2,
            "ADC" => 3,
            "SUPPORT" => 4,
            _ => 0,

        }
    }

    pub fn get_role_from_num(role_num: usize) -> &'static str{
        match role_num{
            0 => "JUNGLE",
            1 => "TOP",
            2 => "MIDDLE",
            3 => "ADC",
            4 => "SUPPORT",
            _ => "UNKNOWN",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatchStats{
    pub _id: String,
    pub username: String,
    pub stats: Vec<TeamDto>,
    pub participant_stats: Vec<ParticipantDto>,
}

impl MatchStats{
    pub fn new_empty() -> MatchStats{
        MatchStats{
            _id: format!("NONE"),
            username: format!("NONE"),
            stats: Vec::new(),
            participant_stats: Vec::new(),
        }
    }

    pub fn new(match_info: &MatchDto, username: &str, account_puuid: &str) -> MatchStats{
        MatchStats{
            _id : match_info.metadata.matchId.clone(),
            username : format!("{}", username),
            stats: match_info.info.teams.clone(),
            participant_stats: match_info.info.participants.clone(),
        }
    }

    pub fn from_document(document: Document) -> MatchStats{
        
        let document_string = serde_json::to_string(&document).unwrap().replace("$", "");

        serde_json::from_str(&document_string).unwrap()
    }

    pub fn get_bson(&self) -> Bson{
        bson! ({
            "_id" : self._id.as_str(),
            "username" : self.username.clone(),
            "stats": self.get_participants_bson(),
            "participant_stats": self.get_teams_bson(),
        })
    }

    pub fn get_participants_bson(&self) -> Vec<Bson>{
        let mut participant_bson = Vec::new();
        for participant in &self.stats{
            participant_bson.push(participant.to_bson());
        }
        participant_bson
    }

    pub fn get_teams_bson(&self)-> Vec<Bson>{
        let mut team_bson = Vec::new();
        for team in &self.participant_stats{
            team_bson.push(team.to_bson());
        }
        team_bson
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ParticipantIdentity{
    pub participant_id: i64,
    pub player_name: String,
}

#[derive(Clone,Debug, Deserialize)]
pub struct ChampionStats{
    pub _id: Id,
    pub username: String,
    pub champion: u32,
    pub stats: GeneralStats,
}

impl ChampionStats{
    pub fn new(champion: u32, username: &str) -> ChampionStats{
        ChampionStats{
            _id: Id::new(format!("")),
            username: format!("{}", username),
            champion,
            stats: GeneralStats::new(),
        }
    }

    pub fn get_bson(&self) -> Bson{
        bson! ({
            "username" : self.username.clone(),
            "champion" : self.champion,
            "stats" : self.stats.get_bson(),
        })
    }

    pub fn from_document(document: Document) -> ChampionStats{
        
        let document_string = serde_json::to_string(&document).unwrap().replace("$", "");

        serde_json::from_str(&document_string).unwrap()
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct LaneStats{
    pub _id: Id,
    pub username: String,
    pub role: String,
    pub champions: HashMap<u32, i64>,
    pub stats: GeneralStats,
}
impl LaneStats{
    pub fn new(role: &str, username: &str) -> LaneStats{
        LaneStats{
            _id: Id::new(format!("")),
            username: format!("{}", username),
            role: format!("{}", role),
            champions: HashMap::new(),
            stats: GeneralStats::new(),
        }
    }

    pub fn get_bson(&self) -> Bson{
        let mut champions_doc: Document = Document::new();
        for (key, value) in &self.champions{
            champions_doc.insert(format!("{}",key), value);
        }
                    
        bson! ({
            "username" : self.username.clone(),
            "role" : format!("{}", self.role),
            "champions": Bson::Document(champions_doc),
            "stats" : self.stats.get_bson(),
        })
    }

    pub fn from_document(document: Document) -> LaneStats{
        
        let document_string = serde_json::to_string(&document).unwrap().replace("$", "");

        serde_json::from_str(&document_string).unwrap()
    }

    pub fn role_num(&self) -> usize{
        match self.role.as_str(){
            "JUNGLE" => 0,
            "TOP" => 1,
            "MIDDLE" => 2,
            "ADC" => 3,
            "SUPPORT" => 4,
            _ => 0,

        }
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct GeneralStats{
    pub matches: i64,
    pub wins: i64,
    pub losses: i64,
    pub kills: i64,
    pub assists: i64,
    pub deaths: i64,
    pub kda_per_match: f64,
    pub kp_per_match: f64,
    pub gold_percentage_per_match: f64,
}

impl GeneralStats{
    pub fn new()-> GeneralStats{
        GeneralStats{
            matches: 0,
            wins: 0,
            losses: 0,
            kills: 0,
            assists: 0,
            deaths: 0,
            kda_per_match: 0.0,
            kp_per_match: 0.0,
            gold_percentage_per_match: 0.0,
        }
    }

    pub fn fill_stats(&mut self, win: bool, participant_id: i64, current_match: &MatchStats){
        let participant_index = current_match.participant_stats.iter().position(|x| x.participantId == (participant_id)).unwrap();
        let participant_team = current_match.participant_stats[participant_index].teamId;
        let mut team_kills = 0;
        current_match.participant_stats.iter().filter(|x| x.teamId == participant_team).for_each(|x| team_kills += x.kills);
        let mut team_gold = 0;
        current_match.participant_stats.iter().filter(|x| x.teamId == participant_team).for_each(|x| team_gold += x.goldEarned);
        let mut deaths = current_match.participant_stats[participant_index].deaths;

        self.matches += 1;
        if win{
            self.wins += 1;
        }else{
            self.losses += 1;
        }

        let mut matches_division: f64 = (self.matches-1) as f64/(self.matches as f64);
        if matches_division <= 0.0{
            matches_division = 1.0;
        }
        if team_kills <= 0{
            team_kills = 1;
        }
        if team_gold <= 0{
            team_gold = 1;
        }
        if deaths <= 0{
            deaths = 1;
        }
        let ka = (current_match.participant_stats[participant_index].kills + current_match.participant_stats[participant_index].assists) as f64;
        let kda = ka/deaths as f64;
        let kp = ka/team_kills as f64;
        let kda_div = kda/(self.matches as f64);
        let kp_div = kp/(self.matches as f64);
        let last_kda_div = self.kda_per_match*matches_division;
        let last_kp_div = self.kp_per_match*matches_division;

        let kda_per_match = last_kda_div + kda_div;
        let kp_per_match = last_kp_div + kp_div;

        let gold = current_match.participant_stats[participant_index].goldEarned as f64/team_gold as f64;
        let gold_div = gold/(self.matches as f64);
        let last_gold_div = self.kp_per_match*matches_division;
        let gold_percentage_per_match = last_gold_div + gold_div;

        self.kills += current_match.participant_stats[participant_index].kills;
        self.assists += current_match.participant_stats[participant_index].assists ;
        self.deaths += current_match.participant_stats[participant_index].kills;
        //self.kda_per_match = ((self.kda_per_match*(self.matches-1) as f64) + ((current_match.team_stats[participant_index].stats.kills + current_match.team_stats[participant_index].stats.assists) as f64/current_match.team_stats[participant_index].stats.deaths as f64) as f64)/self.matches as f64;
        self.kda_per_match = kda_per_match;
        self.kp_per_match =  kp_per_match;
        self.gold_percentage_per_match = gold_percentage_per_match;
    }

    pub fn get_bson(&self) -> Bson{
        bson! ({
            "matches" : self.matches,
            "wins" : self.wins,
            "losses": (self.losses as i64),
            "kills": self.kills,
            "assists": self.assists,
            "deaths": self.deaths,
            "kda_per_match": self.kda_per_match,
            "kp_per_match": self.kp_per_match,
            "gold_percentage_per_match": self.gold_percentage_per_match,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchTimeline{
    pub _id: String,
    pub username: String,
    //pub session: u32,
    pub frames: Vec<MatchFrame>,
    pub frame_interval: i64,
    pub keys: Vec<LoLData>,
    pub facial_inferings: Vec<LoLData>,
    pub bvp: Vec<LoLData>,
    pub ibi: Vec<LoLData>,
    pub hr: Vec<LoLData>,
    pub hrv: Vec<LoLData>,
    pub temp: Vec<LoLData>,
    pub eda: Vec<LoLData>,
    //stress: Vec<LoLData>,
    //flow: Vec<LoLData>,
}
impl MatchTimeline{
    pub fn new(match_info: TimelineInfoDto, username: &str, participant_id: i64, team_id: i64, colleagues_ids:Vec<i64>, game_id: &str, keys: Vec<LoLData>, facial_inferings: Vec<LoLData>) -> MatchTimeline{

        //AQUI!!! BUSCAR OS DADOS QUE ESTÃO NO ARQUIVO DATA.LKANS E PROCESSAR O ROSTO
        let mut frames: Vec<MatchFrame> = Vec::new();
        for frame in match_info.frames{
            let mut participant_frame = None;
            let mut team_frames: Vec<MatchParticipantFrameDto> = Vec::new();
            let mut opposing_team_frames: Vec<MatchParticipantFrameDto> = Vec::new();
            for (_,v) in frame.participantFrames{
                if v.participantId == participant_id as i64{
                    participant_frame = Some(v);
                }else if colleagues_ids.contains(&v.participantId){
                    team_frames.push(v);
                }else{
                    opposing_team_frames.push(v);
                }
                
            }

            let events = frame.events.into_iter().filter(|x| x.is_from_participant(participant_id as i64, team_id)).collect();
            
            frames.push(MatchFrame{
                player_frame: participant_frame.unwrap().clone(),
                team_frames: team_frames,
                opposing_team_frames: opposing_team_frames,
                events: events,
                timestamp: frame.timestamp as i64,
            });
        }


        MatchTimeline{
            _id : format!("{}", game_id),
            username: format!("{}", username),
            //session,
            frames: frames,
            frame_interval: match_info.frameInterval as i64,
            keys,
            facial_inferings,
            bvp: Vec::new(),
            ibi: Vec::new(),
            hr: Vec::new(),
            hrv: Vec::new(),
            temp: Vec::new(),
            eda: Vec::new(),
        }
    }

    pub fn get_bson(&self) -> Bson{
        bson! ({
            "_id" : self._id.as_str(),
            "username" : self.username.clone(),
            //"session" : self.session,
            "frames" : self.get_frames_bson(),
            "frame_interval": self.frame_interval,
            "keys": MatchTimeline::get_data_bson(&self.keys),
            "facial_inferings": MatchTimeline::get_data_bson(&self.facial_inferings),
            "bvp": MatchTimeline::get_data_bson(&self.bvp),
            "eda": MatchTimeline::get_data_bson(&self.eda),
            "ibi": MatchTimeline::get_data_bson(&self.ibi),
            "hr": MatchTimeline::get_data_bson(&self.hr),
            "hrv": MatchTimeline::get_data_bson(&self.hrv),
            "temp": MatchTimeline::get_data_bson(&self.temp),
        })
    }

    pub fn get_frames_bson(&self) -> Vec<Bson>{
        let mut frames_bson = Vec::new();
        for frame in &self.frames{
            frames_bson.push(frame.to_bson());
        }
        frames_bson
    }

    pub fn get_data_bson(data_vec: &Vec<LoLData>) -> Vec<Bson>{
        let mut data_bson = Vec::new();
        for data in data_vec{
            data_bson.push(data.to_bson());
        }
        data_bson
    }

    pub fn from_document(document: Document) -> MatchTimeline{
        
        let document_string = serde_json::to_string(&document).unwrap().replace("$", "");

        serde_json::from_str(&document_string).unwrap()
    }

    pub fn get_match_plot_points(&self, data_type: PlotableValues, start_time: i64, duration: i64, x_size: f64, y_size: f64, max_value: &mut f64, team_id: i64) -> Result<Vec<[f64; 2]>, Box<dyn Error>>{
        let mut points = Vec::new();
        let min_value = 0.0;
        let mut current_value = 0.0;
        //let current_time = *self.frames[0].events[0].realTimestamp.as_ref().unwrap() as f64;
        let current_time = start_time as f64;
        points.insert(0,[start_time as f64, 0.0]);
        for frame in &self.frames{
            for event in &frame.events{
                let new_point;
            let mut current_time = current_time;
            match data_type{
                PlotableValues::Kills => {
                    if event.r#type.as_str() == "CHAMPION_KILL" && event.killerId.unwrap() == frame.player_frame.participantId {
                        current_value += 1.0;
                        current_time += event.timestamp as f64;
                        points.push([current_time, points.last().unwrap()[1]]);
                    }else{
                        continue;
                    }
                    new_point = [current_time, current_value];
                },
                PlotableValues::Deaths => {
                    if event.r#type.as_str() == "CHAMPION_KILL" && event.victimId.unwrap() == frame.player_frame.participantId {
                        current_value += 1.0;
                        current_time += event.timestamp as f64;
                        points.push([current_time, points.last().unwrap()[1]]);
                    }else{
                        continue;
                    }
                    new_point = [current_time, current_value];
                },
                PlotableValues::Assists => {
                    if event.r#type.as_str() == "CHAMPION_KILL" && event.assistingParticipantIds.as_ref().unwrap().contains(&frame.player_frame.participantId){
                        current_value += 1.0;
                        current_time += event.timestamp as f64;
                        points.push([current_time, points.last().unwrap()[1]]);
                    }else{
                        continue;
                    }
                    new_point = [current_time, current_value];
                },
                PlotableValues::Barons => {
                    if event.r#type.as_str() == "ELITE_MONSTER_KILL" && event.monsterType.as_ref().unwrap().as_str() == "BARON_NASHOR" && event.killerTeamId.unwrap() == team_id{
                        current_value += 1.0;
                        current_time += event.timestamp as f64;
                        points.push([current_time, points.last().unwrap()[1]]);
                    }else{
                        continue;
                    }
                    new_point = [current_time, current_value];
                },
                PlotableValues::Dragons => {
                    if event.r#type.as_str() == "ELITE_MONSTER_KILL" && event.monsterType.as_ref().unwrap().as_str() == "DRAGON" && event.killerTeamId.unwrap() == team_id{
                        current_value += 1.0;
                        current_time += event.timestamp as f64;
                        points.push([current_time, points.last().unwrap()[1]]);
                    }else{
                        continue;
                    }
                    new_point = [current_time, current_value];
                },
                _=> return Err("Not an available value to plot".into()),
            }

            if new_point[1] > *max_value{
                *max_value = new_point[1];
            }
            points.push(new_point);
            }
            
        }

        *max_value = (*max_value/5.0).ceil() * 5.0;
        let amplitude = *max_value - min_value;
        
        /*println!("max:{:?} - min:{:?} = {:?}", max_value, min_value, amplitude);
        println!("start:{:?} : duration:{:?}", start_time, duration);
        println!("x:{:?} : y:{:?}", x_size, y_size);
        println!("points before: {:?}", points);*/
        
        for i in 0..points.len(){
            points[i][0] = ((points[i][0] - start_time as f64)/(duration as f64))*x_size;
            points[i][1] = ((points[i][1])/amplitude)*y_size;
        }

        //println!("points after: {:?}", points);
    
        points.push([x_size, points.last().unwrap()[1]]);
        points.push([x_size, y_size]);
        
        Ok(points)
    }

    pub fn get_empatica_plot_points(points_vec: Vec<&LoLData>, start_time: i64, duration: i64, x_size: f64, y_size: f64, normalize: bool) -> Result<Vec<[f64; 2]>, Box<dyn Error>>{
        let mut points = Vec::new();
        let mut min_value = f64::MAX;
        let mut max_value = f64::MIN;
        let start_time = start_time;// + 10800000;
        
        for point in points_vec{
            let new_point = point.value_point()?;
            //println!("Valor: {:?}", new_point);

            if new_point[1] > max_value{
                max_value = new_point[1];
            }else if new_point[1] < min_value{
                min_value = new_point[1];
            }
            points.push(new_point);
        }
        //println!("Start: {}\nDuration: {}", start_time, duration);
        let amplitude = max_value - min_value;

        //println!("{:?} - {:?} = {:?}", max_value, min_value, amplitude);
        //println!("points: {:?}", points_vec);
        
        for i in 0..points.len(){
            let relative_time = (points[i][0] - start_time as f64)/(duration as f64);
            if relative_time > 1.0{
                for _j in i..points.len(){
                    points.remove(i);
                }
                break;
                
            }else{
                points[i][0] = relative_time*x_size;
                if normalize{
                    points[i][1] = ((points[i][1] - min_value)/amplitude)*y_size;
                }else{
                    points[i][1] = points[i][1]*y_size;
                }
                
            }
            
            //println!("Valor pós: {:?}", points[i]);
        }
        
        if normalize{
            points.insert(0,[0.0, points[0][1]]);
            points.push([x_size, points.last().unwrap()[1]]);
        }else{
            points.insert(0,[0.0, 0.0]);
            points.push([x_size, y_size]);
        }
        //println!("{:?} - {:?} = {:?}", max_value, min_value, amplitude);
        Ok(points)
    }

    pub fn export_video(&self) -> Result<(), Box<dyn Error>>{
        /*let video_program = "./Python/exportVideos.py";
        let output = Command::new(video_program)
            .arg(self.arquivo_video.as_str())
            .arg(Sessao::data_string(self.data_inicio))
            .arg("0")
            .output()
            .expect("Erro ao executar exportacao de video");*/
        //println!("{:?}", output);

        Ok(())
    }

}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchFrame{
    player_frame: MatchParticipantFrameDto,
    team_frames: Vec<MatchParticipantFrameDto>,
    opposing_team_frames: Vec<MatchParticipantFrameDto>,
    events: Vec<MatchEventDto>, 	
    timestamp: i64,
}

impl MatchFrame{
    pub fn to_bson(&self) -> Bson{
        bson! ({
            "player_frame" : self.player_frame.to_bson(),
            "team_frames" : self.get_team_frames_bson(),
            "opposing_team_frames" : self.get_opposing_team_frames_bson(),
            "events" : self.get_events_bson(),
            "timestamp" : self.timestamp,
        })
    }

    /*pub fn get_frames_bson(&self) -> Vec<Bson>{
        let mut frames_bson = Vec::new();
        for frame in &self.frames{
            frames_bson.push(frame.to_bson());
        }
        frames_bson
    }*/

    pub fn get_team_frames_bson(&self)-> Vec<Bson>{
        let mut frames_bson = Vec::new();
        for frame in &self.team_frames{
            frames_bson.push(frame.to_bson());
        }
        frames_bson
    }

    pub fn get_opposing_team_frames_bson(&self)-> Vec<Bson>{
        let mut frames_bson = Vec::new();
        for frame in &self.opposing_team_frames{
            frames_bson.push(frame.to_bson());
        }
        frames_bson
    }

    pub fn get_events_bson(&self) -> Vec<Bson>{
        let mut events_bson = Vec::new();
        for event in &self.events{
            events_bson.push(event.to_bson());
        }
        events_bson
    }
}

#[derive(Clone,Debug, Deserialize, Eq)]
pub struct LoLData{
    pub time: i64,
    pub name: String,
    pub value: Variavel,
}


impl Ord for LoLData {
    fn cmp(&self, other: &LoLData) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for LoLData {
    fn partial_cmp(&self, other: &LoLData) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LoLData {
    fn eq(&self, other: &LoLData) -> bool {
        self.time == other.time
    }
}

impl LoLData {
    pub fn new(time: time::Tm, name: String, value: Variavel) -> LoLData {
        LoLData{
            time: LoLU::tm_to_milisec(time) as i64,
            name,
            value,
        }
    }

    pub fn new_timestamp(time: i64, name: String, value: Variavel) -> LoLData{
        LoLData{
            time,//: time + 10800000,
            name,
            value,
        }
    }

    pub fn get_data(line: &str) -> LoLData{
        let values:Vec<_> = line.split('-').collect();
        let variable_values = match values[2].parse(){
            Ok(valor) => valor,
            Err(error) => {
                println!("Parsing error. Original string: ({:?})", line);
                panic!(error);
            },
        };
        LoLData::new_timestamp(values[0].parse().expect("Error parsing timestamp"), String::from(values[1]), variable_values) 
    }

    pub fn get_data_str_time(line: &str) -> LoLData{
        //println!("Original string: ({:?})", line);
        let values:Vec<_> = line.split('-').collect();
        let variable_values = match values[2].parse(){
            Ok(valor) => valor,
            Err(error) => {
                println!("Parsing error. Original string: ({:?})", line);
                panic!(error);
            },
        };
        LoLData::new(Sessao::to_tm(values[0]), String::from(values[1]), variable_values)
        
    }

    pub fn write(&self, file:&mut File){
        file.write(&self.time.to_string().as_bytes()).unwrap();
        file.write(b"-").unwrap();
        file.write(self.name.as_bytes()).unwrap();
        file.write(b"-").unwrap();
        self.value.escrever(file);
        file.write(b"\n").unwrap();
    }

    pub fn copiar(&self) -> LoLData {
        LoLData{
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

    pub fn to_bson(&self) -> Bson{
        bson!({
            "time": self.time,
            "name": self.name.clone(),
            "value": self.value_bson(),
          })
    }

    pub fn value_bson(&self) -> Bson{
        match self.value{
            Variavel::Int(ref value) =>  bson! ({"Int" : *value}),
            Variavel::Float(ref value) => bson! ({"Float" : *value}),
            Variavel::Booleano(ref value) => bson! ({"Booleano" : *value}),
            Variavel::Texto(ref value) => bson! ({"Texto" : value.clone()}),
        }
        /*
        match self.value{
            Variavel::Int(ref value) => Bson::Int32(*value),
            Variavel::Float(ref value) => Bson::Double(*value),
            Variavel::Booleano(ref value) => Bson::Boolean(*value),
            Variavel::Texto(ref value) => Bson::String(value.clone()),
        }*/
    }

    pub fn value_point(&self) -> Result<[f64; 2], Box<dyn Error>>{
        match self.value{
            Variavel::Int(ref value) => Ok([self.time as f64, *value as f64]),
            Variavel::Float(ref value) => Ok([self.time as f64, *value]),
            _=> Err("Variavel Value cannot be turned to Once".into()),
        }
    }
}

pub enum PlotableValues{
    Kills,
    Deaths,
    Assists,
    Barons,
    Dragons,
    Gold,
}