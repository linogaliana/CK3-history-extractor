use std::{borrow::Borrow, cell::{Ref, RefCell}};

use minijinja::{Environment, context};

use serde::{Serialize, ser::SerializeStruct};

use crate::{game_object::GameObject, game_state::GameState};

use super::{renderer::Renderable, Cullable, DerivedRef, Culture, Dynasty, Faith, GameObjectDerived, Memory, Shared, Title};

/// Represents a character in the game.
/// Implements [GameObjectDerived], [Renderable] and [Cullable].
pub struct Character {
    id: u32,
    name: Shared<String>,
    nick: Option<Shared<String>>,
    birth: Shared<String>,
    dead: bool,
    date: Option<Shared<String>>,
    reason: Option<Shared<String>>,
    faith: Option<Shared<Faith>>,
    culture: Option<Shared<Culture>>,
    house: Option<Shared<Dynasty>>,
    skills: Vec<i8>,
    traits: Vec<Shared<String>>,
    recessive: Vec<Shared<String>>,
    spouses: Vec<Shared<Character>>,
    former: Vec<Shared<Character>>,
    children: Vec<Shared<Character>>,
    dna: Option<Shared<String>>,
    memories: Vec<Shared<Memory>>,
    titles: Vec<Shared<Title>>,
    gold: f32,
    piety: f32,
    prestige: f32,
    dread: f32,
    strength: f32,
    kills: Vec<Shared<Character>>,
    languages: Vec<Shared<String>>,
    vassals: Vec<Shared<DerivedRef<Character>>>,
    depth: usize
}

/// Gets the dynasty meant to be the source of some properties
fn get_src_dynasty(house:&Shared<Dynasty>) -> Shared<Dynasty>{
    if house.as_ref().borrow().get_parent().is_some(){
        let p = house.as_ref().borrow().get_parent().as_ref().unwrap().clone();
        return p.clone();
    }
    else{
        house.clone()
    }
}

/// Gets the faith of the character
fn get_faith(house:&Option<Shared<Dynasty>>, base:&GameObject, game_state:&mut GameState) -> Shared<Faith>{
    let faith_node = base.get("faith");
    if faith_node.is_some(){
        game_state.get_faith(faith_node.unwrap().as_string_ref().unwrap().as_str()).clone()
    }
    else{
        let house = get_src_dynasty(house.as_ref().unwrap());
        let h = house.as_ref().borrow();
        for l in h.get_leaders().iter(){
            let l = l.try_borrow();
            if l.is_ok(){
                let o = l.unwrap();
                if o.faith.is_some(){
                    return o.faith.as_ref().unwrap().clone();
                }
            }
        }
        Shared::new(RefCell::new(Faith::dummy(0)))
    }
}

/// Gets the culture of the character
fn get_culture(house:&Option<Shared<Dynasty>>, base:&GameObject, game_state:&mut GameState) -> Shared<Culture>{
    let culture_node = base.get("culture");
    if culture_node.is_some(){
        game_state.get_culture(culture_node.unwrap().as_string_ref().unwrap().as_str()).clone()
    }
    else{
        let h = get_src_dynasty(house.as_ref().unwrap());
        let h = h.as_ref().borrow();
        for l in h.get_leaders().iter(){
            let l = l.try_borrow();
            if l.is_ok(){
                let o = l.unwrap();
                if o.culture.is_some(){
                    return o.culture.as_ref().unwrap().clone();
                }
            }
        }
        Shared::new(RefCell::new(Culture::dummy(0)))
    }
}

/// Gets the skills of the character
fn get_skills(skills:&mut Vec<i8>, base:&GameObject){
    for s in base.get_object_ref("skill").get_array_iter(){
        skills.push(s.as_string_ref().unwrap().parse::<i8>().unwrap());
    }
}

/// Parses the dead_data field of the character
fn get_dead(dead:&mut bool, reason:&mut Option<Shared<String>>, data:&mut Option<Shared<String>>, titles:&mut Vec<Shared<Title>>, base:&GameObject, game_state:&mut GameState){
    let dead_data = base.get("dead_data");
    if dead_data.is_some(){
        *dead = true;
        let o = dead_data.unwrap().as_object_ref().unwrap();
        let reason_node = o.get("reason");
        if reason_node.is_some(){
            *reason = Some(reason_node.unwrap().as_string());
        }
        let domain_node = o.get("domain");
        if domain_node.is_some(){
            for t in domain_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                titles.push(game_state.get_title(t.as_string_ref().unwrap().as_str()));
            }
        }
        *data = Some(o.get("date").unwrap().as_string());
    }
}

/// Gets the recessive traits of the character
fn get_recessive(recessive:&mut Vec<Shared<String>>, base:&GameObject){
    let rec_t = base.get("recessive_traits");
    if rec_t.is_some(){
        for r in rec_t.unwrap().as_object_ref().unwrap().get_array_iter(){
            recessive.push(r.as_string());
        }
    }
}

/// Parses the family_data field of the character
fn get_family(spouses:&mut Vec<Shared<Character>>, former_spouses:&mut Vec<Shared<Character>>, children:&mut Vec<Shared<Character>>, base:&GameObject, game_state:&mut GameState){
    let family_data = base.get("family_data");
    if family_data.is_some(){
        let f = family_data.unwrap().as_object_ref().unwrap();
        let spouse_node = f.get("spouse");
        if spouse_node.is_some() {
            spouses.push(game_state.get_character(spouse_node.unwrap().as_string_ref().unwrap().as_str()).clone());
        }
        let primary_spouse_node = f.get("primary_spouse");
        if primary_spouse_node.is_some() {
            spouses.push(game_state.get_character(primary_spouse_node.unwrap().as_string_ref().unwrap().as_str()).clone());
        }
        let former_spouses_node = f.get("former_spouses");
        if former_spouses_node.is_some() {
            for s in former_spouses_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                former_spouses.push(game_state.get_character(s.as_string_ref().unwrap().as_str()).clone());
            }
        }
        let children_node = f.get("child");
        if children_node.is_some() {
            for s in children_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                children.push(game_state.get_character(s.as_string_ref().unwrap().as_str()).clone());
            }
        }
    }
}

/// Gets the memories of the character
fn get_memories(memories:&mut Vec<Shared<Memory>>, base:&GameObject, game_state:&mut GameState){
    let memory_node = base.get("memories");
    if memory_node.is_some(){
        for m in memory_node.unwrap().as_object_ref().unwrap().get_array_iter(){
            memories.push(game_state.get_memory(m.as_string_ref().unwrap().as_str()).clone());
        }
    }
}

/// Gets the traits of the character
fn get_traits(traits:&mut Vec<Shared<String>>, base:&GameObject, game_state:&mut GameState){
    let traits_node = base.get("traits");
    if traits_node.is_some(){
        for t in traits_node.unwrap().as_object_ref().unwrap().get_array_iter(){
            let index = t.as_string().as_ref().borrow().parse::<u32>().unwrap();
            traits.push(game_state.get_trait(index));
        }
    }
}

/// Parses the alive_data field of the character
fn parse_alive_data(base:&GameObject, piety:&mut f32, prestige:&mut f32, gold:&mut f32, kills:&mut Vec<Shared<Character>>, languages:&mut Vec<Shared<String>>, traits:&mut Vec<Shared<String>>, game_state:&mut GameState){
    let alive_node = base.get("alive_data");
    if alive_node.is_some(){
        let alive_data = base.get("alive_data").unwrap().as_object_ref().unwrap();
        *piety = alive_data.get_object_ref("piety").get_string_ref("accumulated").parse::<f32>().unwrap();
        *prestige = alive_data.get_object_ref("prestige").get_string_ref("accumulated").parse::<f32>().unwrap();
        let kills_node = alive_data.get("kills");
        if kills_node.is_some(){
            for k in kills_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                kills.push(game_state.get_character(k.as_string_ref().unwrap().as_str()).clone());
            }
        }
        let gold_node = alive_data.get("gold");
        if gold_node.is_some(){
            *gold = gold_node.unwrap().as_string_ref().unwrap().parse::<f32>().unwrap();
        }
        for l in alive_data.get_object_ref("languages").get_array_iter(){
            languages.push(l.as_string());
        }
        let perk_node = alive_data.get("perks");
        if perk_node.is_some(){
            for p in perk_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                traits.push(p.as_string());
            }
        }
    }
}

/// Parses the landed_data field of the character
fn get_landed_data(dread:&mut f32, strength:&mut f32, titles:&mut Vec<Shared<Title>>, vassals:&mut Vec<Shared<DerivedRef<Character>>>, base:&GameObject, game_state:&mut GameState){
    let landed_data_node = base.get("landed_data");
    if landed_data_node.is_some(){
        let landed_data = landed_data_node.unwrap().as_object_ref().unwrap();
        let dread_node = landed_data.get("dread");
        if dread_node.is_some(){
            *dread = dread_node.unwrap().as_string_ref().unwrap().parse::<f32>().unwrap();
        }
        let strength_node = landed_data.get("strength");
        if strength_node.is_some(){
            *strength = strength_node.unwrap().as_string_ref().unwrap().parse::<f32>().unwrap();
        }
        let titles_node = landed_data.get("domain");
        if titles_node.is_some(){
            for t in titles_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                titles.push(game_state.get_title(t.as_string_ref().unwrap().as_str()));
            }
        }
        let vassals_node = landed_data.get("vassal_contracts");
        if vassals_node.is_some(){
            for v in vassals_node.unwrap().as_object_ref().unwrap().get_array_iter(){
                vassals.push(game_state.get_vassal(v.as_string_ref().unwrap().as_str()));
            }
        }
    }
}

/// Gets the dynasty of the character
fn get_dynasty(base:&GameObject, game_state:&mut GameState) -> Option<Shared<Dynasty>>{
    let dynasty_id = base.get("dynasty_house");
    if dynasty_id.is_some(){
        return Some(game_state.get_dynasty(dynasty_id.unwrap().as_string_ref().unwrap().as_str()));
    }
    None
}

impl GameObjectDerived for Character {

    fn from_game_object(base:Ref<'_, GameObject>, game_state:&mut GameState) -> Self {
        let mut dead = false;
        let mut reason = None;
        let mut date = None;
        let mut titles: Vec<Shared<Title>> = Vec::new();
        get_dead(&mut dead, &mut reason, &mut date, &mut titles, &base, game_state);
        //find skills
        let mut skills = Vec::new();
        get_skills(&mut skills, &base);
        //find recessive traits
        let mut recessive = Vec::new();
        get_recessive(&mut recessive, &base);
        //find family data
        let mut spouses = Vec::new();
        let mut former_spouses = Vec::new();
        let mut children = Vec::new();
        get_family(&mut spouses, &mut former_spouses, &mut children, &base, game_state);
        //find dna
        let dna = match base.get("dna"){
            Some(d) => Some(d.as_string()),
            None => None
        };
        //find memories
        let mut memories:Vec<Shared<Memory>> = Vec::new();
        get_memories(&mut memories, &base, game_state);
        //find traits
        let mut traits = Vec::new();
        get_traits(&mut traits, &base, game_state);
        //find alive data
        let mut gold = 0.0;
        let mut piety = 0.0;
        let mut prestige = 0.0;
        let mut kills: Vec<Shared<Character>> = Vec::new();
        let mut languages: Vec<Shared<String>> = Vec::new();
        if !dead {
            parse_alive_data(&base, &mut piety, &mut prestige, &mut gold, &mut kills, &mut languages, &mut traits, game_state);
        }
        //find landed data
        let mut dread = 0.0;
        let mut strength = 0.0;
        let mut vassals:Vec<Shared<DerivedRef<Character>>> = Vec::new();
        get_landed_data(&mut dread, &mut strength, &mut titles, &mut vassals, &base, game_state);
        //find house
        let house = get_dynasty(&base, game_state);
        Character{
            name: base.get("first_name").unwrap().as_string(),
            nick: base.get("nickname").map(|v| v.as_string()),
            birth: base.get("birth").unwrap().as_string(),
            dead: dead,
            date: date,
            reason: reason,
            house: house.clone(),
            faith: Some(get_faith(&house, &base, game_state)),
            culture: Some(get_culture(&house, &base, game_state)),
            skills: skills,
            traits: traits,
            recessive:recessive,
            spouses: spouses,
            former: former_spouses,
            children: children,
            dna: dna,
            memories: memories,
            titles: titles,
            piety: piety,
            prestige: prestige,
            dread: dread,
            strength: strength,
            gold: gold,
            kills: kills,
            languages: languages,
            vassals: vassals,
            id: base.get_name().parse::<u32>().unwrap(),
            depth: 0
        }    
    }

    fn dummy(id:u32) -> Self {
        Character{
            name: Shared::new("".to_owned().into()),
            nick: None,
            birth: Shared::new("".to_owned().into()),
            dead: false,
            date: None,
            reason: None,
            faith: None,
            culture: None,
            house: None,
            skills: Vec::new(),
            traits: Vec::new(),
            recessive: Vec::new(),
            spouses: Vec::new(),
            former: Vec::new(),
            children: Vec::new(),
            dna: None,
            memories: Vec::new(),
            titles: Vec::new(),
            gold: 0.0,
            piety: 0.0,
            prestige: 0.0,
            dread: 0.0,
            strength: 0.0,
            kills: Vec::new(),
            languages: Vec::new(),
            vassals: Vec::new(),
            id: id,
            depth: 0
        }
    }

    fn init(&mut self, base:Ref<'_, GameObject>, game_state:&mut GameState) {
        get_dead(&mut self.dead, &mut self.reason, &mut self.date, &mut self.titles, &base, game_state);
        //find skills
        get_skills(&mut self.skills, &base);
        //find recessive traits
        get_recessive(&mut self.recessive, &base);
        get_family(&mut self.spouses, &mut self.former, &mut self.children, &base, game_state);
        //find dna
        let dna = match base.get("dna"){
            Some(d) => Some(d.as_string()),
            None => None
        };
        //find memories
        get_memories(&mut self.memories, &base, game_state);
        //find traits
        get_traits(&mut self.traits, &base, game_state);
        //find alive data
        if !self.dead {
            parse_alive_data(&base, &mut self.piety, &mut self.prestige, &mut self.gold, &mut self.kills, &mut self.languages, &mut self.traits, game_state);
        }
        //find landed data
        get_landed_data(&mut self.dread, &mut self.strength, &mut self.titles, &mut self.vassals, &base, game_state);
        //find house
        let house = get_dynasty(&base, game_state);
        self.name.clone_from(&base.get("first_name").unwrap().as_string());
        self.nick = base.get("nickname").map(|v| v.as_string());
        self.birth.clone_from(&base.get("birth").unwrap().as_string());
        self.house = house.clone();
        self.faith = Some(get_faith(&house, &base, game_state));
        self.culture = Some(get_culture(&house, &base, game_state));
        self.dna = dna;
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self) -> Shared<String> {
        self.name.clone()
    }
}

/// Converts an array of GameObjectDerived to an array of DerivedRef
fn serialize_array<T>(array:&Vec<Shared<T>>) -> Vec<DerivedRef<T>> where T:GameObjectDerived + Cullable{
    let mut res = Vec::new();
    for s in array.iter(){
        res.push(DerivedRef::<T>::from_derived(s.clone()));
    }
    res
}

impl Serialize for Character {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        println!("Serializing character {:?}", self.id);
        if self.depth == 0 {
            let mut state = serializer.serialize_struct("Character", 2)?;
            state.serialize_field("id", &self.id)?;
            state.serialize_field("name", &self.name)?;
            return state.end();
        }
        let mut state = serializer.serialize_struct("Character", 27)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("nick", &self.nick)?;
        state.serialize_field("birth", &self.birth)?;
        state.serialize_field("dead", &self.dead)?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("reason", &self.reason)?;
        let rf = DerivedRef::<Faith>::from_derived(self.faith.as_ref().unwrap().clone());
        state.serialize_field("faith", &rf)?;
        let rc = DerivedRef::<Culture>::from_derived(self.culture.as_ref().unwrap().clone());
        state.serialize_field("culture", &rc)?;
        let rd = DerivedRef::<Dynasty>::from_derived(self.house.as_ref().unwrap().clone());
        state.serialize_field("house", &rd)?;
        state.serialize_field("skills", &self.skills)?;
        state.serialize_field("traits", &self.traits)?;
        state.serialize_field("recessive", &self.recessive)?;
        //serialize spouses as DerivedRef
        let spouses = serialize_array::<Character>(&self.spouses);
        state.serialize_field("spouses", &spouses)?;
        //serialize former as DerivedRef
        let former = serialize_array::<Character>(&self.former);
        state.serialize_field("former", &former)?;
        //serialize children as DerivedRef
        let children = serialize_array::<Character>(&self.children);
        state.serialize_field("children", &children)?;
        state.serialize_field("dna", &self.dna)?;
        //serialize memories as DerivedRef
        state.serialize_field("memories", &self.memories)?;
        //serialize titles as DerivedRef
        let titles = serialize_array::<Title>(&self.titles);
        state.serialize_field("titles", &titles)?;
        state.serialize_field("gold", &self.gold)?;
        state.serialize_field("piety", &self.piety)?;
        state.serialize_field("prestige", &self.prestige)?;
        state.serialize_field("dread", &self.dread)?;
        state.serialize_field("strength", &self.strength)?;
        //serialize kills as DerivedRef
        let kills = serialize_array::<Character>(&self.kills);
        state.serialize_field("kills", &kills)?;
        state.serialize_field("languages", &self.languages)?;
        //vassals is already serialized as DerivedRef
        state.serialize_field("vassals", &self.vassals)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}

impl Renderable for Character {
    fn render(&self, env: &Environment) -> Option<String> {
        if self.depth == 0 {
            return None;
        }
        let ctx = context! {character=>self};
        Some(env.get_template("charTemplate.html").unwrap().render(&ctx).unwrap())
    }

    fn get_subdir(&self) -> &'static str {
        "characters"
    }

    fn render_all(&self, env: &Environment, path: &str) -> std::io::Result<()> {
        let r = self.render_to_file(env, path);
        if r.is_err(){
            if r.as_ref().err().unwrap().kind() != std::io::ErrorKind::AlreadyExists{
                return r;
            }
            else{
                return Ok(());
            }
        }
        if self.faith.is_some(){
            self.faith.as_ref().unwrap().as_ref().borrow().render_all(env, path)?;
        }
        if self.culture.is_some(){
            self.culture.as_ref().unwrap().as_ref().borrow().borrow().render_all(env, path)?;
        }
        if self.house.is_some(){
            self.house.as_ref().unwrap().as_ref().borrow().render_all(env, path)?;
        }
        for s in self.spouses.iter(){
            s.as_ref().borrow().render_all(env, path)?;
        }
        for s in self.former.iter(){
            s.as_ref().borrow().render_all(env, path)?;
        }
        for s in self.children.iter(){
            s.as_ref().borrow().render_all(env, path)?;
        }
        for s in self.kills.iter(){
            s.as_ref().borrow().render_all(env, path)?;
        }
        for s in self.vassals.iter(){
            s.as_ref().borrow().get_ref().as_ref().borrow().render_all(env, path)?;
        }
        for s in self.titles.iter(){
            s.as_ref().borrow().render_all(env, path)?;
        }
        Ok(())
    }
}

impl Cullable for Character {
    fn set_depth(&mut self, depth:usize) {
        if depth <= self.depth || depth == 0 {
            return;
        }
        self.depth = depth;
        for s in self.spouses.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        for s in self.former.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        for s in self.children.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        for s in self.kills.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        for s in self.vassals.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        self.culture.as_ref().unwrap().borrow_mut().set_depth(depth - 1);
        self.faith.as_ref().unwrap().borrow_mut().set_depth(depth - 1);
        for s in self.titles.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        for s in self.memories.iter_mut(){
            s.borrow_mut().set_depth(depth - 1);
        }
        if self.house.is_some(){
            self.house.as_ref().unwrap().borrow_mut().set_depth(depth - 1);
        }
    }

    fn get_depth(&self) -> usize {
        self.depth
    }
}
