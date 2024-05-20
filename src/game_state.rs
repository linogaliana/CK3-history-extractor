use std::collections::HashMap;

use super::structures::{Character, Culture, DerivedRef, Dynasty, Faith, DummyInit, Memory, Title, GameObjectDerived};
use super::game_object::{GameId, GameObject, GameString};
use super::types::{Shared, Wrapper, WrapperMut};

/// A struct representing all known game objects.
/// It is guaranteed to always return a reference to the same object for the same key.
/// Naturally the value of that reference may change as values are added to the game state.
/// This is mainly used during the process of gathering data from the parsed save file.
pub struct GameState{
    /// A character id->Character transform
    characters: HashMap<GameId, Shared<Character>>,
    /// A title id->Title transform
    titles: HashMap<GameId, Shared<Title>>,
    /// A faith id->Title transform
    faiths: HashMap<GameId, Shared<Faith>>,
    /// A culture id->Culture transform
    cultures: HashMap<GameId, Shared<Culture>>,
    /// A dynasty id->Dynasty transform
    dynasties: HashMap<GameId, Shared<Dynasty>>,
    /// A memory id->Memory transform
    memories: HashMap<GameId, Shared<Memory>>,
    /// A trait id->Trait identifier transform
    traits_lookup: Vec<GameString>,
    /// A vassal contract id->Character transform
    contract_transform: HashMap<GameId, Shared<DerivedRef<Character>>>
}

impl GameState{
    /// Create a new GameState
    pub fn new() -> GameState{
        GameState{
            characters: HashMap::new(),
            titles: HashMap::new(),
            faiths: HashMap::new(),
            cultures: HashMap::new(),
            dynasties: HashMap::new(),
            memories: HashMap::new(),
            traits_lookup: Vec::new(),
            contract_transform: HashMap::new()
        }
    }

    /// Add a lookup table for traits
    pub fn add_lookup(&mut self, array:Vec<GameString>){
        self.traits_lookup = array;
    }

    /// Get a trait by id
    pub fn get_trait(&self, id:u16) -> GameString{
        self.traits_lookup[id as usize].clone()
    }

    /// Get a character by key
    pub fn get_character(&mut self, key: &GameId) -> Shared<Character>{
        if !self.characters.contains_key(key){
            let v = Shared::wrap(Character::dummy(*key));
            self.characters.insert(*key, v.clone());
            v
        }
        else{
            self.characters.get(key).unwrap().clone()
        }
    }

    /// Gets the vassal associated with the contract with the given id
    pub fn get_vassal(&mut self, contract_id: &GameId) -> Shared<DerivedRef<Character>>{
        if !self.contract_transform.contains_key(contract_id){
            let v = Shared::wrap(DerivedRef::dummy());
            self.contract_transform.insert(*contract_id, v.clone());
            v
        }
        else{
            self.contract_transform.get(contract_id).unwrap().clone()
        }
    }

    /// Adds a new vassal contract
    pub fn add_contract(&mut self, contract_id:&GameId, character_id: &GameId) {
        let char = self.get_character(character_id);
        if self.contract_transform.contains_key(contract_id){
            let entry = self.contract_transform.get(contract_id).unwrap();
            entry.get_internal_mut().init(char);
        }
        else{
            let r = Shared::wrap(DerivedRef::from_derived(char));
            self.contract_transform.insert(*contract_id, r);
        }
    }

    /// Get a title by key
    pub fn get_title(&mut self, key: &GameId) -> Shared<Title>{
        if !self.titles.contains_key(key){
            let v = Shared::wrap(Title::dummy(*key));
            self.titles.insert(*key, v.clone());
            v
        }
        else{
            self.titles.get(key).unwrap().clone()
        }
    }

    /// Get a faith by key
    pub fn get_faith(&mut self, key: &GameId) -> Shared<Faith>{
        if self.faiths.contains_key(key){
            self.faiths.get(key).unwrap().clone()
        }
        else{
            let v = Shared::wrap(Faith::dummy(*key));
            self.faiths.insert(*key, v.clone());
            v
        }
    }

    /// Get a culture by key
    pub fn get_culture(&mut self, key: &GameId) -> Shared<Culture>{
        if self.cultures.contains_key(key){
            self.cultures.get(key).unwrap().clone()
        }
        else{
            let v = Shared::wrap(Culture::dummy(*key));
            self.cultures.insert(*key, v.clone());
            v
        }
    }

    /// Get a dynasty by key
    pub fn get_dynasty(&mut self, key: &GameId) -> Shared<Dynasty>{
        if self.dynasties.contains_key(key){
            self.dynasties.get(key).unwrap().clone()
        }
        else{
            let v = Shared::wrap(Dynasty::dummy(*key));
            self.dynasties.insert(*key, v.clone());
            v
        }
    }

    /// Get a memory by key
    pub fn get_memory(&mut self, key: &GameId) -> Shared<Memory>{
        if self.memories.contains_key(key){
            self.memories.get(key).unwrap().clone()
        }
        else{
            let v = Shared::wrap(Memory::dummy(*key));
            self.memories.insert(*key, v.clone());
            v
        }
    }

    /// Add a character to the game state    
    pub fn add_character(&mut self, value: &GameObject){
        let key = value.get_name().parse::<GameId>().unwrap();
        if self.characters.contains_key(&key){
            let c = self.characters.get(&key).unwrap().clone();
            c.get_internal_mut().init(value, self);
        }
        else{
            let c = Shared::wrap(Character::dummy(key));
            self.characters.insert(key.clone(), c.clone());
            c.get_internal_mut().init(value, self);
        }
    }

    /// Add a title to the game state
    pub fn add_title(&mut self, value: &GameObject){
        let key = value.get_name().parse::<GameId>().unwrap();
        if self.titles.contains_key(&key){
            let t = self.titles.get(&key).unwrap().clone();
            t.get_internal_mut().init(value, self);
        }
        else{
            let t = Shared::wrap(Title::dummy(key));
            self.titles.insert(key.clone(), t.clone());
            t.get_internal_mut().init(value, self);
            
        }
    }

    /// Add a faith to the game state
    pub fn add_faith(&mut self, value: &GameObject){
        let key = value.get_name().parse::<GameId>().unwrap();
        if self.faiths.contains_key(&key){
            let f = self.faiths.get(&key).unwrap().clone();
            f.get_internal_mut().init(value, self);
        }
        else{
            let f = Shared::wrap(Faith::dummy(key));
            self.faiths.insert(key.clone(), f.clone());
            f.get_internal_mut().init(value, self);
        }
    }

    /// Add a culture to the game state
    pub fn add_culture(&mut self, value: &GameObject){
        let key = value.get_name().parse::<GameId>().unwrap();
        if self.cultures.contains_key(&key){
            let c = self.cultures.get(&key).unwrap().clone();
            c.get_internal_mut().init(value, self);
        }
        else{
            let c = Shared::wrap(Culture::dummy(key));
            self.cultures.insert(key.clone(), c.clone());
            c.get_internal_mut().init(value, self);
        }
    }

    /// Add a dynasty to the game state
    pub fn add_dynasty(&mut self, value: &GameObject){
        let key = value.get_name().parse::<GameId>().unwrap();
        if self.dynasties.contains_key(&key){
            let d = self.dynasties.get(&key).unwrap().clone();
            d.get_internal_mut().init(value, self);
        }
        else{
            let d = Shared::wrap(Dynasty::dummy(key));
            self.dynasties.insert(key.clone(), d.clone());
            d.get_internal_mut().init(value, self);
        }
    }

    /// Add a memory to the game state
    pub fn add_memory(&mut self, value: &GameObject){
        let key = value.get_name().parse::<GameId>().unwrap();
        if self.memories.contains_key(&key){
            let m = self.memories.get(&key).unwrap().clone();
            m.get_internal_mut().init(value, self);
        }
        else{
            let m = Shared::wrap(Memory::dummy(key));
            self.memories.insert(key.clone(), m.clone());
            m.get_internal_mut().init(value, self);
        }
    }

    /// Returns a hashmap year->number of deaths for a given faith
    pub fn get_faith_graph_data(&self, faith_id:GameId) -> Vec<(u16, i32)>{
        let mut res = HashMap::new();
        for (_, character) in &self.characters {
            let char = character.get_internal();
            let f = char.get_faith();
            if f.is_none(){
                continue;
            }
            let faith = f.unwrap();
            if faith.get_internal().get_id() != faith_id {
                continue;
            }
            let death_date = char.get_death_date();
            if death_date.is_none() {
                continue;
            }
            let death_date = death_date.unwrap();
            let death_year:u16 = death_date.split_once('.').unwrap().0.parse().unwrap();
            if res.contains_key(&death_year){
                let entry = res.get_mut(&death_year).unwrap();
                *entry += 1;
            }
            else{
                res.insert(death_year, 1);
            }
        }
        let mut res:Vec<(u16, i32)> = res.into_iter().collect();
        res.sort_by(|a, b| a.0.cmp(&b.0));
        return res;
    }

}
