use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use dashmap::DashMap;
use std::sync::Arc;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::{RngCore, rng};

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

    pub fn get_level(&self, capability: Capability) -> u8 {
        self.capabilities.get(&capability).map(|s| s.level).unwrap_or(1)
    }

    pub fn get_xp(&self, capability: Capability) -> f64 {
        self.capabilities.get(&capability).map(|s| s.xp).unwrap_or(0.0)
    }

    pub fn grant_xp(&self, capability: Capability, amount: f64) -> u8 {
        if let Some(mut skill) = self.capabilities.get_mut(&capability) {
            skill.xp += amount;
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
    pub fn encrypt(data: &[u8], password: &str) -> anyhow::Result<Vec<u8>> {
        let mut salt = [0u8; 16];
        rng().fill_bytes(&mut salt);
        
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 100_000, &mut key);
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| anyhow::anyhow!("Cipher creation failed: {}", e))?;
        
        let mut nonce_bytes = [0u8; 12];
        rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        // Output format: [salt:16][nonce:12][ciphertext]
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
        
        let (salt, rest) = encrypted_data.split_at(16);
        let (nonce_bytes, ciphertext) = rest.split_at(12);
        
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
        
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
                let xp = value * 0.1;
                avatar.grant_xp(Capability::Farming, xp);
                avatar.add_material("RawData".into(), value as u64);
                format!("Oracle: Cosechados {} fragmentos de RawData. +{:.2} XP en Farming.", value, xp)
            },
            "infrastructure_built" => {
                let xp = 500.0;
                avatar.grant_xp(Capability::Smithing, xp);
                "Oracle: Nueva infraestructura forjada. +500.00 XP en Smithing.".into()
            },
            "security_breach_deflected" => {
                avatar.grant_xp(Capability::Combat, 1000.0);
                "Oracle: Intrusión repelida con éxito. +1000.00 XP en Combat.".into()
            },
            "revenue_generated" => {
                let xp = value * 2.0;
                avatar.grant_xp(Capability::Merchanting, xp);
                format!("Oracle: Transacción de ${:.2} completada. +{:.2} XP en Merchanting.", value, xp)
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
        // Level 1: 0 XP
        assert_eq!(avatar.get_level(Capability::Farming), 1);
        
        // Gain XP to reach Level 2
        avatar.grant_xp(Capability::Farming, 1300.0);
        assert_eq!(avatar.get_level(Capability::Farming), 2);
    }

    #[test]
    fn test_inventory_management() {
        let brain = UltraBrain::new();
        let avatar = brain.avatar();
        avatar.add_material("Iron".into(), 50);
        assert_eq!(avatar.get_material_count("Iron"), 50);
        
        avatar.add_material("Iron".into(), 50);
        assert_eq!(avatar.get_material_count("Iron"), 100);
    }

    #[test]
    fn test_neural_crypt() {
        let data = b"Sensitive vanguard avatar data";
        let password = "oracle-secret-token";
        
        let encrypted = NeuralCrypt::encrypt(data, password).expect("Encryption failed");
        let decrypted = NeuralCrypt::decrypt(&encrypted, password).expect("Decryption failed");
        
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_oracle_bridge_validation() {
        let token = "secure-token-123";
        let bridge = OracleBridge::new(token.to_string());
        
        assert!(bridge.validate_token(token));
        assert!(!bridge.validate_token("wrong-token"));
    }
}
