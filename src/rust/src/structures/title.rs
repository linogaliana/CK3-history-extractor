use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use minijinja::{Environment, context};

use serde::Serialize;
use serde::ser::SerializeStruct;

use crate::game_object::{GameObject, SaveFileValue};
use crate::game_state::GameState;

use super::renderer::Renderable;
use super::{Character, GameObjectDerived, Shared};

pub struct Title {
    pub name: Shared<String>,
    pub de_jure: Option<Shared<Title>>,
    pub de_facto: Option<Shared<Title>>,
    pub vassals: Vec<Shared<Title>>,
    pub history: HashMap<String, (Option<Shared<Character>>, Shared<String>)>
}

fn get_history(base:Ref<'_, GameObject>, game_state:&mut GameState) -> HashMap<String, (Option<Shared<Character>>, Shared<String>)>{
    let mut history = HashMap::new();
    let hist = base.get("history");
    if hist.is_some() {
        let hist_obj = hist.unwrap().as_object_ref().unwrap();
        for h in hist_obj.get_keys(){
            let val = hist_obj.get(&h);
            let character;
            let action:Shared<String>;
            match val{
                Some(&SaveFileValue::Object(ref o)) => {
                    let r = o.as_ref().borrow();
                    action = r.get("type").unwrap().as_string();
                    let holder = r.get("holder");
                    match holder{
                        Some(h) => {
                            character = Some(game_state.get_character(h.as_string_ref().unwrap().as_str()).clone());
                        },
                        None => {
                            character = None;
                        }
                    }
                },
                Some(&SaveFileValue::String(ref o)) => {
                    action = Rc::new(RefCell::new("Inherited".to_owned()));
                    character = Some(game_state.get_character(o.as_ref().borrow().as_str()).clone());
                }
                _ => {
                    panic!("Invalid history entry")
                }
            }
            let ent = (character, action);
            history.insert(h, ent);
        }
    }
    history
}

impl GameObjectDerived for Title{

    fn from_game_object(base: Ref<'_, GameObject>, game_state: &mut GameState) -> Self {
        //first we get the optional de_jure_liege and de_facto_liege
        let de_jure_id = base.get("de_jure_liege");
        let de_jure = match de_jure_id{
            Some(de_jure) => Some(game_state.get_title(de_jure.as_string_ref().unwrap().as_str()).clone()),
            None => None
        };
        let de_facto_id = base.get("de_facto_liege");
        let de_facto = match de_facto_id{
            Some(de_facto) => Some(game_state.get_title(de_facto.as_string_ref().unwrap().as_str()).clone()),
            None => None
        };
        let mut vassals = Vec::new();
        //if the title has vassals, we get them
        let vas = base.get("vassals");
        if !vas.is_none(){
            for v in base.get_object_ref("vassals").get_array_iter(){
                vassals.push(game_state.get_title(v.as_string_ref().unwrap().as_str()).clone());
            }
        }
        let name = base.get("name").unwrap().as_string();
        let history = get_history(base, game_state);
        Title{
            name: name,
            de_jure: de_jure,
            de_facto: de_facto,
            vassals: vassals,
            history: history
        }
    }

    fn type_name() -> &'static str {
        "title"
    }

    fn dummy() -> Self {
        Title{
            name: Rc::new(RefCell::new("Dummy".to_owned())),
            de_jure: None,
            de_facto: None,
            vassals: Vec::new(),
            history: HashMap::new()
        }
    }

    fn init(&mut self, base:Ref<'_, GameObject>, game_state:&mut GameState) {
        let de_jure_id = base.get("de_jure_liege");
        self.de_jure = match de_jure_id{
            Some(de_jure) => Some(game_state.get_title(de_jure.as_string_ref().unwrap().as_str()).clone()),
            None => None
        };
        let de_facto_id = base.get("de_facto_liege");
        self.de_facto = match de_facto_id{
            Some(de_facto) => Some(game_state.get_title(de_facto.as_string_ref().unwrap().as_str()).clone()),
            None => None
        };
        let mut vassals = Vec::new();
        let vas = base.get("vassals");
        if !vas.is_none(){
            for v in base.get_object_ref("vassals").get_array_iter(){
                vassals.push(game_state.get_title(v.as_string_ref().unwrap().as_str()).clone());
            }
        }
        self.vassals = vassals;
        let history = get_history(base, game_state);
        self.history = history;
    
    }
}

impl Serialize for Title {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Title", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("de_jure", &self.de_jure)?;
        state.serialize_field("de_facto", &self.de_facto)?;
        state.serialize_field("vassals", &self.vassals)?;
        state.serialize_field("history", &self.history)?;
        state.end()
    }
}

impl Renderable for Title {
    fn render(&self, env: &Environment, template_name: &'static String) -> String {
        let ctx = context! {title=>self};
        env.get_template(template_name).unwrap().render(&ctx).unwrap()   
    }
}