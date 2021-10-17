use serde::Deserialize;
use std::collections::HashMap;
use mongodb::bson::{bson, Document, Bson};

#[derive(Debug, Deserialize)]
pub struct ChampionList{
    pub key: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MatchDto{
    pub metadata: MetadataDto,
    pub info: InfoDto,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MetadataDto{
    pub dataVersion: String,                //Match data version.
    pub matchId: String,     	            //Match id
    pub participants: Vec<String>,          //A list of participant PUUIDs. 
}

#[derive(Clone, Debug, Deserialize)]
pub struct InfoDto{
    pub gameCreation: i64,                  //Unix timestamp for when the game is created on the game server (i.e., the loading screen).
    pub gameDuration: i64,                  //Prior to patch 11.20, this field returns the game length in milliseconds calculated from gameEndTimestamp - gameStartTimestamp. Post patch 11.20, this field returns the max timePlayed of any participant in the game in seconds, which makes the behavior of this field consistent with that of match-v4. The best way to handling the change in this field is to treat the value as milliseconds if the gameEndTimestamp field isn't in the response and to treat the value as seconds if gameEndTimestamp is in the response.
    pub gameEndTimestamp: Option<i64>,              //Unix timestamp for when match ends on the game server. This timestamp can occasionally be significantly longer than when the match "ends". The most reliable way of determining the timestamp for the end of the match would be to add the max time played of any participant to the gameStartTimestamp. This field was added to match-v5 in patch 11.20 on Oct 5th, 2021.
    pub gameId: f64,
    pub gameMode: String,                   //Refer to the Game Constants documentation.
    pub gameName: String,
    pub gameStartTimestamp: i64,            //Unix timestamp for when match starts on the game server.
    pub gameType: String,
    pub gameVersion: String,                //The first two parts can be used to determine the patch a game was played on.
    pub mapId: i64,                         //Refer to the Game Constants documentation.
    pub participants: Vec<ParticipantDto>,
    pub platformId: String,                 //Platform where the match was played.
    pub queueId: i64,                       //Refer to the Game Constants documentation.
    pub teams: Vec<TeamDto>, 	
    pub tournamentCode: Option<String>,     //Tournament code used to generate the match. This field was added to match-v5 in patch 11.13 on June 23rd, 2021. 
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamDto{
    pub bans: Vec<BanDto>,
    pub objectives: ObjectivesDto,
    pub teamId: i64,
    pub win: bool,
}
impl TeamDto{
    pub fn to_bson(&self)-> Bson{
        let mut bans = Vec::new();
        for ban in &self.bans{
            bans.push(ban.to_bson());
        }
        bson!({
            "bans": bans,
            "objectives": self.objectives.to_bson(),
            "teamId": self.teamId,
            "win": self.win,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BanDto{
    pub championId: i64,
    pub pickTurn: i64,
}
impl BanDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "championId": self.championId,
            "pickTurn": self.pickTurn,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ObjectivesDto{
    pub baron: ObjectiveDto,
    pub champion: ObjectiveDto,
    pub dragon: ObjectiveDto,
    pub inhibitor: ObjectiveDto,
    pub riftHerald: ObjectiveDto,
    pub tower: ObjectiveDto,
}
impl ObjectivesDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "baron": self.baron.to_bson(),
            "champion": self.champion.to_bson(),
            "dragon": self.dragon.to_bson(),
            "inhibitor": self.inhibitor.to_bson(),
            "riftHerald": self.riftHerald.to_bson(),
            "tower": self.tower.to_bson(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ObjectiveDto{
    pub first: bool,
    pub kills: i64,
}
impl ObjectiveDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "first": self.first,
            "kills": self.kills,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct ParticipantDto{
    pub assists: i64,
    pub baronKills: i64,
    pub bountyLevel: i64,
    pub champExperience: i64,
    pub champLevel: i64,
    pub championId: i64,                                //Prior to patch 11.4, on Feb 18th, 2021, this field returned invalid championIds. We recommend determining the champion based on the championName field for matches played prior to patch 11.4.
    pub championName: String,
    pub championTransform: i64,                         //This field is currently only utilized for Kayn's transformations. (Legal values: 0 - None, 1 - Slayer, 2 - Assassin)
    pub consumablesPurchased: i64,
    pub damageDealtToBuildings: i64,
    pub damageDealtToObjectives: i64,
    pub damageDealtToTurrets: i64,
    pub damageSelfMitigated: i64,
    pub deaths: i64,
    pub detectorWardsPlaced: i64,
    pub doubleKills: i64,
    pub dragonKills: i64,
    pub firstBloodAssist: bool,
    pub firstBloodKill: bool,
    pub firstTowerAssist: bool,
    pub firstTowerKill: bool,
    pub gameEndedInEarlySurrender: bool,
    pub gameEndedInSurrender: bool,
    pub goldEarned: i64,
    pub goldSpent: i64,
    pub individualPosition: String,                     //Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.
    pub inhibitorKills: i64,
    pub inhibitorTakedowns: Option<i64>,
    pub inhibitorsLost: i64,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub itemsPurchased: i64,
    pub killingSprees: i64,
    pub kills: i64,
    pub lane: String,
    pub largestCriticalStrike: i64,
    pub largestKillingSpree: i64,
    pub largestMultiKill: i64,
    pub longestTimeSpentLiving: i64,
    pub magicDamageDealt: i64,
    pub magicDamageDealtToChampions: i64,
    pub magicDamageTaken: i64,
    pub neutralMinionsKilled: i64,
    pub nexusKills: i64,
    pub nexusTakedowns: Option<i64>,
    pub nexusLost: i64,
    pub objectivesStolen: i64,
    pub objectivesStolenAssists: i64,
    pub participantId: i64,
    pub pentaKills: i64,
    pub perks: PerksDto,
    pub physicalDamageDealt: i64,
    pub physicalDamageDealtToChampions: i64,
    pub physicalDamageTaken: i64,
    pub profileIcon: i64,
    pub puuid: String,
    pub quadraKills: i64,
    pub riotIdName: String,
    pub riotIdTagline: String,
    pub role: String,
    pub sightWardsBoughtInGame: i64,
    pub spell1Casts: i64,
    pub spell2Casts: i64,
    pub spell3Casts: i64,
    pub spell4Casts: i64,
    pub summoner1Casts: i64,
    pub summoner1Id: i64,
    pub summoner2Casts: i64,
    pub summoner2Id: i64,
    pub summonerId: String,
    pub summonerLevel: i64,
    pub summonerName: String,
    pub teamEarlySurrendered: bool,
    pub teamId: i64,
    pub teamPosition: String,                           //Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.
    pub timeCCingOthers: i64,
    pub timePlayed: i64,
    pub totalDamageDealt: i64,
    pub totalDamageDealtToChampions: i64,
    pub totalDamageShieldedOnTeammates: i64,
    pub totalDamageTaken: i64,
    pub totalHeal: i64,
    pub totalHealsOnTeammates: i64,
    pub totalMinionsKilled: i64,
    pub totalTimeCCDealt: i64,
    pub totalTimeSpentDead: i64,
    pub totalUnitsHealed: i64,
    pub tripleKills: i64,
    pub trueDamageDealt: i64,
    pub trueDamageDealtToChampions: i64,
    pub trueDamageTaken: i64,
    pub turretKills: i64,
    pub turretTakedowns: Option<i64>,
    pub turretsLost: i64,
    pub unrealKills: i64,
    pub visionScore: i64,
    pub visionWardsBoughtInGame: i64,
    pub wardsKilled: i64,
    pub wardsPlaced: i64,
    pub win: bool,
}
impl ParticipantDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "assists": self.assists,
            "baronKills": self.baronKills,
            "bountyLevel": self.bountyLevel,
            "champExperience": self.champExperience,
            "champLevel": self.champLevel,
            "championId": self.championId,
            "championName": self.championName.as_str(),
            "championTransform": self.championTransform,
            "consumablesPurchased": self.consumablesPurchased,
            "damageDealtToBuildings": self.damageDealtToBuildings,
            "damageDealtToObjectives": self.damageDealtToObjectives,
            "damageDealtToTurrets": self.damageDealtToTurrets,
            "damageSelfMitigated": self.damageSelfMitigated,
            "deaths": self.deaths,
            "detectorWardsPlaced": self.detectorWardsPlaced,
            "doubleKills": self.doubleKills,
            "dragonKills": self.dragonKills,
            "firstBloodAssist": self.firstBloodAssist,
            "firstBloodKill": self.firstBloodKill,
            "firstTowerAssist": self.firstTowerAssist,
            "firstTowerKill": self.firstTowerKill,
            "gameEndedInEarlySurrender": self.gameEndedInEarlySurrender,
            "gameEndedInSurrender": self.gameEndedInSurrender,
            "goldEarned": self.goldEarned,
            "goldSpent": self.goldSpent,
            "individualPosition": self.individualPosition.as_str(),
            "inhibitorKills": self.inhibitorKills,
            "inhibitorTakedowns": match &self.inhibitorTakedowns{
                Some(inhibitorTakedowns)=> Bson::Int64(*inhibitorTakedowns as i64),
                None=> Bson::Null,
            },
            "inhibitorsLost": self.inhibitorsLost,
            "item0": self.item0,
            "item1": self.item1,
            "item2": self.item2,
            "item3": self.item3,
            "item4": self.item4,
            "item5": self.item5,
            "item6": self.item6,
            "itemsPurchased": self.itemsPurchased,
            "killingSprees": self.killingSprees,
            "kills": self.kills,
            "lane": self.lane.as_str(),
            "largestCriticalStrike": self.largestCriticalStrike,
            "largestKillingSpree": self.largestKillingSpree,
            "largestMultiKill": self.largestMultiKill,
            "longestTimeSpentLiving": self.longestTimeSpentLiving,
            "magicDamageDealt": self.magicDamageDealt,
            "magicDamageDealtToChampions": self.magicDamageDealtToChampions,
            "magicDamageTaken": self.magicDamageTaken,
            "neutralMinionsKilled": self.neutralMinionsKilled,
            "nexusKills": self.nexusKills,
            "nexusTakedowns": match &self.nexusTakedowns{
                Some(nexusTakedowns)=> Bson::Int64(*nexusTakedowns as i64),
                None=> Bson::Null,
            },
            "nexusLost": self.nexusLost,
            "objectivesStolen": self.objectivesStolen,
            "objectivesStolenAssists": self.objectivesStolenAssists,
            "participantId": self.participantId,
            "pentaKills": self.pentaKills,
            "perks": self.perks.to_bson(),
            "physicalDamageDealt": self.physicalDamageDealt,
            "physicalDamageDealtToChampions": self.physicalDamageDealtToChampions,
            "physicalDamageTaken": self.physicalDamageTaken,
            "profileIcon": self.profileIcon,
            "puuid": self.puuid.as_str(),
            "quadraKills": self.quadraKills,
            "riotIdName": self.riotIdName.as_str(),
            "riotIdTagline": self.riotIdTagline.as_str(),
            "role": self.role.as_str(),
            "sightWardsBoughtInGame": self.sightWardsBoughtInGame,
            "spell1Casts": self.spell1Casts,
            "spell2Casts": self.spell2Casts,
            "spell3Casts": self.spell3Casts,
            "spell4Casts": self.spell4Casts,
            "summoner1Casts": self.summoner1Casts,
            "summoner1Id": self.summoner1Id,
            "summoner2Casts": self.summoner2Casts,
            "summoner2Id": self.summoner2Id,
            "summonerId": self.summonerId.as_str(),
            "summonerLevel": self.summonerLevel,
            "summonerName": self.summonerName.as_str(),
            "teamEarlySurrendered": self.teamEarlySurrendered,
            "teamId": self.teamId,
            "teamPosition": self.teamPosition.as_str(),
            "timeCCingOthers": self.timeCCingOthers,
            "timePlayed": self.timePlayed,
            "totalDamageDealt": self.totalDamageDealt,
            "totalDamageDealtToChampions": self.totalDamageDealtToChampions,
            "totalDamageShieldedOnTeammates": self.totalDamageShieldedOnTeammates,
            "totalDamageTaken": self.totalDamageTaken,
            "totalHeal": self.totalHeal,
            "totalHealsOnTeammates": self.totalHealsOnTeammates,
            "totalMinionsKilled": self.totalMinionsKilled,
            "totalTimeCCDealt": self.totalTimeCCDealt,
            "totalTimeSpentDead": self.totalTimeSpentDead,
            "totalUnitsHealed": self.totalUnitsHealed,
            "tripleKills": self.tripleKills,
            "trueDamageDealt": self.trueDamageDealt,
            "trueDamageDealtToChampions": self.trueDamageDealtToChampions,
            "trueDamageTaken": self.trueDamageTaken,
            "turretKills": self.turretKills,
            "turretTakedowns": match &self.turretTakedowns{
                Some(turretTakedowns)=> Bson::Int64(*turretTakedowns as i64),
                None=> Bson::Null,
            },
            "turretsLost": self.turretsLost,
            "unrealKills": self.unrealKills,
            "visionScore": self.visionScore,
            "visionWardsBoughtInGame": self.visionWardsBoughtInGame,
            "wardsKilled": self.wardsKilled,
            "wardsPlaced": self.wardsPlaced,
            "win": self.win,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct PerksDto{
    pub statPerks: PerkStatsDto,
    pub styles: Vec<PerkStyleDto>,
}
impl PerksDto{
    pub fn to_bson(&self)-> Bson{
        let mut styles_bson = Vec::new();
        for style in &self.styles{
            styles_bson.push(style.to_bson());
        }
        bson!({
            "statPerks": self.statPerks.to_bson(),
            "styles": styles_bson,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct PerkStatsDto{
    pub defense: i64,
    pub flex: i64,
    pub offense: i64,
}
impl PerkStatsDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "defense": self.defense,
            "flex": self.flex,
            "offense": self.offense,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct PerkStyleDto{
    pub description: String,
    pub selections: Vec<PerkStyleSelectionDto>,
    pub style: i64, 
}
impl PerkStyleDto{
    pub fn to_bson(&self)-> Bson{
        let mut selections = Vec::new();
        for selection in &self.selections{
            selections.push(selection.to_bson());
        }
        bson!({
            "description": self.description.as_str(),
            "selections": selections,
            "style": self.style,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct PerkStyleSelectionDto{
    pub perk: i64,	
    pub var1: i64,
    pub var2: i64,
    pub var3: i64,
}
impl PerkStyleSelectionDto{
    pub fn to_bson(&self)-> Bson{
        bson!({
            "perk": self.perk,
            "var1": self.var1,
            "var2": self.var2,
            "var3": self.var3,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ChampionMasteryDTO {
    pub championPointsUntilNextLevel: f64, // 	Number of points needed to achieve next level. Zero if player reached maximum champion level for this champion.
    pub chestGranted: bool, // 	Is chest granted for this champion or not in current season.
    pub championId: isize, // 	Champion ID for this entry.
    pub lastPlayTime: f64, // 	Last time this champion was played by this player - in Unix milliseconds time format.
    pub championLevel: i64, // 	Champion level for specified player and champion combination.
    pub summonerId: String, // 	Summoner ID for this entry. (Encrypted)
    pub championPoints: i64, // 	Total number of champion points for this player and champion combination - they are used to determine championLevel.
    pub championPointsSinceLastLevel: f64, // 	Number of points earned since current level has been achieved.
    pub tokensEarned: i64, // 	The token earned for this champion to levelup. 
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchTimelineDto{
    pub metadata: MetadataDto,
    pub info: TimelineInfoDto,
}

#[derive(Clone,Debug, Deserialize)]
pub struct TimelineInfoDto{
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
    championStats: ChampionStatsDto,
    currentGold: i64,
    damageStats: DamageStatsDto,
    goldPerSecond: i64,
    jungleMinionsKilled: i64,
    level: i64,
    minionsKilled: i64,
    pub participantId: i64,
    position: Option<MatchPositionDto>,
    timeEnemySpentControlled: Option<i64>,
    totalGold: i64,
    xp: i64,
}

impl MatchParticipantFrameDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "championStats": self.championStats.to_bson(),
            "currentGold": self.currentGold,
            "damageStats": self.damageStats.to_bson(),
            "goldPerSecond": self.goldPerSecond,
            "jungleMinionsKilled": self.jungleMinionsKilled,
            "level": self.level,
            "minionsKilled": self.minionsKilled,
            "participantId": self.participantId as i64,
            "position": match &self.position{
                Some(position)=> position.to_bson(),
                None=> Bson::Null,
            },
            "timeEnemySpentControlled": match &self.timeEnemySpentControlled{
                Some(timeEnemySpentControlled)=> Bson::Int64(*timeEnemySpentControlled as i64),
                None=> Bson::Null,
            },
            "totalGold": self.totalGold,
            "xp": self.xp,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchPositionDto{
    x: i64,
    y: i64,
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
pub struct ChampionStatsDto{
    abilityPower: i64,
    armor: i64,
    armorPen: i64,
    armorPenPercent: i64,
    attackDamage: i64,
    attackSpeed: i64,
    bonusArmorPenPercent: i64,
    bonusMagicPenPercent: i64,
    ccReduction: i64,
    cooldownReduction: i64,
    health: i64,
    healthMax: i64,
    healthRegen: i64,
    lifesteal: i64,
    magicPen: i64,
    magicPenPercent: i64,
    magicResist: i64,
    movementSpeed: i64,
    power: i64,
    powerMax: i64,
    powerRegen: i64,
    spellVamp: i64,
}
impl ChampionStatsDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "abilityPower": self.abilityPower,
            "armor": self.armor,
            "armorPen": self.armorPen,
            "armorPenPercent": self.armorPenPercent,
            "attackDamage": self.attackDamage,
            "attackSpeed": self.attackSpeed,
            "bonusArmorPenPercent": self.bonusArmorPenPercent,
            "bonusMagicPenPercent": self.bonusMagicPenPercent,
            "ccReduction": self.ccReduction,
            "cooldownReduction": self.cooldownReduction,
            "health": self.health,
            "healthMax": self.healthMax,
            "healthRegen": self.healthRegen,
            "lifesteal": self.lifesteal,
            "magicPen": self.magicPen,
            "magicPenPercent": self.magicPenPercent,
            "magicResist": self.magicResist,
            "movementSpeed": self.movementSpeed,
            "power": self.power,
            "powerMax": self.powerMax,
            "powerRegen": self.powerRegen,
            "spellVamp": self.spellVamp,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct DamageStatsDto{
    magicDamageDone: i64,
    magicDamageDoneToChampions: i64,
    magicDamageTaken: i64,
    physicalDamageDone: i64,
    physicalDamageDoneToChampions: i64,
    physicalDamageTaken: i64,
    totalDamageDone: i64,
    totalDamageDoneToChampions: i64,
    totalDamageTaken: i64,
    trueDamageDone: i64,
    trueDamageDoneToChampions: i64,
    trueDamageTaken: i64,
}
impl DamageStatsDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "magicDamageDone": self.magicDamageDone,
            "magicDamageDoneToChampions": self.magicDamageDoneToChampions,
            "magicDamageTaken": self.magicDamageTaken,
            "physicalDamageDone": self.physicalDamageDone,
            "physicalDamageDoneToChampions": self.physicalDamageDoneToChampions,
            "physicalDamageTaken": self.physicalDamageTaken,
            "totalDamageDone": self.totalDamageDone,
            "totalDamageDoneToChampions": self.totalDamageDoneToChampions,
            "totalDamageTaken": self.totalDamageTaken,
            "trueDamageDone": self.trueDamageDone,
            "trueDamageDoneToChampions": self.trueDamageDoneToChampions,
            "trueDamageTaken": self.trueDamageTaken,
        })
    }
}

#[derive(Clone,Debug, Deserialize)]
pub struct MatchEventDto{
    pub laneType: Option<String>,
    pub skillSlot: Option<i64>,
    pub ascendedType: Option<String>,
    pub creatorId: Option<i64>,
    pub afterId: Option<i64>,
    pub eventType: Option<String>,
    pub r#type: String, 	//(Legal values: CHAMPION_KILL, WARD_PLACED, WARD_KILL, BUILDING_KILL, ELITE_MONSTER_KILL, ITEM_PURCHASED, ITEM_SOLD, ITEM_DESTROYED, ITEM_UNDO, SKILL_LEVEL_UP, ASCENDED_EVENT, CAPTURE_POINT, PORO_KING_SUMMON)
    pub levelUpType: Option<String>,
    pub wardType: Option<String>,
    pub participantId: Option<i64>,
    pub towerType: Option<String>,
    pub itemId: Option<i64>,
    pub beforeId: Option<i64>,
    pub pointCaptured: Option<String>,
    pub monsterType: Option<String>,
    pub monsterSubType: Option<String>,
    pub teamId: Option<i64>,
    pub position: Option<MatchPositionDto>,
    pub killerId: Option<i64>,
    pub killerTeamId: Option<i64>,
    pub timestamp: i64,
    pub realTimestamp: Option<i64>,
    pub assistingParticipantIds: Option<Vec<i64>>, 	
    pub buildingType: Option<String>,
    pub victimId: Option<i64>,
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

    pub fn is_from_participant(&self, participant_id: i64, team_id: i64) -> bool{
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

/*impl ParticipantDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "participantId": self.participantId as i64,
            "championId": self.championId,
            "runes": self.get_runes_bson(),
            "stats": self.stats.to_bson(),
            "teamId": self.teamId as i64,
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
pub struct MatchReferenceDto{
    pub gameId:  i64, 	
    pub role: String, 	
    pub season: i64, 	
    pub platformId: String,
    pub champion: i64, 	
    pub queue: i64,
    pub lane: String,	
    pub timestamp: i64, 
}

#[derive(Clone,Debug, Deserialize)]
pub struct ParticipantIdentityDto{
    pub participantId: 	i64,
    pub player:	PlayerDto,	                                //Player information not included in the response for custom matches. Custom matches are considered private unless a tournament code was used to create the match.
}

#[derive(Clone,Debug, Deserialize)]
pub struct PlayerDto{
    pub profileIcon: i64, 	
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
    pub towerKills: i64,                           	    //Number of towers the team destroyed.
    pub riftHeraldKills: i64,      	                    //Number of times the team killed Rift Herald.
    pub firstBlood: bool, 	                                //Flag indicating whether or not the team scored the first blood.
    pub inhibitorKills: i64, 	                            //Number of inhibitors the team destroyed.
    pub bans: Vec<TeamBansDto>, 	                        //If match queueId has a draft, contains banned champion data, otherwise empty.
    pub firstBaron: bool, 	                                //Flag indicating whether or not the team scored the first Baron kill.
    pub firstDragon: bool, 	                                //Flag indicating whether or not the team scored the first Dragon kill.
    pub dominionVictoryScore: i64, 	                    //For Dominion matches, specifies the points the team had at game end.
    pub dragonKills: i64, 	                            //Number of times the team killed Dragon.
    pub baronKills: i64, 	                                //Number of times the team killed Baron.
    pub firstInhibitor: bool, 	                            //Flag indicating whether or not the team destroyed the first inhibitor.
    pub firstTower: bool, 	                                //Flag indicating whether or not the team destroyed the first tower.
    pub vilemawKills: i64, 	                            //Number of times the team killed Vilemaw.
    pub firstRiftHerald: bool, 	                            //Flag indicating whether or not the team scored the first Rift Herald kill.
    pub teamId: i64, 	                                    //100 for blue side. 200 for red side.
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
            "teamId": self.teamId as i64,
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
    pub pickTurn: i64,  	                                        //Turn during which the champion was banned. 
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
pub struct RuneDto{
    pub runeId: i64,
    pub rank: i64,
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
    pub item0: i64,
    pub item2: i64,
    pub totalUnitsHealed: i64,
    pub item1: i64,
    pub largestMultiKill: i64,
    pub goldEarned: i64,
    pub firstInhibitorKill: Option<bool>,
    pub physicalDamageTaken: f64,
    pub nodeNeutralizeAssist: Option<i64>,
    pub totalPlayerScore: i64,
    pub champLevel: i64,
    pub damageDealtToObjectives: f64,
    pub totalDamageTaken: f64,
    pub neutralMinionsKilled: i64,
    pub deaths: i64,
    pub tripleKills: i64,
    pub magicDamageDealtToChampions: f64,
    pub wardsKilled: i64,
    pub pentaKills: i64,
    pub damageSelfMitigated: f64,
    pub largestCriticalStrike: i64,
    pub nodeNeutralize: Option<i64>,
    pub totalTimeCrowdControlDealt: i64,
    pub firstTowerKill: Option<bool>,
    pub magicDamageDealt: f64,
    pub totalScoreRank: i64,
    pub nodeCapture: Option<i64>,
    pub wardsPlaced: i64,
    pub totalDamageDealt: f64,
    pub timeCCingOthers: f64,
    pub magicalDamageTaken: f64,
    pub largestKillingSpree: f64,
    pub totalDamageDealtToChampions: f64,
    pub physicalDamageDealtToChampions: f64,
    pub neutralMinionsKilledTeamJungle: i64,
    pub totalMinionsKilled: i64,
    pub firstInhibitorAssist: Option<bool>,
    pub visionWardsBoughtInGame: i64,
    pub objectivePlayerScore: i64,
    pub kills: i64,
    pub firstTowerAssist: Option<bool>,
    pub combatPlayerScore: i64,
    pub inhibitorKills: i64,
    pub turretKills: i64,
    pub participantId: i64,
    pub trueDamageTaken: f64,
    pub firstBloodAssist: Option<bool>,
    pub nodeCaptureAssist: Option<i64>,
    pub assists: i64,
    pub teamObjective: Option<i64>,
    pub altarsNeutralized: Option<i64>,
    pub goldSpent: i64,
    pub damageDealtToTurrets: f64,
    pub altarsCaptured: Option<i64>,
    pub win: bool,
    pub totalHeal: f64,
    pub unrealKills: i64,
    pub visionScore: f64,
    pub physicalDamageDealt: f64,
    pub firstBloodKill: Option<bool>,
    pub longestTimeSpentLiving: i64,
    pub killingSprees: i64,
    pub sightWardsBoughtInGame: i64,
    pub trueDamageDealtToChampions: f64,
    pub neutralMinionsKilledEnemyJungle: i64,
    pub doubleKills: i64,
    pub trueDamageDealt: f64,
    pub quadraKills: i64,
    pub item4: i64,
    pub item3: i64,
    pub item6: i64,
    pub item5: i64,
    pub playerScore0: i64,
    pub playerScore1: i64,
    pub playerScore2: i64,
    pub playerScore3: i64,
    pub playerScore4: i64,
    pub playerScore5: i64,
    pub playerScore6: i64,
    pub playerScore7: i64,
    pub playerScore8: i64,
    pub playerScore9: i64,
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
    pub participantId: i64, 	
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
    rank: i64,
    masteryId: i64,
}

impl MasteryDto{
    pub fn to_bson(&self) -> Bson{
        bson!({
            "rank": self.rank,
            "masteryId": self.masteryId,
        })
    }
}
*/