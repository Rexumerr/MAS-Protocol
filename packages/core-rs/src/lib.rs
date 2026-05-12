use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use dashmap::DashMap;
use std::sync::Arc;

// --- THE ENTERPRISE RPG KERNEL ---

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    DataMining,      // Extraction of raw data
    LeadGen,         // Hunting for prospects
    Infrastructure,  // Building the systems
    Optimization,    // Improving performance
    Automation,      // Making things run themselves
    Security,        // Protecting the assets
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkillProgress {
    pub level: u8,
    pub xp: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessQuest {
    pub id: String,
    pub name: String,
    pub requirements: Vec<(Capability, u8)>,
    pub reward_xp: f64,
    pub status: QuestStatus,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum QuestStatus {
    Locked,
    Unlocked,
    InProgress,
    Completed,
}

// --- THE ULTRA BRAIN: MULTIVERSAL ORCHESTRATOR ---

#[wasm_bindgen]
pub struct UltraBrain {
    // Capability storage for the "Master" profile
    capabilities: Arc<DashMap<Capability, SkillProgress>>,
    // Universal state storage (Blackboard)
    universes: Arc<DashMap<String, String>>,
}

#[wasm_bindgen]
impl UltraBrain {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let capabilities = Arc::new(DashMap::new());
        // Initialize capabilities at Level 1
        capabilities.insert(Capability::DataMining, SkillProgress { level: 1, xp: 0.0 });
        capabilities.insert(Capability::Automation, SkillProgress { level: 1, xp: 0.0 });
        capabilities.insert(Capability::LeadGen, SkillProgress { level: 1, xp: 0.0 });
        capabilities.insert(Capability::Infrastructure, SkillProgress { level: 1, xp: 0.0 });
        capabilities.insert(Capability::Optimization, SkillProgress { level: 1, xp: 0.0 });
        capabilities.insert(Capability::Security, SkillProgress { level: 1, xp: 0.0 });

        Self {
            capabilities,
            universes: Arc::new(DashMap::new()),
        }
    }

    pub fn grant_xp(&self, capability: Capability, amount: f64) -> u8 {
        if let Some(mut skill) = self.capabilities.get_mut(&capability) {
            skill.xp += amount;
            let new_level = self.calculate_level(skill.xp);
            if new_level > skill.level {
                skill.level = new_level;
                return new_level; // Level Up!
            }
        }
        0
    }

    fn calculate_level(&self, xp: f64) -> u8 {
        // RS-Inspired XP Curve
        // Level 99 is reached at ~13M XP
        if xp <= 0.0 { return 1; }
        let level = (xp.sqrt() / 36.0) as u8 + 1; // Adjusted for "Serious" scaling
        if level > 99 { 99 } else { level }
    }

    pub fn get_level(&self, capability: Capability) -> u8 {
        self.capabilities.get(&capability).map(|s| s.level).unwrap_or(1)
    }

    pub fn create_universe(&self, id: String, name: String) {
        self.universes.insert(id, name);
    }
}

// --- THE ORACLE BRIDGE: REALITY SYNC ---

#[wasm_bindgen]
pub struct OracleBridge {
    api_key_hash: String,
}

#[wasm_bindgen]
impl OracleBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Self {
        // En un entorno real, aquí validaríamos o hashearíamos la clave
        Self { api_key_hash: api_key }
    }

    // Traduce eventos reales a XP
    pub fn ingest_event(&self, brain: &UltraBrain, event_type: String, value: f64) -> String {
        match event_type.as_str() {
            "ad_spend_optimized" => {
                let xp = value * 0.5;
                brain.grant_xp(Capability::Optimization, xp);
                format!("Oracle: Optimized ${} in ads. Gained {} XP in Optimization.", value, xp)
            },
            "new_lead_generated" => {
                let xp = 500.0;
                brain.grant_xp(Capability::LeadGen, xp);
                "Oracle: High-Ticket Lead detected. +500 XP in LeadGen.".into()
            },
            "automation_success" => {
                brain.grant_xp(Capability::Automation, 200.0);
                "Oracle: Workflow executed flawlessly. +200 XP in Automation.".into()
            },
            _ => "Oracle: Unknown event type.".into()
        }
    }
}
