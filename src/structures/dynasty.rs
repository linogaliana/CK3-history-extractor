use minijinja::{Environment, context};

use serde::Serialize;
use serde::ser::SerializeStruct;
use super::renderer::Renderable;
use super::{Character, GameObjectDerived, Shared};
use crate::game_object::{GameObject, SaveFileValue};
use crate::game_state::GameState;
use std::cell::Ref;

pub struct Dynasty{
    pub id: u32,
    pub parent: Option<Shared<Dynasty>>,
    pub name: Option<Shared<String>>,
    pub members: u32,
    pub houses: u32,
    pub prestige_tot: f32,
    pub prestige: f32,
    pub perks: Vec<Shared<String>>,
    pub leaders: Vec<Shared<Character>>,
}

///Gets the perks of the dynasty and appends them to the perks vector
fn get_perks(perks:&mut Vec<Shared<String>>, base:&Ref<'_, GameObject>){
    let perks_obj = base.get("perks");
    if perks_obj.is_some(){
        for p in perks_obj.unwrap().as_object_ref().unwrap().get_array_iter(){
            perks.push(p.as_string());
        }
    }
}

///Gets the leaders of the dynasty and appends them to the leaders vector
fn get_leaders(leaders:&mut Vec<Shared<Character>>, base:&Ref<'_, GameObject>, game_state:&mut GameState){
    let leaders_obj = base.get("historical");
    if leaders_obj.is_some(){
        for l in leaders_obj.unwrap().as_object_ref().unwrap().get_array_iter(){
            leaders.push(game_state.get_character(l.as_string_ref().unwrap().as_str()).clone());
        }
    }
}

///Gets the dynasty head of the dynasty
fn get_dynasty_head(base:&Ref<'_, GameObject>, game_state:&mut GameState) -> Option<Shared<Character>>{
    let current = base.get("dynasty_head");
    if current.is_some(){
        return Some(game_state.get_character(current.unwrap().as_string_ref().unwrap().as_str()).clone());
    }
    else{
        let current = base.get("head_of_house");
        if current.is_some(){
            return Some(game_state.get_character(current.unwrap().as_string_ref().unwrap().as_str()).clone());
        }
    }
    None
}

///Gets the prestige of the dynasty and returns a tuple with the total prestige and the current prestige
fn get_prestige(base:&Ref<'_, GameObject>) -> (f32, f32){
    let currency = base.get("prestige");
    let mut prestige_tot = 0.0;
    let mut prestige = 0.0;
    if currency.is_some(){
        let o = currency.unwrap().as_object_ref().unwrap();
        match o.get("accumulated").unwrap() {
            SaveFileValue::Object(ref o) => {
                prestige_tot = o.as_ref().borrow().get_string_ref("value").parse::<f32>().unwrap();
            },
            SaveFileValue::String(ref o) => {
                prestige_tot = o.as_ref().borrow().parse::<f32>().unwrap();
            },
        }
        match o.get("currency") {
            Some(v) => match v {
                SaveFileValue::Object(ref o) => {
                    prestige = o.as_ref().borrow().get_string_ref("value").parse::<f32>().unwrap();
                },
                SaveFileValue::String(ref o) => {
                    prestige = o.as_ref().borrow().parse::<f32>().unwrap();
                },
            },
            None => {}
        }
    }
    (prestige_tot, prestige)
}

///Gets the parent dynasty of the dynasty
fn get_parent(base:&Ref<'_, GameObject>, game_state:&mut GameState) -> Option<Shared<Dynasty>>{
    let parent_id = base.get("dynasty");
    match parent_id {
        None => None,
        k => Some(game_state.get_dynasty(k.unwrap().as_string_ref().unwrap().as_str()).clone())
    }
}

impl GameObjectDerived for Dynasty {
    fn from_game_object(base:Ref<'_, GameObject>, game_state:&mut GameState) -> Self {
        //get the dynasty legacies
        let mut perks = Vec::new();
        get_perks(&mut perks, &base);
        //get the array of leaders
        let mut leaders = Vec::new();
        get_leaders(&mut leaders, &base, game_state);
        //append to this array the leader if its not already there, you would assume that the leader is the first element in the array, but not always
        let head = get_dynasty_head(&base, game_state);
        if head.is_some(){
            let head = head.unwrap();
            leaders.insert(0, head);
        }
        let res = get_prestige(&base);
        let name:Option<Shared<String>> = match base.get("name") {
            Some(n) => Some(n.as_string()),
            None => None
        };
        Dynasty{
            name: name,
            parent: get_parent(&base, game_state),
            members: 0,
            houses: 0,
            prestige_tot: res.0,
            prestige: res.1,
            perks: perks,
            leaders: leaders,
            id: base.get_name().parse::<u32>().unwrap()
        }
    }

    fn dummy(id:u32) -> Self {
        Dynasty{
            name: Some(Shared::new("".to_owned().into())),
            parent: None,
            members: 0,
            houses: 0,
            prestige_tot: 0.0,
            prestige: 0.0,
            perks: Vec::new(),
            leaders: Vec::new(),
            id: id
        }
    }

    fn init(&mut self, base:Ref<'_, GameObject>, game_state:&mut crate::game_state::GameState) {
        get_perks(&mut self.perks, &base);
        get_leaders(&mut self.leaders, &base, game_state);
        let head = get_dynasty_head(&base, game_state);
        if head.is_some(){
            let head = head.unwrap();
            self.leaders.insert(0, head);
        }
        let res = get_prestige(&base);
        self.prestige_tot = res.0;
        self.prestige = res.1;
        self.parent = get_parent(&base, game_state);
        let name:Option<Shared<String>> = match base.get("name") {
            Some(n) => Some(n.as_string()),
            None => None
        };
        self.name = name;
        self.members = 0;
        self.houses = 0;
        self.id = base.get_name().parse::<u32>().unwrap();
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Serialize for Dynasty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Dynasty", 8)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("members", &self.members)?;
        state.serialize_field("houses", &self.houses)?;
        state.serialize_field("prestige_tot", &self.prestige_tot)?;
        state.serialize_field("prestige", &self.prestige)?;
        state.serialize_field("perks", &self.perks)?;
        state.serialize_field("leaders", &self.leaders)?;
        state.end()
    }
}

impl Renderable for Dynasty {
    fn render(&self, env: &Environment, template_name: &'static String) -> String {
        let ctx = context! {dynasty=>self};
        env.get_template(template_name).unwrap().render(&ctx).unwrap()   
    }
}