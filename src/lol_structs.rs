use serde::Deserialize;
use std::collections::HashMap;
use mongodb::bson::{bson, Document, Bson};

#[derive(Debug, Deserialize)]
pub struct ChampionMasteryDTO {
    pub championPointsUntilNextLevel: f64, // 	Number of points needed to achieve next level. Zero if player reached maximum champion level for this champion.
    pub chestGranted: bool, // 	Is chest granted for this champion or not in current season.
    pub championId: isize, // 	Champion ID for this entry.
    pub lastPlayTime: f64, // 	Last time this champion was played by this player - in Unix milliseconds time format.
    pub championLevel: u64, // 	Champion level for specified player and champion combination.
    pub summonerId: String, // 	Summoner ID for this entry. (Encrypted)
    pub championPoints: u64, // 	Total number of champion points for this player and champion combination - they are used to determine championLevel.
    pub championPointsSinceLastLevel: f64, // 	Number of points earned since current level has been achieved.
    pub tokensEarned: u64, // 	The token earned for this champion to levelup. 
}

/*#[derive(Debug, Deserialize)]
pub struct Third {
    pub r#type: String,
    pub format: String,
    pub version: String,
    pub data: Vec<Forth>,
}

#[derive(Debug, Deserialize)]
pub struct Forth {
    pub main: String,
    pub version: String,
    pub id: String,
    pub key: u64,
    pub name: String,
    pub title: String,
    pub blurb: String,
    //pub info: Fifth,

    

}*/

#[derive(Debug, Deserialize)]
pub struct ChampionList{
    pub key: String,
    pub name: String,
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchlistDto{
    pub matches: Vec<MatchReferenceDto>,
    pub startIndex: u64,
    pub endIndex: u64,
    pub totalGames: u64, 		
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchReferenceDto{
    pub gameId:  u64, 	
    pub role: String, 	
    pub season: u64, 	
    pub platformId: String,
    pub champion: u64, 	
    pub queue: u64,
    pub lane: String,	
    pub timestamp: i64, 
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchDto{
    pub gameId: u64,
    pub participantIdentities: Vec<ParticipantIdentityDto>, //Participant identity information. Participant identity information is purposefully excluded for custom games.
    pub queueId: u64,
    pub gameType: String,
    pub gameDuration: u64, 	                                //Match duration in seconds.
    pub teams: 	Vec<TeamStatsDto>,                          //Team information.
    pub platformId: String, 	                                //Platform where the match was played.
    pub gameCreation: u64, 	                                //Designates the timestamp when champion select ended and the loading screen appeared, NOT when the game timer was at 0:00.
    pub seasonId: u64,
    pub gameVersion: String, 	                            //The major.minor version typically indicates the patch the match was played on.
    pub mapId: 	u64,
    pub gameMode: String,
    pub participants: 	Vec<ParticipantDto>,  	            //Participant information. 
}

impl MatchDto{
    pub fn get_participants_bson(&self) -> Vec<Bson>{
        let mut participant_bson = Vec::new();
        for participant in &self.participants{
            participant_bson.push(participant.to_bson());
        }
        participant_bson
    }

    pub fn get_teams_bson(&self)-> Vec<Bson>{
        let mut team_bson = Vec::new();
        for team in &self.teams{
            team_bson.push(team.to_bson());
        }
        team_bson
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct ParticipantIdentityDto{
    pub participantId: 	u64,
    pub player:	PlayerDto,	                                //Player information not included in the response for custom matches. Custom matches are considered private unless a tournament code was used to create the match.
}

#[derive(Clone,Debug, Deserialize)]
pub struct PlayerDto{
    pub profileIcon: u64, 	
    pub accountId: String, 	                                //Player's original accountId.
    pub matchHistoryUri: String,	
    pub currentAccountId: String, 	                        //Player's current accountId when the match was played.
    pub currentPlatformId: String, 	                        //Player's current platformId when the match was played.
    pub summonerName: String, 	
    pub summonerId: String, 	                            //Player's summonerId (Encrypted)
    pub platformId: String, 	                            //Player's original platformId. 
}

#[derive(Clone,Debug, Deserialize)]
pub struct TeamStatsDto{
    pub towerKills: u64,                           	    //Number of towers the team destroyed.
    pub riftHeraldKills: u64,      	                    //Number of times the team killed Rift Herald.
    pub firstBlood: bool, 	                                //Flag indicating whether or not the team scored the first blood.
    pub inhibitorKills: u64, 	                            //Number of inhibitors the team destroyed.
    pub bans: Vec<TeamBansDto>, 	                        //If match queueId has a draft, contains banned champion data, otherwise empty.
    pub firstBaron: bool, 	                                //Flag indicating whether or not the team scored the first Baron kill.
    pub firstDragon: bool, 	                                //Flag indicating whether or not the team scored the first Dragon kill.
    pub dominionVictoryScore: u64, 	                    //For Dominion matches, specifies the points the team had at game end.
    pub dragonKills: u64, 	                            //Number of times the team killed Dragon.
    pub baronKills: u64, 	                                //Number of times the team killed Baron.
    pub firstInhibitor: bool, 	                            //Flag indicating whether or not the team destroyed the first inhibitor.
    pub firstTower: bool, 	                                //Flag indicating whether or not the team destroyed the first tower.
    pub vilemawKills: u64, 	                            //Number of times the team killed Vilemaw.
    pub firstRiftHerald: bool, 	                            //Flag indicating whether or not the team scored the first Rift Herald kill.
    pub teamId: u8, 	                                    //100 for blue side. 200 for red side.
    pub win: Option<String>,     	                                //String indicating whether or not the team won. There are only two values visibile in public match history. (Legal values: Fail, Win) 
}
impl TeamStatsDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "towerKills": self.towerKills,
            "riftHeraldKills": self.riftHeraldKills,
            "firstBlood": self.firstBlood,
            "inhibitorKills": self.inhibitorKills,
            "bans": self.get_bans_bson(),
            "firstBaron": self.firstBaron,
            "firstDragon": self.firstDragon,
            "dominionVictoryScore": self.dominionVictoryScore,
            "dragonKills": self.dragonKills,
            "baronKills": self.baronKills,
            "firstInhibitor": self.firstInhibitor,
            "firstTower": self.firstTower,
            "vilemawKills": self.vilemawKills,
            "firstRiftHerald": self.firstRiftHerald,
            "teamId": self.teamId as u64,
            "win":  match &self.win{
                Some(win)=> Bson::String(win.to_string()),
                None=> Bson::Null,
            },
        })
    }

    pub fn get_bans_bson(&self)-> Vec<Bson>{
        let mut bans_bson = Vec::new();
        for ban in &self.bans{
            bans_bson.push(ban.to_bson());
        }
        bans_bson
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct TeamBansDto{
    pub championId: i64, 	                                        //Banned championId.
    pub pickTurn: u64,  	                                        //Turn during which the champion was banned. 
}
impl TeamBansDto{
    pub fn to_bson(&self)->Bson{
        bson!({
            "championId": self.championId,
            "pickTurn": self.pickTurn,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct ParticipantDto{
    pub participantId: u8, 	
    pub championId: u64, 	
    pub runes: Option<Vec<RuneDto>>, 	                                    //List of legacy Rune information. Not included for matches played with Runes Reforged.
    pub stats: ParticipantStatsDto, 	                            //Participant statistics.
    pub teamId: u8, 	                                            //100 for blue side. 200 for red side.
    pub timeline: ParticipantTimelineDto, 	                        //Participant timeline data.
    pub spell1Id: u64, 	                                        //First Summoner Spell id.
    pub spell2Id: u64, 	                                        //Second Summoner Spell id.
    pub highestAchievedSeasonTier: Option<String>,    	                    //Highest ranked tier achieved for the previous season in a specific subset of queueIds, if any, otherwise null. Used to display border in game loading screen. Please refer to the Ranked Info documentation. (Legal values: CHALLENGER, MASTER, DIAMOND, PLATINUM, GOLD, SILVER, BRONZE, UNRANKED)
    pub masteries: Option<Vec<MasteryDto>>, 	                            //List of legacy Mastery information. Not included for matches played with Runes Reforged. 
}

impl ParticipantDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "participantId": self.participantId as u64,
            "championId": self.championId,
            "runes": self.get_runes_bson(),
            "stats": self.stats.to_bson(),
            "teamId": self.teamId as u64,
            "timeline": self.timeline.to_bson(),
            "spell1Id": self.spell1Id,
            "spell2Id": self.spell2Id,
            "highestAchievedSeasonTier": match &self.highestAchievedSeasonTier{
                Some(highestAchievedSeasonTier)=> Bson::String(highestAchievedSeasonTier.to_string()),
                None=> Bson::Null,
            },
            "masteries": self.get_masteries_bson(),
          })
    }

    pub fn get_runes_bson(&self)-> Vec<Bson>{
        let mut runes_bson = Vec::new();
        match &self.runes{
            None => (),
            Some(runes)=> {
                for rune in runes{
                    runes_bson.push(rune.to_bson());
                }
            },
        }
        
        runes_bson            
    }

    pub fn get_masteries_bson(&self)-> Vec<Bson>{
        let mut masteries_bson = Vec::new();
        match &self.masteries{
            None => (),
            Some(masteries)=> {
                for mastery in masteries{
                    masteries_bson.push(mastery.to_bson());
                }
            },
        }
        
        masteries_bson 
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct RuneDto{
    pub runeId: u64,
    pub rank: u64,
}
impl RuneDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "runeId": self.runeId,
            "rank": self.rank,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct ParticipantStatsDto{
    pub item0: u64,
    pub item2: u64,
    pub totalUnitsHealed: u64,
    pub item1: u64,
    pub largestMultiKill: u64,
    pub goldEarned: u64,
    pub firstInhibitorKill: Option<bool>,
    pub physicalDamageTaken: f64,
    pub nodeNeutralizeAssist: Option<i64>,
    pub totalPlayerScore: u64,
    pub champLevel: u64,
    pub damageDealtToObjectives: f64,
    pub totalDamageTaken: f64,
    pub neutralMinionsKilled: u64,
    pub deaths: u64,
    pub tripleKills: u64,
    pub magicDamageDealtToChampions: f64,
    pub wardsKilled: u64,
    pub pentaKills: u64,
    pub damageSelfMitigated: f64,
    pub largestCriticalStrike: u64,
    pub nodeNeutralize: Option<i64>,
    pub totalTimeCrowdControlDealt: u64,
    pub firstTowerKill: Option<bool>,
    pub magicDamageDealt: f64,
    pub totalScoreRank: u64,
    pub nodeCapture: Option<i64>,
    pub wardsPlaced: u64,
    pub totalDamageDealt: f64,
    pub timeCCingOthers: f64,
    pub magicalDamageTaken: f64,
    pub largestKillingSpree: f64,
    pub totalDamageDealtToChampions: f64,
    pub physicalDamageDealtToChampions: f64,
    pub neutralMinionsKilledTeamJungle: u64,
    pub totalMinionsKilled: u64,
    pub firstInhibitorAssist: Option<bool>,
    pub visionWardsBoughtInGame: u64,
    pub objectivePlayerScore: u64,
    pub kills: u64,
    pub firstTowerAssist: Option<bool>,
    pub combatPlayerScore: u64,
    pub inhibitorKills: u64,
    pub turretKills: u64,
    pub participantId: u64,
    pub trueDamageTaken: f64,
    pub firstBloodAssist: Option<bool>,
    pub nodeCaptureAssist: Option<i64>,
    pub assists: u64,
    pub teamObjective: Option<i64>,
    pub altarsNeutralized: Option<i64>,
    pub goldSpent: u64,
    pub damageDealtToTurrets: f64,
    pub altarsCaptured: Option<i64>,
    pub win: bool,
    pub totalHeal: f64,
    pub unrealKills: u64,
    pub visionScore: f64,
    pub physicalDamageDealt: f64,
    pub firstBloodKill: Option<bool>,
    pub longestTimeSpentLiving: u64,
    pub killingSprees: u64,
    pub sightWardsBoughtInGame: u64,
    pub trueDamageDealtToChampions: f64,
    pub neutralMinionsKilledEnemyJungle: u64,
    pub doubleKills: u64,
    pub trueDamageDealt: f64,
    pub quadraKills: u64,
    pub item4: u64,
    pub item3: u64,
    pub item6: u64,
    pub item5: u64,
    pub playerScore0: u64,
    pub playerScore1: u64,
    pub playerScore2: u64,
    pub playerScore3: u64,
    pub playerScore4: u64,
    pub playerScore5: u64,
    pub playerScore6: u64,
    pub playerScore7: u64,
    pub playerScore8: u64,
    pub playerScore9: u64,
    pub perk0: Option<i64>, 	                                        //Primary path keystone rune.
    pub perk0Var1: Option<i64>,            	                        //Post game rune stats.
    pub perk0Var2: Option<i64>, 	                                    //Post game rune stats.
    pub perk0Var3: Option<i64>, 	                                    //Post game rune stats.
    pub perk1: Option<i64>, 	                                        //Primary path rune.
    pub perk1Var1: Option<i64>, 	                                    //Post game rune stats.
    pub perk1Var2: Option<i64>, 	                                    //Post game rune stats.
    pub perk1Var3: Option<i64>, 	                                    //Post game rune stats.
    pub perk2: Option<i64>, 	                                        //Primary path rune.
    pub perk2Var1: Option<i64>, 	                                    //Post game rune stats.
    pub perk2Var2: Option<i64>, 	                                    //Post game rune stats.
    pub perk2Var3: Option<i64>, 	                                    //Post game rune stats.
    pub perk3: Option<i64>, 	                                        //Primary path rune.
    pub perk3Var1: Option<i64>, 	                                    //Post game rune stats.
    pub perk3Var2: Option<i64>, 	                                    //Post game rune stats.
    pub perk3Var3: Option<i64>, 	                                    //Post game rune stats.
    pub perk4: Option<i64>, 	                                        //Secondary path rune.
    pub perk4Var1: Option<i64>, 	                                    //Post game rune stats.
    pub perk4Var2: Option<i64>, 	                                    //Post game rune stats.
    pub perk4Var3: Option<i64>, 	                                    //Post game rune stats.
    pub perk5: Option<i64>, 	                                        //Secondary path rune.
    pub perk5Var1: Option<i64>, 	                                    //Post game rune stats.
    pub perk5Var2: Option<i64>, 	                                    //Post game rune stats.
    pub perk5Var3: Option<i64>, 	                                    //Post game rune stats.
    pub perkPrimaryStyle: Option<i64>, 	                            //Primary rune path
    pub perkSubStyle: Option<i64>, 	                                //Secondary rune path
    pub statPerk0: Option<i64>, 	                                    //Stat rune
    pub statPerk1: Option<i64>, 	                                    //Stat rune
    pub statPerk2: Option<i64>, 	                                    //Stat rune 
}

impl ParticipantStatsDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "item0": self.item0,
            "item2": self.item2,
            "totalUnitsHealed": self.totalUnitsHealed,
            "item1": self.item1,
            "largestMultiKill": self.largestMultiKill,
            "goldEarned": self.goldEarned,
            "firstInhibitorKill": match &self.firstInhibitorKill{
                Some(firstInhibitorKill)=> Bson::Boolean(*firstInhibitorKill),
                None=> Bson::Null,
            },
            "physicalDamageTaken": self.physicalDamageTaken,
            "nodeNeutralizeAssist": match &self.nodeNeutralizeAssist{
                Some(nodeNeutralizeAssist)=> Bson::Int64(*nodeNeutralizeAssist),
                None=> Bson::Null,
            },
            "totalPlayerScore": self.totalPlayerScore,
            "champLevel": self.champLevel,
            "damageDealtToObjectives": self.damageDealtToObjectives,
            "totalDamageTaken": self.totalDamageTaken,
            "neutralMinionsKilled": self.neutralMinionsKilled,
            "deaths": self.deaths,
            "tripleKills": self.tripleKills,
            "magicDamageDealtToChampions": self.magicDamageDealtToChampions,
            "wardsKilled": self.wardsKilled,
            "pentaKills": self.pentaKills,
            "damageSelfMitigated": self.damageSelfMitigated,
            "largestCriticalStrike": self.largestCriticalStrike,
            "nodeNeutralize": match &self.nodeNeutralize{
                Some(nodeNeutralize)=> Bson::Int64(*nodeNeutralize),
                None=> Bson::Null,
            },
            "totalTimeCrowdControlDealt": self.totalTimeCrowdControlDealt,
            "firstTowerKill": match &self.firstTowerKill{
                Some(firstTowerKill)=> Bson::Boolean(*firstTowerKill),
                None=> Bson::Null,
            },
            "magicDamageDealt": self.magicDamageDealt,
            "totalScoreRank": self.totalScoreRank,
            "nodeCapture": match &self.nodeCapture{
                Some(nodeCapture)=> Bson::Int64(*nodeCapture),
                None=> Bson::Null,
            },
            "wardsPlaced": self.wardsPlaced,
            "totalDamageDealt": self.totalDamageDealt,
            "timeCCingOthers": self.timeCCingOthers,
            "magicalDamageTaken": self.magicalDamageTaken,
            "largestKillingSpree": self.largestKillingSpree,
            "totalDamageDealtToChampions": self.totalDamageDealtToChampions,
            "physicalDamageDealtToChampions": self.physicalDamageDealtToChampions,
            "neutralMinionsKilledTeamJungle": self.neutralMinionsKilledTeamJungle,
            "totalMinionsKilled": self.totalMinionsKilled,
            "firstInhibitorAssist": match &self.firstInhibitorKill{
                Some(firstInhibitorKill)=> Bson::Boolean(*firstInhibitorKill),
                None=> Bson::Null,
            },
            "visionWardsBoughtInGame": self.visionWardsBoughtInGame,
            "objectivePlayerScore": self.objectivePlayerScore,
            "kills": self.kills,
            "firstTowerAssist": match &self.firstTowerAssist{
                Some(firstTowerAssist)=> Bson::Boolean(*firstTowerAssist),
                None=> Bson::Null,
            },
            "combatPlayerScore": self.combatPlayerScore,
            "inhibitorKills": self.inhibitorKills,
            "turretKills": self.turretKills,
            "participantId": self.participantId,
            "trueDamageTaken": self.trueDamageTaken,
            "firstBloodAssist": match &self.firstBloodAssist{
                Some(firstBloodAssist)=> Bson::Boolean(*firstBloodAssist),
                None=> Bson::Null,
            },
            "nodeCaptureAssist": match &self.nodeCaptureAssist{
                Some(nodeCaptureAssist)=> Bson::Int64(*nodeCaptureAssist),
                None=> Bson::Null,
            },
            "assists": self.assists,
            "teamObjective": match &self.teamObjective{
                Some(teamObjective)=> Bson::Int64(*teamObjective),
                None=> Bson::Null,
            },
            "altarsNeutralized": match &self.altarsNeutralized{
                Some(altarsNeutralized)=> Bson::Int64(*altarsNeutralized),
                None=> Bson::Null,
            },
            "goldSpent": self.goldSpent,
            "damageDealtToTurrets": self.damageDealtToTurrets,
            "altarsCaptured": match &self.altarsCaptured{
                Some(altarsCaptured)=> Bson::Int64(*altarsCaptured),
                None=> Bson::Null,
            },
            "win": self.win,
            "totalHeal": self.totalHeal,
            "unrealKills": self.unrealKills,
            "visionScore": self.visionScore,
            "physicalDamageDealt": self.physicalDamageDealt,
            "firstBloodKill": match &self.firstBloodKill{
                Some(firstBloodKill)=> Bson::Boolean(*firstBloodKill),
                None=> Bson::Null,
            },
            "longestTimeSpentLiving": self.longestTimeSpentLiving,
            "killingSprees": self.killingSprees,
            "sightWardsBoughtInGame": self.sightWardsBoughtInGame,
            "trueDamageDealtToChampions": self.trueDamageDealtToChampions,
            "neutralMinionsKilledEnemyJungle": self.neutralMinionsKilledEnemyJungle,
            "doubleKills": self.doubleKills,
            "trueDamageDealt": self.trueDamageDealt,
            "quadraKills": self.quadraKills,
            "item4": self.item4,
            "item3": self.item3,
            "item6": self.item6,
            "item5": self.item5,
            "playerScore0": self.playerScore0,
            "playerScore1": self.playerScore1,
            "playerScore2": self.playerScore2,
            "playerScore3": self.playerScore3,
            "playerScore4": self.playerScore4,
            "playerScore5": self.playerScore5,
            "playerScore6": self.playerScore6,
            "playerScore7": self.playerScore7,
            "playerScore8": self.playerScore8,
            "playerScore9": self.playerScore9,
            "perk0": match &self.perk0{
                Some(perk0)=> Bson::Int64(*perk0),
                None=> Bson::Null,
            },	                                        //Primary path keystone rune.
            "perk0Var1": match &self.perk0Var1{
                Some(perk0Var1)=> Bson::Int64(*perk0Var1),
                None=> Bson::Null,
            },   	                        //Post game rune stats.
            "perk0Var2": match &self.perk0Var2{
                Some(perk0Var2)=> Bson::Int64(*perk0Var2),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk0Var3": match &self.perk0Var3{
                Some(perk0Var3)=> Bson::Int64(*perk0Var3),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk1": match &self.perk1{
                Some(perk1)=> Bson::Int64(*perk1),
                None=> Bson::Null,
            },	      	                                        //Primary path rune.
            "perk1Var1": match &self.perk1Var1{
                Some(perk1Var1)=> Bson::Int64(*perk1Var1),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk1Var2": match &self.perk1Var2{
                Some(perk1Var2)=> Bson::Int64(*perk1Var2),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk1Var3": match &self.perk1Var3{
                Some(perk1Var3)=> Bson::Int64(*perk1Var3),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk2": match &self.perk2{
                Some(perk2)=> Bson::Int64(*perk2),
                None=> Bson::Null,
            },	       	                                        //Primary path rune.
            "perk2Var1": match &self.perk2Var1{
                Some(perk2Var1)=> Bson::Int64(*perk2Var1),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk2Var2": match &self.perk2Var2{
                Some(perk2Var2)=> Bson::Int64(*perk2Var2),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk2Var3": match &self.perk2Var3{
                Some(perk2Var3)=> Bson::Int64(*perk2Var3),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk3": match &self.perk3{
                Some(perk3)=> Bson::Int64(*perk3),
                None=> Bson::Null,
            },	       	                                        //Primary path rune.
            "perk3Var1": match &self.perk3Var1{
                Some(perk3Var1)=> Bson::Int64(*perk3Var1),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk3Var2": match &self.perk3Var2{
                Some(perk3Var2)=> Bson::Int64(*perk3Var2),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk3Var3":match &self.perk3Var3{
                Some(perk3Var3)=> Bson::Int64(*perk3Var3),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk4": match &self.perk4{
                Some(perk4)=> Bson::Int64(*perk4),
                None=> Bson::Null,
            },	       	                                        //Secondary path rune.
            "perk4Var1": match &self.perk4Var1{
                Some(perk4Var1)=> Bson::Int64(*perk4Var1),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk4Var2": match &self.perk4Var2{
                Some(perk4Var2)=> Bson::Int64(*perk4Var2),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk4Var3": match &self.perk4Var3{
                Some(perk4Var3)=> Bson::Int64(*perk4Var3),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk5": match &self.perk5{
                Some(perk5)=> Bson::Int64(*perk5),
                None=> Bson::Null,
            },	      	                                        //Secondary path rune.
            "perk5Var1": match &self.perk5Var1{
                Some(perk5Var1)=> Bson::Int64(*perk5Var1),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk5Var2": match &self.perk5Var2{
                Some(perk5Var2)=> Bson::Int64(*perk5Var2),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perk5Var3": match &self.perk5Var3{
                Some(perk5Var3)=> Bson::Int64(*perk5Var3),
                None=> Bson::Null,
            }, 	                                    //Post game rune stats.
            "perkPrimaryStyle": match &self.perkPrimaryStyle{
                Some(perkPrimaryStyle)=> Bson::Int64(*perkPrimaryStyle),
                None=> Bson::Null,
            }, 	 	                            //Primary rune path
            "perkSubStyle": match &self.perkSubStyle{
                Some(perkSubStyle)=> Bson::Int64(*perkSubStyle),
                None=> Bson::Null,
            }, 	 	                                //Secondary rune path
            "statPerk0": match &self.statPerk0{
                Some(statPerk0)=> Bson::Int64(*statPerk0),
                None=> Bson::Null,
            }, 	 	                                    //Stat rune
            "statPerk1": match &self.statPerk1{
                Some(statPerk1)=> Bson::Int64(*statPerk1),
                None=> Bson::Null,
            }, 	 	                                    //Stat rune
            "statPerk2": match &self.statPerk2{
                Some(statPerk2)=> Bson::Int64(*statPerk2),
                None=> Bson::Null,
            }, 	 	                                    //Stat rune 
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct ParticipantTimelineDto{
    pub participantId: u64, 	
    pub csDiffPerMinDeltas: Option<HashMap<String,f64>>, 	            //Creep score difference versus the calculated lane opponent(s) for a specified period.
    pub damageTakenPerMinDeltas: Option<HashMap<String,f64>>, 	        //Damage taken for a specified period.
    pub role: String, 	                                        //Participant's calculated role. (Legal values: DUO, NONE, SOLO, DUO_CARRY, DUO_SUPPORT)
    pub damageTakenDiffPerMinDeltas: Option<HashMap<String,f64>>, 	    //Damage taken difference versus the calculated lane opponent(s) for a specified period.
    pub xpPerMinDeltas: Option<HashMap<String,f64>>, 	                //Experience change for a specified period.
    pub xpDiffPerMinDeltas: Option<HashMap<String,f64>>, 	            //Experience difference versus the calculated lane opponent(s) for a specified period.
    pub lane: String, 	                                        //Participant's calculated lane. MID and BOT are legacy values. (Legal values: MID, MIDDLE, TOP, JUNGLE, BOT, BOTTOM)
    pub creepsPerMinDeltas: Option<HashMap<String,f64>>, 	            //Creeps for a specified period.
    pub goldPerMinDeltas: Option<HashMap<String,f64>>, 	            //Gold for a specified period.
}

impl ParticipantTimelineDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "participantId": self.participantId, 	
            "csDiffPerMinDeltas": match &self.csDiffPerMinDeltas{
                Some(csDiffPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in csDiffPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
            "damageTakenPerMinDeltas": match &self.damageTakenPerMinDeltas{
                Some(damageTakenPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in damageTakenPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
            "role": self.role.to_string(),
            "damageTakenDiffPerMinDeltas":  match &self.damageTakenDiffPerMinDeltas{
                Some(damageTakenDiffPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in damageTakenDiffPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
            "xpPerMinDeltas":  match &self.xpPerMinDeltas{
                Some(xpPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in xpPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
            "xpDiffPerMinDeltas":  match &self.xpDiffPerMinDeltas{
                Some(xpDiffPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in xpDiffPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
            "lane": self.lane.to_string(),
            "creepsPerMinDeltas":  match &self.creepsPerMinDeltas{
                Some(creepsPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in creepsPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
            "goldPerMinDeltas":  match &self.goldPerMinDeltas{
                Some(goldPerMinDeltas) =>{
                    let mut bsonDoc: Document = Document::new();
                    for (key, value) in goldPerMinDeltas{
                        bsonDoc.insert(key, value);
                    }
                    Bson::Document(bsonDoc)
                },
                None => Bson::Null,
            },
        })
    }
}

pub struct TimedValue{

}

#[derive(Clone,Debug, Deserialize)]
pub struct MasteryDto{
    rank: u64,
    masteryId: u64,
}

impl MasteryDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "rank": self.rank,
            "masteryId": self.masteryId,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchTimelineDto{
    pub frames: Vec<MatchFrameDto>, 	
    pub frameInterval: i64, 	
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchFrameDto{
    pub participantFrames: HashMap<String, MatchParticipantFrameDto>,
    pub events: Vec<MatchEventDto>, 	
    pub timestamp: i64,
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchParticipantFrameDto{
    pub participantId: u8,
    minionsKilled: u64,
    teamScore: Option<u64>,
    dominionScore: Option<u64>,
    totalGold: u64,
    level: u64,
    xp: u64,
    currentGold: i64,
    position: Option<MatchPositionDto>,
    jungleMinionsKilled: u64,
}

impl MatchParticipantFrameDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "participantId": self.participantId as u64,
            "minionsKilled": self.minionsKilled,
            "teamScore": match &self.teamScore{
                Some(teamScore)=> Bson::Int64(*teamScore as i64),
                None=> Bson::Null,
            },
            "dominionScore": match &self.dominionScore{
                Some(dominionScore)=> Bson::Int64(*dominionScore as i64),
                None=> Bson::Null,
            },
            "totalGold": self.totalGold,
            "level": self.level,
            "xp": self.xp,
            "currentGold": self.currentGold,
            "position": match &self.position{
                Some(position)=> position.to_bson(),
                None=> Bson::Null,
            },
            "jungleMinionsKilled": self.jungleMinionsKilled,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchPositionDto{
    x: u64,
    y: u64,
}
impl MatchPositionDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "x": self.x,
            "y": self.y,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchEventDto{
    pub laneType: Option<String>,
    pub skillSlot: Option<u64>,
    pub ascendedType: Option<String>,
    pub creatorId: Option<u8>,
    pub afterId: Option<u64>,
    pub eventType: Option<String>,
    pub r#type: String, 	//(Legal values: CHAMPION_KILL, WARD_PLACED, WARD_KILL, BUILDING_KILL, ELITE_MONSTER_KILL, ITEM_PURCHASED, ITEM_SOLD, ITEM_DESTROYED, ITEM_UNDO, SKILL_LEVEL_UP, ASCENDED_EVENT, CAPTURE_POINT, PORO_KING_SUMMON)
    pub levelUpType: Option<String>,
    pub wardType: Option<String>,
    pub participantId: Option<u8>,
    pub towerType: Option<String>,
    pub itemId: Option<u64>,
    pub beforeId: Option<u64>,
    pub pointCaptured: Option<String>,
    pub monsterType: Option<String>,
    pub monsterSubType: Option<String>,
    pub teamId: Option<u8>,
    pub position: Option<MatchPositionDto>,
    pub killerId: Option<u8>,
    pub killerTeamId: Option<u8>,
    pub timestamp: i64,
    pub realTimestamp: Option<i64>,
    pub assistingParticipantIds: Option<Vec<u8>>, 	
    pub buildingType: Option<String>,
    pub victimId: Option<u8>,
}

impl MatchEventDto{
    pub fn to_bson(&self) -> Bson {
        bson!({
            "laneType": match &self.laneType{
                Some(laneType)=> Bson::String(laneType.to_string()),
                None=> Bson::Null,
            },
            "skillSlot": match &self.skillSlot{
                Some(skillSlot)=> Bson::Int64(*skillSlot as i64),
                None=> Bson::Null,
            },
            "ascendedType": match &self.ascendedType{
                Some(ascendedType)=> Bson::String(ascendedType.to_string()),
                None=> Bson::Null,
            },
            "creatorId": match &self.creatorId{
                Some(creatorId)=> Bson::Int64(*creatorId as i64),
                None=> Bson::Null,
            },
            "afterId": match &self.afterId{
                Some(afterId)=> Bson::Int64(*afterId as i64),
                None=> Bson::Null,
            },
            "eventType": match &self.eventType{
                Some(eventType)=> Bson::String(eventType.to_string()),
                None=> Bson::Null,
            },
            "type": &self.r#type.to_string(),
            "levelUpType": match &self.levelUpType{
                Some(levelUpType)=> Bson::String(levelUpType.to_string()),
                None=> Bson::Null,
            },
            "wardType": match &self.wardType{
                Some(wardType)=> Bson::String(wardType.to_string()),
                None=> Bson::Null,
            },
            "participantId": match &self.participantId{
                Some(participantId)=> Bson::Int64(*participantId as i64),
                None=> Bson::Null,
            },
            "towerType": match &self.towerType{
                Some(towerType)=> Bson::String(towerType.to_string()),
                None=> Bson::Null,
            },
            "itemId": match &self.itemId{
                Some(itemId)=> Bson::Int64(*itemId as i64),
                None=> Bson::Null,
            },
            "beforeId": match &self.beforeId{
                Some(beforeId)=> Bson::Int64(*beforeId as i64),
                None=> Bson::Null,
            },
            "pointCaptured": match &self.pointCaptured{
                Some(pointCaptured)=> Bson::String(pointCaptured.to_string()),
                None=> Bson::Null,
            },
            "monsterType": match &self.monsterType{
                Some(monsterType)=> Bson::String(monsterType.to_string()),
                None=> Bson::Null,
            },
            "monsterSubType": match &self.monsterSubType{
                Some(monsterSubType)=> Bson::String(monsterSubType.to_string()),
                None=> Bson::Null,
            },
            "teamId": match &self.teamId{
                Some(teamId)=> Bson::Int64(*teamId as i64),
                None=> Bson::Null,
            },
            "position": match &self.position{
                Some(position)=> position.to_bson(),
                None=> Bson::Null,
            },
            "killerId": match &self.killerId{
                Some(killerId)=> Bson::Int64(*killerId as i64),
                None=> Bson::Null,
            },
            "killerTeamId": match &self.killerTeamId{
                Some(killerTeamId)=> Bson::Int64(*killerTeamId as i64),
                None=> Bson::Null,
            },
            "timestamp": self.timestamp,
            "realTimestamp": match &self.realTimestamp{
                Some(realTimestamp)=> Bson::Int64(*realTimestamp as i64),
                None=> Bson::Null,
            },
            "assistingParticipantIds": match &self.assistingParticipantIds{
                Some(assistingParticipantIds)=> {
                    let mut assistingParticipantIds_bson = Vec::new();
                    for id in assistingParticipantIds{
                        assistingParticipantIds_bson.push(Bson::Int64(*id as i64));
                    }
                    Bson::Array(assistingParticipantIds_bson)
                },
                None=> Bson::Null,
            },
            "buildingType": match &self.buildingType{
                Some(buildingType)=> Bson::String(buildingType.to_string()),
                None=> Bson::Null,
            },
            "victimId": match &self.victimId{
                Some(victimId)=> Bson::Int64(*victimId as i64),
                None=> Bson::Null,
            },
        })
    }

    pub fn is_from_participant(&self, participant_id: u8, team_id: u8) -> bool{
        if self.creatorId.is_some() && self.creatorId.unwrap() == participant_id{
            return true
        }
        if self.participantId.is_some() && self.participantId.unwrap() == participant_id{
            return true
        }
        if self.teamId.is_some() && self.teamId.unwrap() == team_id{
            return true
        }
        if self.killerId.is_some() && self.killerId.unwrap() == participant_id{
            return true
        }
        if self.victimId.is_some() && self.victimId.unwrap() == participant_id{
            return true
        }
        if self.assistingParticipantIds.is_some(){
            for assistant in self.assistingParticipantIds.as_ref().unwrap(){
                if *assistant == participant_id{
                    return true;
                }
            }
        }
        false
    }
}


/*

*/