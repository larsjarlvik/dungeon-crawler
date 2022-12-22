use bevy_ecs::prelude::*;
use std::ops::Range;
mod health;
pub use health::*;

#[derive(Component)]
pub struct Stats {
    pub strength: u32,
    pub strength_damage: f32,
    pub damage_base: Range<f32>,

    pub vitality: u32,
    pub vitality_health: f32,

    pub dexterity: u32,
    pub dexterity_speed: f32,
    pub speed_base: f32,

    pub experience: u32,
    pub health: health::Health,

    pub team: usize,
}

impl Stats {
    pub fn new(strength: u32, vitality: u32, dexterity: u32, experience: u32, team: usize) -> Self {
        let level = get_level(experience) - 1;
        let vitality = vitality + level * 2;
        let vitality_health = 2.0;

        Self {
            strength: strength + level * 2,
            strength_damage: 0.05,
            vitality,
            vitality_health: 2.0,
            dexterity: dexterity + level,
            dexterity_speed: 0.02,
            speed_base: 0.4,
            damage_base: 0.5..1.2,
            experience,
            health: Health::new(vitality as f32 * vitality_health),
            team,
        }
    }

    pub fn get_base_health(&self) -> f32 {
        self.vitality as f32 * self.vitality_health
    }

    pub fn get_attack_time(&self) -> f32 {
        self.speed_base / (self.dexterity as f32 * self.dexterity_speed)
    }

    pub fn get_attack_damage(&self) -> Range<f32> {
        let additional_damage = self.strength as f32 * self.strength_damage;
        (self.damage_base.start + additional_damage)..(self.damage_base.end + additional_damage)
    }

    pub fn get_recovery_time(&self) -> f32 {
        (self.speed_base * 0.4) / (self.dexterity as f32 * self.dexterity_speed * 0.7)
    }

    pub fn get_kill_experience(&self) -> u32 {
        (self.experience as f32 * 0.02) as u32
    }

    pub fn get_level(&self) -> u32 {
        get_level(self.experience)
    }

    pub fn level_up(&mut self) {
        self.vitality += 2;
        self.strength += 2;
        self.dexterity += 1;
    }
}

pub fn get_level(experience: u32) -> u32 {
    let mut level = 0;
    while experience >= get_level_experience(level + 1) && level < 40 {
        level += 1;
    }
    level
}

pub fn get_level_experience(level: u32) -> u32 {
    ((level as f32 - 1.0).powf(1.3) * 1000.0) as u32
}
