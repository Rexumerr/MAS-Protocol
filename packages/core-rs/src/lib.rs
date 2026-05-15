use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use dashmap::DashMap;
use std::sync::Arc;
use std::fs;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::{RngCore, rng};

// --- THE KYBALION UNIVERSE: HERMETIC DICTIONARY ---

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HermeticPrinciple {
    Mentalism,
    Correspondence,
    Vibration,
    Polarity,
    Rhythm,
    CauseEffect,
    Gender,
}

impl HermeticPrinciple {
    pub fn get_axiom(self) -> &'static str {
        match self {
            HermeticPrinciple::Mentalism => "The All is Mind; the Universe is Mental.",
            HermeticPrinciple::Correspondence => "As above, so below; as below, so above.",
            HermeticPrinciple::Vibration => "Nothing rests; everything moves; everything vibrates.",
            HermeticPrinciple::Polarity => "Everything is dual; everything has poles; everything has its pair of opposites.",
            HermeticPrinciple::Rhythm => "Everything flows, out and in; everything has its tides; all things rise and fall.",
            HermeticPrinciple::CauseEffect => "Every Cause has its Effect; every Effect has its Cause; everything happens according to Law.",
            HermeticPrinciple::Gender => "Gender is in everything; everything has its Masculine and Feminine Principles.",
        }
    }
}

pub struct KybalionUniverse;

impl KybalionUniverse {
    pub fn consult(principle: HermeticPrinciple) -> &'static str {
        principle.get_axiom()
    }
}


// --- THE ENTERPRISE RPG KERNEL ---

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    Farming,         // Data Mining & Scraping
    Woodcutting,     // Resource Extraction
    Smithing,        // Infrastructure & Tooling (Forja)
    Alchemy,         // Data Transformation (Insights)
    Combat,          // Security & Defense
    Merchanting,     // Revenue & Lead Gen
    Optimization,    // Performance
    Automation,      // Workflow Efficiency
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkinGrade {
    Common,
    Rare,
    Epic,
    Mythic, // Grade S
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MythicSkin {
    pub grade: SkinGrade,
    name: String,
    pub fire_intensity: f32,
    pub thunder_sparks: u32,
}

#[wasm_bindgen]
impl MythicSkin {
    #[wasm_bindgen(constructor)]
    pub fn new(grade: SkinGrade, name: String, fire_intensity: f32, thunder_sparks: u32) -> Self {
        Self { grade, name, fire_intensity, thunder_sparks }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkillProgress {
    pub level: u8,
    pub xp: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Inventory {
    pub materials: DashMap<String, u64>,
    pub equipment: Vec<String>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Avatar {
    name: String,
    title: String,
    capabilities: Arc<DashMap<Capability, SkillProgress>>,
    inventory: Arc<Inventory>,
    current_skin: Option<MythicSkin>,
}

#[wasm_bindgen]
impl Avatar {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        let capabilities = Arc::new(DashMap::new());
        for cap in [
            Capability::Farming,
            Capability::Woodcutting,
            Capability::Smithing,
            Capability::Alchemy,
            Capability::Combat,
            Capability::Merchanting,
            Capability::Optimization,
            Capability::Automation,
        ] {
            capabilities.insert(cap, SkillProgress { level: 1, xp: 0.0 });
        }

        Self {
            name,
            title: "Novice Architect".to_string(),
            capabilities,
            inventory: Arc::new(Inventory::default()),
            current_skin: None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_skin(&mut self, skin: MythicSkin) {
        self.current_skin = Some(skin);
    }

    pub fn get_skin_name(&self) -> String {
        self.current_skin.as_ref().map(|s| s.name.clone()).unwrap_or("None".into())
    }

    pub fn get_level(&self, capability: Capability) -> u8 {
        self.capabilities.get(&capability).map(|s| s.level).unwrap_or(1)
    }

    pub fn get_xp(&self, capability: Capability) -> f64 {
        self.capabilities.get(&capability).map(|s| s.xp).unwrap_or(0.0)
    }

    pub fn grant_xp(&self, capability: Capability, amount: f64) -> u8 {
        let mut boost = 1.0;
        if let Some(_skin) = self.current_skin.as_ref().filter(|s| s.grade == SkinGrade::Mythic && s.name.contains("Phoenix")) {
            if capability == Capability::Alchemy { boost = 1.5; }
            if capability == Capability::Smithing { boost = 1.25; }
        }

        if let Some(mut skill) = self.capabilities.get_mut(&capability) {
            skill.xp += amount * boost;
            let new_level = (skill.xp.sqrt() / 36.0) as u8 + 1;
            let final_level = if new_level > 99 { 99 } else { new_level };
            if final_level > skill.level {
                skill.level = final_level;
                return final_level;
            }
        }
        0
    }

    pub fn add_material(&self, material: String, amount: u64) {
        let mut entry = self.inventory.materials.entry(material).or_insert(0);
        *entry += amount;
    }

    pub fn get_material_count(&self, material: &str) -> u64 {
        self.inventory.materials.get(material).map(|v| *v).unwrap_or(0)
    }
}

// --- PHOENIX ARCHITECT: AI ORCHESTRATION ---

pub struct PhoenixArchitect {
    client: reqwest::Client,
    openrouter_key: Option<String>,
}

impl PhoenixArchitect {
    pub fn new(key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            openrouter_key: key,
        }
    }

    pub async fn ask_ollama(&self, prompt: &str) -> anyhow::Result<String> {
        let res = self.client.post("http://localhost:11434/api/generate")
            .json(&serde_json::json!({
                "model": "llama3",
                "prompt": prompt,
                "stream": false
            }))
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;
        Ok(json["response"].as_str().unwrap_or("").to_string())
    }

    pub async fn hermetic_reasoning(&self, prompt: &str, principle: HermeticPrinciple) -> anyhow::Result<String> {
        let axiom = principle.get_axiom();
        let hermetic_prompt = format!(
            "Acting under the Hermetic Law of {}, which states: '{}'.\n\
            Analyze the following request and provide a response aligned with this universal law:\n\n{}",
            format!("{:?}", principle),
            axiom,
            prompt
        );

        println!("[📜] Applying Hermetic Law: {:?}...", principle);
        self.ask_ollama(&hermetic_prompt).await
    }
    pub async fn ask_openrouter(&self, prompt: &str) -> anyhow::Result<String> {
        let key = self.openrouter_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenRouter key missing"))?;

        let res = self.client.post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            .json(&serde_json::json!({
                "model": "anthropic/claude-3-opus",
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;
        Ok(json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
    }
}

// --- THE ULTRA BRAIN: MULTIVERSAL ORCHESTRATOR ---

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct UltraBrain {
    avatar: Arc<Avatar>,
    universes: Arc<DashMap<String, String>>,
}

impl Default for UltraBrain {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl UltraBrain {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            avatar: Arc::new(Avatar::new("Vanguard".to_string())),
            universes: Arc::new(DashMap::new()),
        }
    }

    pub fn avatar(&self) -> Avatar {
        (*self.avatar).clone()
    }

    pub fn grant_xp(&self, capability: Capability, amount: f64) -> u8 {
        self.avatar.grant_xp(capability, amount)
    }

    pub fn get_level(&self, capability: Capability) -> u8 {
        self.avatar.get_level(capability)
    }

    pub fn create_universe(&self, id: String, name: String) {
        self.universes.insert(id, name);
    }
}

// --- SECURITY: THE NEURAL CRYPT ---

pub struct NeuralCrypt;

impl NeuralCrypt {
    fn get_device_fingerprint() -> String {
        // Obtenemos un identificador único basado en el sistema
        let hw_id = fs::read_to_string("/proc/sys/kernel/random/boot_id")
            .unwrap_or_else(|_| "fallback-hw-id".to_string());
        hw_id.trim().to_string()
    }

    pub fn encrypt(data: &[u8], password: &str) -> anyhow::Result<Vec<u8>> {
        let hw_id = Self::get_device_fingerprint();
        let binding_key = format!("{}:{}", password, hw_id);
        
        let mut salt = [0u8; 16];
        rng().fill_bytes(&mut salt);
        
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(binding_key.as_bytes(), &salt, 100_000, &mut key);
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| anyhow::anyhow!("Cipher creation failed: {}", e))?;
        
        let mut nonce_bytes = [0u8; 12];
        rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        let mut output = Vec::with_capacity(16 + 12 + ciphertext.len());
        output.extend_from_slice(&salt);
        output.extend_from_slice(&nonce_bytes);
        output.extend_from_slice(&ciphertext);
        
        Ok(output)
    }

    pub fn decrypt(encrypted_data: &[u8], password: &str) -> anyhow::Result<Vec<u8>> {
        if encrypted_data.len() < 28 {
            anyhow::bail!("Invalid encrypted data format");
        }
        
        let hw_id = Self::get_device_fingerprint();
        let binding_key = format!("{}:{}", password, hw_id);

        let (salt, rest) = encrypted_data.split_at(16);
        let (nonce_bytes, ciphertext) = rest.split_at(12);
        
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(binding_key.as_bytes(), salt, 100_000, &mut key);
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| anyhow::anyhow!("Cipher creation failed: {}", e))?;
        
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }
}

// --- THE ORACLE BRIDGE: REALITY SYNC ---

#[wasm_bindgen]
pub struct OracleBridge {
    token_hash: Vec<u8>,
}

#[wasm_bindgen]
impl OracleBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(token: String) -> Self {
        let mut hash = [0u8; 32];
        pbkdf2_hmac::<Sha256>(token.as_bytes(), b"oracle-salt", 10_000, &mut hash);
        Self { token_hash: hash.to_vec() }
    }

    pub fn validate_token(&self, token: &str) -> bool {
        let mut hash = [0u8; 32];
        pbkdf2_hmac::<Sha256>(token.as_bytes(), b"oracle-salt", 10_000, &mut hash);
        self.token_hash == hash
    }

    pub fn ingest_event(&self, brain: &UltraBrain, event_type: String, value: f64) -> String {
        let avatar = brain.avatar();
        match event_type.as_str() {
            "raw_data_scraped" => {
                let xp = value * 1.0; // Optimized for Speedrun
                avatar.grant_xp(Capability::Farming, xp);
                avatar.add_material("RawData".into(), value as u64);
                format!("Oracle: Cosechados {} fragmentos de RawData. +{:.2} XP en Farming.", value, xp)
            },
            "infrastructure_built" => {
                let xp = value * 1.0;
                avatar.grant_xp(Capability::Smithing, xp);
                format!("Oracle: Nueva infraestructura forjada. +{:.2} XP en Smithing.", xp)
            },
            "security_breach_deflected" => {
                let xp = value * 1.0;
                avatar.grant_xp(Capability::Combat, xp);
                format!("Oracle: Intrusión repelida con éxito. +{:.2} XP en Combat.", xp)
            },
            "revenue_generated" => {
                let xp = value * 2.0;
                avatar.grant_xp(Capability::Merchanting, xp);
                format!("Oracle: Transacción de ${:.2} completada. +{:.2} XP en Merchanting.", value, xp)
            },
            "alchemy_experiment" => {
                let xp = value * 1.5;
                avatar.grant_xp(Capability::Alchemy, xp);
                format!("Oracle: Experimento de alquimia exitoso. +{:.2} XP en Alchemy.", xp)
            },
            "optimization_task" => {
                let xp = value * 1.0;
                avatar.grant_xp(Capability::Optimization, xp);
                format!("Oracle: Optimización de recursos completa. +{:.2} XP en Optimization.", xp)
            },
            "workflow_automated" => {
                let xp = value * 1.0;
                avatar.grant_xp(Capability::Automation, xp);
                format!("Oracle: Flujo de trabajo automatizado. +{:.2} XP en Automation.", xp)
            },
            "resource_extracted" => {
                let xp = value * 1.0;
                avatar.grant_xp(Capability::Woodcutting, xp);
                format!("Oracle: Recursos extraídos con éxito. +{:.2} XP en Woodcutting.", xp)
            },
            _ => "Oracle: Unknown event type.".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_scaling() {
        let brain = UltraBrain::new();
        let avatar = brain.avatar();
        assert_eq!(avatar.get_level(Capability::Farming), 1);
        avatar.grant_xp(Capability::Farming, 1300.0);
        assert_eq!(avatar.get_level(Capability::Farming), 2);
    }

    #[test]
    fn test_phoenix_buffs() {
        let mut avatar = Avatar::new("Test".to_string());
        avatar.set_skin(MythicSkin {
            grade: SkinGrade::Mythic,
            name: "Mythic Phoenix".to_string(),
            fire_intensity: 1.0,
            thunder_sparks: 10,
        });

        // Alchemy XP with Phoenix buff: 1000 * 1.5 = 1500
        avatar.grant_xp(Capability::Alchemy, 1000.0);
        assert!(avatar.get_xp(Capability::Alchemy) >= 1500.0);
    }
}
