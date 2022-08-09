use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApexUser {
    pub global: ApexGlobal,
    pub realtime: ApexRealtime,
    #[serde(alias = "total")]
    pub stats: ApexStats,
}
#[derive(Deserialize)]
pub struct ApexGlobal {
    pub name: String,
    pub uid: i64,
    pub avatar: String,
    pub platform: String,
    pub level: i32,
    #[serde(alias = "levelPrestige")]
    pub level_prestige: i32,
    #[serde(alias = "toNextLevelPercent")]
    pub to_next_level_percent: i32,
    pub rank: ApexRank,
    pub arena: ApexRank,
    pub bans: ApexBans,
    pub battlepass: ApexBattlepass,
}

#[derive(Deserialize)]
pub struct ApexRealtime {
    #[serde(alias = "lobbyState")]
    pub lobby_state: String,
    #[serde(alias = "isOnline")]
    pub is_online: i32,
    #[serde(alias = "isInGame")]
    pub is_in_game: i32,
    #[serde(alias = "canJoin")]
    pub can_join: i32,
    #[serde(alias = "partyFull")]
    pub party_full: i32,
    #[serde(alias = "selectedLegend")]
    pub selected_legend: String,
    #[serde(alias = "currentState")]
    pub current_state: String,
}

#[derive(Deserialize)]
pub struct ApexBattlepass {
    pub level: String,
    pub history: ApexBattlepassHistory,
}

#[derive(Deserialize)]
pub struct ApexBattlepassHistory {
    pub season1: i32,
    pub season2: i32,
    pub season3: i32,
    pub season4: i32,
    pub season5: i32,
    pub season6: i32,
    pub season7: i32,
    pub season8: i32,
    pub season9: i32,
    pub season10: i32,
}

#[derive(Deserialize)]
pub struct ApexRank {
    #[serde(alias = "rankScore")]
    pub rank_score: i32,
    #[serde(alias = "rankName")]
    pub rank_name: String,
    #[serde(alias = "rankDiv")]
    pub rank_division: i32,
    #[serde(alias = "rankImg")]
    pub rank_img: String,
    #[serde(alias = "rankedSeason")]
    pub ranked_season: String,
}

#[derive(Deserialize)]
pub struct ApexBans {
    #[serde(alias = "isActive")]
    pub is_active: bool,
    #[serde(alias = "remainingSeconds")]
    pub remaining_seconds: i32,
    #[serde(alias = "last_banReason")]
    pub last_ban_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct ApexGame {
    pub uid: String,
    pub name: String,
    #[serde(alias = "legendPlayed")]
    pub legend_played: String,
    #[serde(alias = "gameMode")]
    pub game_mode: String,
    #[serde(alias = "gameLengthSecs")]
    pub game_length_seconds: i32,
    #[serde(alias = "gameEndTimestamp")]
    pub game_end_timestamp: i32,
    #[serde(alias = "gameData")]
    pub game_data: Vec<ApexGameData>,
    #[serde(alias = "estimatedLevelProgress")]
    pub estimated_level_progress: i32,
    #[serde(alias = "BRScoreChange")]
    pub br_score_change: i32,
    #[serde(alias = "BRScore")]
    pub br_score: i32,
    #[serde(alias = "ArenasScoreChange")]
    pub arenas_score_change: i32,
    #[serde(alias = "ArenasScore")]
    pub arenas_score: i32,
    pub cosmetics: ApexCosmetics,
}

#[derive(Deserialize, Debug)]
pub struct ApexCosmetics {
    pub pose: String,
    pub skin: String,
    pub frame: String,
    pub intro: String,
    #[serde(alias = "poseRarity")]
    pub pose_rarity: String,
    #[serde(alias = "skinRarity")]
    pub skin_rarity: String,
    #[serde(alias = "frameRarity")]
    pub frame_rarity: String,
    #[serde(alias = "introRarity")]
    pub intro_rarity: String,
}

#[derive(Deserialize, Debug)]
pub struct ApexGameData {
    pub key: String,
    pub value: i32,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApexProfile {
    pub name: String,
    pub uid: String,
    pub pid: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug)]
pub struct ApexMapRotation {
    pub battle_royale: ApexMapRotationData,
    pub arenas: ApexMapRotationData,
    pub ranked: ApexRankedMapRotationData,
    #[serde(alias = "arenasRanked")]
    pub arenas_ranked: ApexMapRotationData,
}

#[derive(Deserialize, Debug)]
pub struct ApexMapRotationData {
    pub current: ApexMapRotationItem,
    pub next: ApexMapRotationItem,
}

#[derive(Deserialize, Debug)]
pub struct ApexRankedMapRotationData {
    pub current: ApexRankedMapRotationItem,
    pub next: ApexRankedMapRotationItem,
}

#[derive(Deserialize, Debug)]
pub struct ApexRankedMapRotationItem {
    pub map: String,
}

#[derive(Deserialize, Debug)]
pub struct ApexMapRotationItem {
    pub start: i64,
    pub end: i64,
    #[serde(alias = "readableDate_start")]
    pub readable_date_start: String,
    #[serde(alias = "readableDate_end")]
    pub readable_date_end: String,
    pub map: String,
    #[serde(alias = "DurationInSecs")]
    pub duration_in_seconds: i32,
    #[serde(alias = "DurationInMinutes")]
    pub duration_in_minutes: i32,
}

#[derive(Deserialize, Debug)]
pub struct Stat<V> {
    pub name: String,
    pub value: V,
}

#[derive(Deserialize, Debug)]
pub struct ApexStats {
    #[serde(alias = "kills")]
    pub br_kills: Stat<i32>,
    #[serde(alias = "damage")]
    pub br_damage: Stat<i32>,
    pub arenas_damage: Stat<i32>,
    pub games_played: Stat<i32>,
    pub kd: Stat<String>,
}
