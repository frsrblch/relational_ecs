use relational_ecs::prelude::*;
use relational_ecs::id_type;

id_type!(SheepId);
id_type!(CrookId);
id_type!(ShepherdId);

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Position(f32, f32);

impl Position {
    pub fn magnitude(&self) -> f32 {
        let magnitude_squared = self.0.powi(2) + self.1.powi(2);
        magnitude_squared.sqrt()
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Length(f32);

#[derive(Debug, Default)]
pub struct Game {
    pub state: State,
    pub entities: Entities,
}

#[derive(Debug, Default)]
pub struct State {
    pub shepherd_name: IndexedVec<ShepherdId, String>,
    pub shepherd_crook: IndexedVec<ShepherdId, CrookId>,
    pub shepherd_sheep: IndexedVec<ShepherdId, EntitySet<SheepId>>,

    pub crook_length: IndexedVec<CrookId, Length>,

    pub sheep_position: IndexedVec<SheepId, Position>,
    pub sheep_shepherd: IndexedVec<SheepId, ShepherdId>,
}

impl Insert<SheepId, ShepherdId> for State {
    fn insert(&mut self, id: &VerifiedEntity<SheepId>, value: ShepherdId) {
        self.sheep_shepherd.insert(id, value);
    }
}

impl Insert<ShepherdId, SheepId> for State {
    fn insert(&mut self, id: &VerifiedEntity<ShepherdId>, value: SheepId) {
        self.shepherd_sheep.get_mut(id).unwrap().insert(value);
    }
}

impl Remove<ShepherdId, SheepId> for State {
    fn remove(&mut self, id: &VerifiedEntity<ShepherdId>, value: SheepId) -> Option<SheepId> {
        self.shepherd_sheep.get_mut(id).unwrap().remove(&value)
    }
}

impl Link<ShepherdId, SheepId> for State {}

#[derive(Debug, Default)]
pub struct Entities {
    pub shepherds: Allocator<ShepherdId>,
    pub crooks: Allocator<CrookId>,
    pub sheep: Allocator<SheepId>,
}

impl State {
    pub fn create_shepherd<'a>(&mut self, name: &str, shepherds: &'a mut Allocator<ShepherdId>) -> VerifiedEntity<'a, ShepherdId> {
        let id = shepherds.create_entity();

        self.shepherd_name.insert(&id, String::from(name));
        self.shepherd_sheep.insert(&id, EntitySet::new());

        id
    }

    pub fn create_sheep<'a>(&mut self, position: Position, sheep: &'a mut Allocator<SheepId>) -> VerifiedEntity<'a, SheepId> {
        let id = sheep.create_entity();

        self.sheep_position.insert(&id, position);

        id
    }

    pub fn create_crook<'a>(&mut self, length: Length, crooks: &'a mut Allocator<CrookId>) -> VerifiedEntity<'a, CrookId> {
        let id = crooks.create_entity();

        self.crook_length.insert(&id, length);

        id
    }

    pub fn lose_all_sheep(&mut self, id: &VerifiedEntity<ShepherdId>, sheep: &mut Allocator<SheepId>) {
        let shepherds_sheep = self.shepherd_sheep.get_mut(id).unwrap();

        for s in shepherds_sheep.iter() {
            sheep.kill(*s);
        }

        shepherds_sheep.clear();
    }

    pub fn lose_even_sheep(&mut self, id: &VerifiedEntity<ShepherdId>, sheep: &mut Allocator<SheepId>) {
        let shepherds_sheep = self.shepherd_sheep.get_mut(id).unwrap();

        let sheep_to_remove = shepherds_sheep.iter()
            .copied()
            .enumerate()
            .filter_map(|(i, s)| if i % 2 == 0 {
                Some(s)
            } else {
                None
            })
            .collect::<Vec<_>>();

        for s in sheep_to_remove.iter() {
            shepherds_sheep.remove(s);
            sheep.kill(*s);
        }
    }

    pub fn lose_distant_sheep(&mut self, entities: &mut Entities) {
        let mut lost_sheep = Vec::new();

        for shepherd in entities.shepherds.ids() {
            let crook = self.shepherd_crook
                .get(&shepherd)
                .and_then(|c| entities.crooks.verify(*c))
                .unwrap();

            let length = self.crook_length.get(&crook).unwrap();

            for sheep in self.shepherd_sheep.get(&shepherd).unwrap().iter() {
                if let Some(sheep) = entities.sheep.verify(*sheep) {
                    let distance = self.sheep_position.get(&sheep).unwrap();
                    if distance.magnitude() > length.0 {
                        lost_sheep.push(sheep.entity);
                    }
                }
            }
        }

        for sheep in lost_sheep {
            let shepherd = *self.sheep_shepherd.get(&VerifiedEntity::assert_valid(sheep)).unwrap();
            self.remove(&VerifiedEntity::assert_valid(shepherd), sheep);
            entities.sheep.kill(sheep);
        }
    }

    pub fn count_sheep(&self, id: &VerifiedEntity<ShepherdId>) -> usize {
        self.shepherd_sheep.get(id).unwrap().len()
    }
}

pub struct Flock {
    shepherd: String,
    crook: Length,
    sheep: Vec<Position>,
}

impl Flock {
    pub fn create(self, state: &mut State, entities: &mut Entities) -> ShepherdId {
        let shepherd = state.create_shepherd(&self.shepherd, &mut entities.shepherds);

        let crook = state.create_crook(self.crook, &mut entities.crooks);
        state.shepherd_crook.insert(&shepherd, crook.entity);

        for position in self.sheep.iter() {
            let sheep = state.create_sheep(*position, &mut entities.sheep);
            state.link(&shepherd, &sheep);
        }

        shepherd.entity
    }
}

fn main() {
    let mut game = Game::default();

    let shepherd = Flock {
        shepherd: String::from("Little Bo-Peep"),
        crook: Length(1.25),
        sheep: vec![Position(0.0, 0.0), Position(1.0, 0.0), Position(0.0, 1.0), Position(1.0, 1.0)],
    };

    let shepherd = shepherd.create(&mut game.state, &mut game.entities);
    let little_bo_peep = game.entities.shepherds.verify(shepherd).unwrap();

    assert_eq!(4, game.state.count_sheep(&little_bo_peep));
    assert_eq!(4, game.entities.sheep.ids().collect::<Vec<_>>().len());
//    drop(little_bo_peep);

    game.state.lose_distant_sheep(&mut game.entities);

    let little_bo_peep = game.entities.shepherds.verify(shepherd).unwrap();
    assert_eq!(3, game.state.count_sheep(&little_bo_peep));
    assert_eq!(3, game.entities.sheep.ids().collect::<Vec<_>>().len());
}