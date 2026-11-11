//! Game detection layer for multi-game telemetry support.
//!
//! Based on RaceIQ's adapter pattern: each game registers a `canHandle(buf)` check
//! that quickly identifies whether a UDP buffer belongs to that game.
//!
//! Supported games:
//! - F1 25: UDP on port 20777, packet_format == 2025 in header
//! - Assetto Corsa / ACC: via Windows shared memory (Content Manager)
//! - Assetto Corsa Evo: via Windows shared memory

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Unique game identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameId {
    F1_2025,
    AssettoCorsa,
    AssettoCorsaCompetizione,
    AssettoCorsaEvo,
    Unknown,
}

impl GameId {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameId::F1_2025 => "f1-2025",
            GameId::AssettoCorsa => "assetto-corsa",
            GameId::AssettoCorsaCompetizione => "assetto-corsa-competizione",
            GameId::AssettoCorsaEvo => "assetto-corsa-evo",
            GameId::Unknown => "unknown",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            GameId::F1_2025 => "F1 25",
            GameId::AssettoCorsa => "Assetto Corsa",
            GameId::AssettoCorsaCompetizione => "Assetto Corsa Competizione",
            GameId::AssettoCorsaEvo => "Assetto Corsa Evo",
            GameId::Unknown => "Unknown",
        }
    }

    pub fn default_port(&self) -> u16 {
        match self {
            GameId::F1_2025 => 20777,
            GameId::AssettoCorsa => 0, // shared memory, no UDP port
            GameId::AssettoCorsaCompetizione => 0, // shared memory
            GameId::AssettoCorsaEvo => 0, // shared memory
            GameId::Unknown => 0,
        }
    }
}

/// Result of game detection for a received UDP packet
#[derive(Debug, Clone)]
pub struct GameDetection {
    pub game_id: GameId,
    pub source_addr: SocketAddr,
    pub confidence: f32, // 0.0 to 1.0
}

/// Trait for game-specific packet detection and parsing
pub trait GameAdapter: Send + Sync {
    /// Unique game identifier
    fn id(&self) -> GameId;

    /// Display name for UI
    fn display_name(&self) -> &'static str;

    /// Quick check: does this buffer belong to this game?
    fn can_handle(&self, buf: &[u8]) -> bool;

    /// Default UDP port for this game (0 if shared memory only)
    fn default_port(&self) -> u16;

    /// Process names to check if this game is running (Windows)
    fn process_names(&self) -> Vec<&'static str> {
        Vec::new()
    }
}

/// F1 25 game adapter - detects via UDP packet header (packet_format == 2025)
pub struct F1_2025Adapter;

impl GameAdapter for F1_2025Adapter {
    fn id(&self) -> GameId {
        GameId::F1_2025
    }

    fn display_name(&self) -> &'static str {
        "F1 25"
    }

    fn can_handle(&self, buf: &[u8]) -> bool {
        // F1 25 header: 29 bytes, packet_format (u16 LE at offset 0) == 2025
        buf.len() >= 29 && u16::from_le_bytes([buf[0], buf[1]]) == 2025
    }

    fn default_port(&self) -> u16 {
        20777
    }

    fn process_names(&self) -> Vec<&'static str> {
        vec!["F1_25.exe", "F1_2025.exe"]
    }
}

/// Assetto Corsa game adapter - detects via shared memory or UDP
pub struct AssettoCorsaAdapter;

impl GameAdapter for AssettoCorsaAdapter {
    fn id(&self) -> GameId {
        GameId::AssettoCorsa
    }

    fn display_name(&self) -> &'static str {
        "Assetto Corsa"
    }

    fn can_handle(&self, _buf: &[u8]) -> bool {
        // AC doesn't send standard UDP telemetry by default;
        // detection via shared memory (Content Manager) or custom UDP plugin
        false
    }

    fn default_port(&self) -> u16 {
        0 // shared memory only
    }

    fn process_names(&self) -> Vec<&'static str> {
        vec!["acs.exe", "assettocorsa.exe"]
    }
}

/// Assetto Corsa Competizione game adapter - Windows shared memory
pub struct AssettoCorsaCompetizioneAdapter;

impl GameAdapter for AssettoCorsaCompetizioneAdapter {
    fn id(&self) -> GameId {
        GameId::AssettoCorsaCompetizione
    }

    fn display_name(&self) -> &'static str {
        "Assetto Corsa Competizione"
    }

    fn can_handle(&self, _buf: &[u8]) -> bool {
        // ACC uses shared memory, not UDP
        false
    }

    fn default_port(&self) -> u16 {
        0
    }

    fn process_names(&self) -> Vec<&'static str> {
        vec!["acc.exe", "acs2.exe", "AC2-Win64-Shipping.exe"]
    }
}

/// Assetto Corsa Evo game adapter - Windows shared memory
pub struct AssettoCorsaEvoAdapter;

impl GameAdapter for AssettoCorsaEvoAdapter {
    fn id(&self) -> GameId {
        GameId::AssettoCorsaEvo
    }

    fn display_name(&self) -> &'static str {
        "Assetto C Corsa Evo"
    }

    fn can_handle(&self, _buf: &[u8]) -> bool {
        false
    }

    fn default_port(&self) -> u16 {
        0
    }

    fn process_names(&self) -> Vec<&'static str> {
        vec!["AC2-Win64-Shipping.exe"]
    }
}

/// Registry of game adapters
pub struct GameRegistry {
    adapters: Vec<Box<dyn GameAdapter>>,
    cached_game: Option<GameId>,
    cache_checked_at: std::time::Instant,
}

impl GameRegistry {
    pub fn new() -> Self {
        let mut adapters: Vec<Box<dyn GameAdapter>> = Vec::new();
        adapters.push(Box::new(F1_2025Adapter));
        adapters.push(Box::new(AssettoCorsaAdapter));
        adapters.push(Box::new(AssettoCorsaCompetizioneAdapter));
        adapters.push(Box::new(AssettoCorsaEvoAdapter));

        Self {
            adapters,
            cached_game: None,
            cache_checked_at: std::time::Instant::now(),
        }
    }

    /// Detect which game sent this UDP packet. Caches result for 5 seconds.
    pub fn detect_game(&mut self, buf: &[u8], source_addr: SocketAddr) -> Option<GameDetection> {
        // Check cached game first
        if let Some(game_id) = self.cached_game {
            if self.cache_checked_at.elapsed() < std::time::Duration::from_secs(5) {
                if let Some(adapter) = self.adapters.iter().find(|a| a.id() == game_id) {
                    if adapter.can_handle(buf) {
                        return Some(GameDetection {
                            game_id,
                            source_addr,
                            confidence: 0.95,
                        });
                    }
                }
                // Cache miss - game changed, re-detect
                self.cached_game = None;
            }
        }

        // Try all adapters in registration order (priority order)
        for adapter in &self.adapters {
            if adapter.can_handle(buf) {
                self.cached_game = Some(adapter.id());
                self.cache_checked_at = std::time::Instant::now();
                return Some(GameDetection {
                    game_id: adapter.id(),
                    source_addr,
                    confidence: 0.9,
                });
            }
        }

        None
    }

    /// Get adapter by game ID
    pub fn get_adapter(&self, game_id: GameId) -> Option<&Box<dyn GameAdapter>> {
        self.adapters.iter().find(|a| a.id() == game_id)
    }

    /// Get all registered game IDs
    pub fn get_all_game_ids(&self) -> Vec<GameId> {
        self.adapters.iter().map(|a| a.id()).collect()
    }

    /// Clear cache (force re-detection on next packet)
    pub fn clear_cache(&mut self) {
        self.cached_game = None;
    }
}

impl Default for GameRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1_25_detection() {
        let adapter = F1_2025Adapter;
        // Valid F1 25 header: packet_format = 2025 (0x07E9)
        let mut buf = [0u8; 29];
        buf[0] = 0xE9; // low byte of 2025
        buf[1] = 0x07; // high byte of 2025
        assert!(adapter.can_handle(&buf));

        // Invalid: wrong packet format
        let mut buf2 = [0u8; 29];
        buf2[0] = 0x00;
        buf2[1] = 0x00;
        assert!(!adapter.can_handle(&buf2));

        // Too short
        let buf3 = [0u8; 10];
        assert!(!adapter.can_handle(&buf3));
    }

    #[test]
    fn test_registry_detection() {
        let mut registry = GameRegistry::new();

        // Valid F1 25 packet
        let mut buf = [0u8; 29];
        buf[0] = 0xE9;
        buf[1] = 0x07;
        let addr: SocketAddr = "127.0.0.1:20777".parse().unwrap();

        let detection = registry.detect_game(&buf, addr);
        assert!(detection.is_some());
        assert_eq!(detection.unwrap().game_id, GameId::F1_2025);

        // Unknown packet
        let buf2 = [0u8; 100];
        let detection2 = registry.detect_game(&buf2, addr);
        assert!(detection2.is_none());
    }
}
