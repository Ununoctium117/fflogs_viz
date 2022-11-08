use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CombatantStats {
    pub attack: i64,
    #[serde(rename = "attackMagicPotency")]
    pub attack_magic_potency: i64,
    #[serde(rename = "criticalHit")]
    pub critical_hit: i64,
    pub determination: i64,
    pub dexterity: i64,
    #[serde(rename = "directHit")]
    pub direct_hit: i64,
    #[serde(rename = "healMagicPotency")]
    pub heal_magic_potency: i64,
    pub intelligence: i64,
    pub level: i64,
    pub mind: i64,
    pub piety: i64,
    #[serde(rename = "skillSpeed")]
    pub skill_speed: i64,
    #[serde(rename = "spellSpeed")]
    pub spell_speed: i64,
    pub strength: i64,
    pub tenacity: i64,
    pub vitality: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    pub absorb: Option<i64>,
    pub facing: i64, // to get radians: divide by 1000, multiply by pi
    #[serde(rename = "hitPoints")]
    pub hit_points: i64,
    #[serde(rename = "maxHitPoints")]
    pub max_hit_points: i64,
    pub mp: i64,
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Aura {
    pub ability: i64,
    pub name: String,
    pub source: i64,
    pub stacks: i64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SourceInfo {
    #[serde(rename = "sourceID")]
    id: i64,
    #[serde(rename = "sourceMarker")]
    marker: Option<i64>,
    #[serde(rename = "sourceInstance")]
    instance: Option<i64>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TargetInfo {
    #[serde(rename = "targetID")]
    id: i64,
    #[serde(rename = "targetMarker")]
    marker: Option<i64>,
    #[serde(rename = "targetInstance")]
    instance: Option<i64>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "absorbed")]
    Absorbed {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,
        #[serde(rename = "extraAbilityGameID")]
        extra_ability_game_id: Option<i64>,

        amount: i64,

        #[serde(rename = "attackerID")]
        attacker_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "applybuff")]
    ApplyBuff {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,
        #[serde(rename = "extraAbilityGameID")]
        extra_ability_game_id: Option<i64>,

        duration: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "applybuffstack")]
    ApplyBuffStack {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        stack: i64,

        timestamp: i64,
    },

    #[serde(rename = "applydebuff")]
    ApplyDebuff {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,
        #[serde(rename = "extraAbilityGameID")]
        extra_ability_game_id: Option<i64>,

        duration: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "begincast")]
    BeginCast {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        duration: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "calculateddamage")]
    CalculatedDamage {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        amount: i64,
        #[serde(rename = "unmitigatedAmount")]
        unmitigated_amount: Option<i64>,
        multiplier: f64,

        #[serde(rename = "directHit")]
        direct_hit: Option<bool>,
        #[serde(rename = "hitType")]
        hit_type: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "calculatedheal")]
    CalculatedHeal {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        amount: i64,
        multiplier: f64,

        #[serde(rename = "hitType")]
        hit_type: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "cast")]
    Cast {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "combatantinfo")]
    CombatantInfo {
        #[serde(flatten)]
        stats: Option<CombatantStats>,
        auras: Vec<Aura>,

        #[serde(flatten)]
        source: SourceInfo,

        timestamp: i64,
    },

    #[serde(rename = "damage")]
    Damage {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        amount: i64,
        #[serde(rename = "unmitigatedAmount")]
        unmitigated_amount: Option<i64>,
        multiplier: Option<f64>,

        #[serde(rename = "directHit")]
        direct_hit: Option<bool>,
        #[serde(rename = "hitType")]
        hit_type: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "death")]
    Death {
        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "encounterend")]
    EncounterEnd { kill: bool, timestamp: i64 },

    #[serde(rename = "gaugeupdate")]
    GaugeUpdate {
        // there are other fields, but this is only present on the logging player, so we don't care
        timestamp: i64,
    },

    #[serde(rename = "headmarker")]
    HeadMarker {
        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "heal")]
    Heal {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        amount: i64,
        multiplier: Option<f64>,

        #[serde(rename = "hitType")]
        hit_type: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        #[serde(rename = "sourceResources")]
        source_resources: Option<Resources>,
        #[serde(rename = "targetResources")]
        target_resources: Option<Resources>,

        timestamp: i64,
    },

    #[serde(rename = "limitbreakupdate")]
    LimitBreakUpdate {
        bars: i64,
        value: i64,

        timestamp: i64,
    },

    #[serde(rename = "refreshbuff")]
    RefreshBuff {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        duration: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "refreshdebuff")]
    RefreshDebuff {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,
        #[serde(rename = "extraAbilityGameID")]
        extra_ability_game_id: Option<i64>,

        duration: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "removebuff")]
    RemoveBuff {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "removebuffstack")]
    RemoveBuffStack {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        stack: i64,

        timestamp: i64,
    },

    #[serde(rename = "removedebuff")]
    RemoveDebuff {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "targetabilityupdate")]
    TargetabilityUpdate {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        targetable: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },

    #[serde(rename = "tether")]
    Tether {
        #[serde(rename = "abilityGameID")]
        ability_game_id: i64,

        #[serde(flatten)]
        source: SourceInfo,
        #[serde(flatten)]
        target: TargetInfo,

        timestamp: i64,
    },
}
impl Event {
    pub fn get_timestamp(&self) -> i64 {
        match self {
            Event::Absorbed { timestamp, .. } => *timestamp,
            Event::CalculatedDamage { timestamp, .. } => *timestamp,
            Event::CalculatedHeal { timestamp, .. } => *timestamp,
            Event::Cast { timestamp, .. } => *timestamp,
            Event::CombatantInfo { timestamp, .. } => *timestamp,
            Event::RefreshBuff { timestamp, .. } => *timestamp,
            Event::ApplyBuff { timestamp, .. } => *timestamp,
            Event::ApplyBuffStack { timestamp, .. } => *timestamp,
            Event::LimitBreakUpdate { timestamp, .. } => *timestamp,
            Event::RemoveBuff { timestamp, .. } => *timestamp,
            Event::Damage { timestamp, .. } => *timestamp,
            Event::Heal { timestamp, .. } => *timestamp,
            Event::ApplyDebuff { timestamp, .. } => *timestamp,
            Event::BeginCast { timestamp, .. } => *timestamp,
            Event::RemoveBuffStack { timestamp, .. } => *timestamp,
            Event::GaugeUpdate { timestamp } => *timestamp,
            Event::RemoveDebuff { timestamp, .. } => *timestamp,
            Event::RefreshDebuff { timestamp, .. } => *timestamp,
            Event::TargetabilityUpdate { timestamp, .. } => *timestamp,
            Event::EncounterEnd { timestamp, .. } => *timestamp,
            Event::Death { timestamp, .. } => *timestamp,
            Event::HeadMarker { timestamp, .. } => *timestamp,
            Event::Tether { timestamp, .. } => *timestamp,
        }
    }

    pub fn get_source_resources(&self) -> Option<(i64, &Resources)> {
        match self {
            Event::Absorbed { .. } => None,
            Event::ApplyBuff { .. } => None,
            Event::ApplyBuffStack { .. } => None,
            Event::ApplyDebuff { .. } => None,
            Event::BeginCast { .. } => None,
            Event::CalculatedDamage {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::CalculatedHeal {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::Cast {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::CombatantInfo { .. } => None,
            Event::Damage {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::Death {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::EncounterEnd { .. } => None,
            Event::GaugeUpdate { .. } => None,
            Event::Heal {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::HeadMarker {
                source,
                source_resources,
                ..
            } => source_resources.as_ref().map(|r| (source.id, r)),
            Event::LimitBreakUpdate { .. } => None,
            Event::RefreshBuff { .. } => None,
            Event::RefreshDebuff { .. } => None,
            Event::RemoveBuff { .. } => None,
            Event::RemoveBuffStack { .. } => None,
            Event::RemoveDebuff { .. } => None,
            Event::TargetabilityUpdate { .. } => None,
            Event::Tether { .. } => None,
        }
    }

    pub fn get_target_resources(&self) -> Option<(i64, &Resources)> {
        match self {
            Event::Absorbed {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::ApplyBuff { .. } => None,
            Event::ApplyBuffStack { .. } => None,
            Event::ApplyDebuff { .. } => None,
            Event::BeginCast { .. } => None,
            Event::CalculatedDamage {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::CalculatedHeal {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::Cast {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::CombatantInfo { .. } => None,
            Event::Damage {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::Death {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::EncounterEnd { .. } => None,
            Event::GaugeUpdate { .. } => None,
            Event::Heal {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::HeadMarker {
                target,
                target_resources,
                ..
            } => target_resources.as_ref().map(|r| (target.id, r)),
            Event::LimitBreakUpdate { .. } => None,
            Event::RefreshBuff { .. } => None,
            Event::RefreshDebuff { .. } => None,
            Event::RemoveBuff { .. } => None,
            Event::RemoveBuffStack { .. } => None,
            Event::RemoveDebuff { .. } => None,
            Event::TargetabilityUpdate { .. } => None,
            Event::Tether { .. } => None,
        }
    }
}
