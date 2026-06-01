/// Griot-style room memory — rooms remember their history with exponential decay.
/// Rooms that maintain their traditions are healthier.

use std::collections::HashMap;

/// A single remembered event.
#[derive(Debug, Clone)]
pub struct MemoryEvent {
    pub event_type: String,
    pub weight: f64,
    pub tick: u64,
}

/// Griot-style room memory with exponential decay.
#[derive(Debug, Clone)]
pub struct RoomMemory {
    room_id: String,
    events: Vec<MemoryEvent>,
    decay_rate: f64, // 0.0–1.0, higher = faster decay
    current_tick: u64,
    /// Traditional practices this room maintains
    traditions: HashMap<String, f64>,
}

impl RoomMemory {
    pub fn new(room_id: String, decay_rate: f64) -> Self {
        Self {
            room_id,
            events: Vec::new(),
            decay_rate: decay_rate.clamp(0.0, 1.0),
            current_tick: 0,
            traditions: HashMap::new(),
        }
    }

    /// Record an event in the room's memory.
    pub fn record_event(&mut self, event_type: String) {
        self.current_tick += 1;
        self.events.push(MemoryEvent {
            event_type,
            weight: 1.0,
            tick: self.current_tick,
        });
    }

    /// Record an event with a specific weight.
    pub fn record_weighted_event(&mut self, event_type: String, weight: f64) {
        self.current_tick += 1;
        self.events.push(MemoryEvent {
            event_type,
            weight,
            tick: self.current_tick,
        });
    }

    /// Get memory strengths of all events with exponential decay applied.
    pub fn memory_strengths(&self) -> Vec<f64> {
        self.events
            .iter()
            .map(|e| {
                let age = self.current_tick.saturating_sub(e.tick) as f64;
                e.weight * (-self.decay_rate * age).exp()
            })
            .collect()
    }

    /// Traditions are event types that recur regularly. Score reflects how well
    /// the room maintains its traditional patterns.
    pub fn tradition_score(&self) -> f64 {
        if self.events.is_empty() {
            return 0.0;
        }

        // Count recurrence of each event type
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for e in &self.events {
            *type_counts.entry(e.event_type.clone()).or_default() += 1;
        }

        // Traditions are types that occur more than once
        let tradition_types: Vec<_> = type_counts.iter().filter(|(_, &c)| c > 1).collect();

        if tradition_types.is_empty() {
            return 0.0;
        }

        // Score based on how much of the room's activity is traditional
        let traditional_events: usize = tradition_types.iter().map(|(_, &c)| c).sum();
        let recency_bonus: f64 = tradition_types
            .iter()
            .map(|(t, _)| {
                // Find the most recent occurrence
                let last_tick = self
                    .events
                    .iter()
                    .rev()
                    .find(|e| &e.event_type == *t)
                    .map(|e| e.tick)
                    .unwrap_or(0);
                let age = self.current_tick.saturating_sub(last_tick) as f64;
                (-self.decay_rate * age).exp()
            })
            .sum::<f64>()
            / tradition_types.len() as f64;

        let coverage = traditional_events as f64 / self.events.len() as f64;
        (coverage * 0.7 + recency_bonus * 0.3).min(1.0)
    }

    /// Get the room ID.
    pub fn room_id(&self) -> &str {
        &self.room_id
    }

    /// Get event count.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Register a tradition for this room.
    pub fn add_tradition(&mut self, name: String, importance: f64) {
        self.traditions.insert(name, importance);
    }

    /// Check if a specific tradition is being maintained.
    pub fn tradition_maintained(&self, tradition: &str) -> bool {
        let count = self.events.iter().filter(|e| e.event_type == tradition).count();
        count >= 2
    }
}

/// Griot memory manager: manages memories for multiple rooms.
#[derive(Debug, Clone)]
pub struct GriotMemory {
    rooms: HashMap<String, RoomMemory>,
    default_decay: f64,
}

impl GriotMemory {
    pub fn new(default_decay: f64) -> Self {
        Self {
            rooms: HashMap::new(),
            default_decay,
        }
    }

    /// Register a room with memory.
    pub fn add_room(&mut self, room_id: String) {
        self.rooms
            .entry(room_id.clone())
            .or_insert_with(|| RoomMemory::new(room_id, self.default_decay));
    }

    /// Record an event in a room.
    pub fn record(&mut self, room_id: &str, event_type: String) {
        let decay = self.default_decay;
        self.rooms
            .entry(room_id.to_string())
            .or_insert_with(|| RoomMemory::new(room_id.to_string(), decay))
            .record_event(event_type);
    }

    /// Get the health score based on tradition maintenance across all rooms.
    pub fn system_health(&self) -> f64 {
        if self.rooms.is_empty() {
            return 1.0;
        }
        let total: f64 = self.rooms.values().map(|r| r.tradition_score()).sum();
        total / self.rooms.len() as f64
    }

    /// Get a specific room's memory.
    pub fn room(&self, room_id: &str) -> Option<&RoomMemory> {
        self.rooms.get(room_id)
    }

    /// Get all room scores.
    pub fn all_tradition_scores(&self) -> Vec<(&str, f64)> {
        self.rooms
            .iter()
            .map(|(id, mem)| (id.as_str(), mem.tradition_score()))
            .collect()
    }
}

impl Default for GriotMemory {
    fn default() -> Self {
        Self::new(0.1)
    }
}
