use image::{
    codecs::gif::{GifEncoder, Repeat},
    Delay, Frame,
};

use serde::{ser::SerializeStruct, Serialize};

use super::super::{
    display::{Cullable, Localizer, Renderable, RenderableType, Renderer},
    game_object::{GameObject, GameString},
    game_state::GameState,
    jinja_env::H_TEMPLATE_NAME,
    types::Wrapper,
};

use super::{Character, FromGameObject, GameId, GameObjectDerived, LineageNode, Shared};
use std::{collections::HashMap, fs::File};

/// A struct representing a player in the game
pub struct Player {
    pub name: GameString,
    pub id: GameId,
    pub character: Option<Shared<Character>>,
    pub lineage: Vec<LineageNode>,
}

/// Gets the lineage of the player and appends it to the lineage vector
fn get_lineage(lineage: &mut Vec<LineageNode>, base: &GameObject, game_state: &mut GameState) {
    let lineage_node = base.get_object_ref("legacy");
    for leg in lineage_node.get_array_iter() {
        let o = leg.as_object().unwrap();
        lineage.push(LineageNode::from_game_object(o, game_state))
    }
}

impl FromGameObject for Player {
    fn from_game_object(base: &GameObject, game_state: &mut GameState) -> Self {
        let mut lineage: Vec<LineageNode> = Vec::new();
        get_lineage(&mut lineage, &base, game_state);
        let key = base.get("character").unwrap().as_id();
        Player {
            name: base.get("name").unwrap().as_string(),
            id: base.get("player").unwrap().as_id(),
            character: Some(game_state.get_character(&key).clone()),
            lineage: lineage,
        }
    }
}

impl GameObjectDerived for Player {
    fn get_id(&self) -> GameId {
        self.id
    }

    fn get_name(&self) -> GameString {
        self.name.clone()
    }
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Player", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("character", &self.character)?;
        state.serialize_field("lineage", &self.lineage)?;
        state.end()
    }
}

impl Renderable for Player {
    fn get_subdir() -> &'static str {
        "."
    }

    fn get_path(&self, path: &str) -> String {
        format!("{}/index.html", path)
    }

    fn get_template() -> &'static str {
        H_TEMPLATE_NAME
    }

    fn render_all(&self, stack: &mut Vec<RenderableType>, renderer: &mut Renderer) {
        renderer.render(self);
        let map = renderer.get_map();
        if map.is_some() {
            let game_state = renderer.get_state();
            //timelapse rendering
            let map = map.unwrap();
            let target_color = [70, 255, 70];
            let path = renderer.get_path().to_owned() + "/timelapse.gif";
            let mut file = File::create(&path).unwrap();
            let mut gif_encoder = GifEncoder::new(&mut file);
            for char in self.lineage.iter() {
                /* Note on timelapse:
                Paradox doesn't save any data regarding top level liege changes.
                Not even basic data that would allow us to reconstruct the map through implication.
                We would need something as basic as adding liege changes to history, or even just storing dead character's vassal relations
                I once had an idea that it could be possible to still have a timelapse by looking at dead vassals of the children of chars in lineage
                But that idea got stuck at the recursive step of that algorithm, and even so the result would have NO accuracy
                 */
                let char = char.get_character(); //this variable for no reason other than compiler bitching
                let char = char.get_internal();
                //we get the provinces held by the character and the vassals who died under their reign.
                //This is the closes approximation we can get of changes in the map that are 100% accurate
                let death_date = char.get_death_date();
                let date = if death_date.is_some() {
                    death_date.as_ref().unwrap().as_str()
                } else {
                    game_state.get_current_date().unwrap()
                };
                let fbytes = map.create_map_buffer(char.get_barony_keys(true), &target_color, date);
                //these variables cuz fbytes is moved
                let width = fbytes.width();
                let height = fbytes.height();
                let frame =
                    Frame::from_parts(fbytes, width, height, Delay::from_numer_denom_ms(3000, 1));
                gif_encoder.encode_frame(frame).unwrap();
            }
            gif_encoder.set_repeat(Repeat::Infinite).unwrap();
            // genetic similarity gradient rendering
            let last = self.lineage.last().unwrap().get_character();
            let title_iter = game_state.get_title_iter();
            let mut sim = HashMap::new();
            for (_, title) in title_iter {
                let title = title.get_internal();
                let key = title.get_key();
                if key.is_none() {
                    continue;
                }
                let key = key.unwrap();
                if !(key.starts_with("c_") || key.starts_with("b_")) {
                    continue;
                }
                let similarity;
                let ruler = title.get_holder();
                if ruler.is_none() {
                    similarity = 0.0;
                } else {
                    let ruler = ruler.as_ref().unwrap().get_internal();
                    similarity = ruler.dna_similarity(last.clone());
                }
                for barony in title.get_de_jure_barony_keys() {
                    if sim.contains_key(barony.as_ref())
                        && similarity < *sim.get(barony.as_ref()).unwrap()
                    {
                        continue;
                    }
                    sim.insert(barony.as_ref().clone(), similarity);
                }
            }
            map.create_map_graph(
                |key: &str| {
                    [
                        255,
                        255,
                        (255.0 * (1.0 - sim.get(&key.to_owned()).unwrap())) as u8,
                    ]
                },
                &format!("{}/sim.png", renderer.get_path()),
            );
            let mut titles = Vec::new();
            let first = self.lineage.first().unwrap().get_character();
            let first = first.get_internal();
            let dynasty = first.get_dynasty();
            let dynasty = dynasty.as_ref().unwrap().get_internal();
            let descendants = dynasty.get_founder().get_internal().get_descendants();
            for desc in descendants {
                let desc = desc.get_internal();
                titles.append(&mut desc.get_barony_keys(false));
            }
            map.create_map_file(
                titles,
                &target_color,
                &format!("{}/dynastyMap.png", renderer.get_path()),
                &format!("Lands of the {} dynasty", dynasty.get_name()),
            );
        }
        for char in self.lineage.iter() {
            char.get_character()
                .get_internal()
                .render_all(stack, renderer);
        }
        let grapher = renderer.get_grapher();
        if grapher.is_some() {
            let last = self.lineage.last().unwrap().get_character();
            grapher.unwrap().create_tree_graph::<Character>(
                last,
                true,
                &format!("{}/line.svg", renderer.get_path()),
            );
        }
    }
}

impl Cullable for Player {
    fn set_depth(&mut self, depth: usize, localization: &Localizer) {
        for node in self.lineage.iter_mut() {
            node.set_depth(depth, localization);
        }
    }

    fn get_depth(&self) -> usize {
        0
    }

    fn is_ok(&self) -> bool {
        true
    }
}
