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

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Wool(f32);

#[derive(Debug, Default)]
pub struct Game {
    pub state: State,
    pub entities: Entities,
}

type ShepherdRow = String;

type SheepRow = (Position, Wool);

#[derive(Debug)]
pub enum Material { Wood, Metal }

type CrookRow = (Length, Material);

#[derive(Debug, Default)]
pub struct State {
    pub shepherd_name: IndexedVec<ShepherdId, String>,
    pub shepherd_crook: IndexedVec<ShepherdId, CrookId>,
    pub shepherd_sheep: IndexedVec<ShepherdId, EntitySet<SheepId>>,

    pub crook_length: IndexedVec<CrookId, Length>,
    pub crook_material: IndexedVec<CrookId, Material>,

    pub sheep_position: IndexedVec<SheepId, Position>,
    pub sheep_wool: IndexedVec<SheepId, Wool>,
    pub sheep_shepherd: IndexedVec<SheepId, ShepherdId>,
}

impl Insert<ShepherdId, ShepherdRow> for State {
    fn insert(&mut self, id: &VerifiedEntity<ShepherdId>, value: String) {
        self.shepherd_name.insert(id, value);
        self.shepherd_sheep.insert(id, EntitySet::new());
    }
}

impl<'a> Create<'a, ShepherdId, ShepherdRow> for State {}

impl Insert<CrookId, CrookRow> for State {
    fn insert(&mut self, id: &VerifiedEntity<CrookId>, value: (Length, Material)) {
        self.crook_length.insert(id, value.0);
        self.crook_material.insert(id, value.1);
    }
}

impl<'a> Create<'a, CrookId, CrookRow> for State {}

impl Insert<SheepId, ShepherdId> for State {
    fn insert(&mut self, id: &VerifiedEntity<SheepId>, value: ShepherdId) {
        self.sheep_shepherd.insert(id, value);
    }
}

impl Insert<SheepId, SheepRow> for State {
    fn insert(&mut self, id: &VerifiedEntity<SheepId>, value: (Position, Wool)) {
        self.sheep_position.insert(id, value.0);
        self.sheep_wool.insert(id, value.1);
    }
}

impl<'a> Create<'a, SheepId, SheepRow> for State {}

impl Insert<ShepherdId, SheepId> for State {
    fn insert(&mut self, id: &VerifiedEntity<ShepherdId>, value: SheepId) {
        let sheep = &mut self.shepherd_sheep[id];
        sheep.insert(value);
    }
}

impl Remove<ShepherdId, SheepId> for State {
    fn remove(&mut self, id: &VerifiedEntity<ShepherdId>, value: SheepId) -> Option<SheepId> {
        let sheep = &mut self.shepherd_sheep[id];
        sheep.remove(&value)
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
    pub fn lose_distant_sheep(&mut self, entities: &mut Entities) {
        let mut lost_sheep = Vec::new();

        for shepherd in entities.shepherds.ids() {
            let crook = self.shepherd_crook[&shepherd];
            let crook = entities.crooks.verify(crook).unwrap();

            let length = self.crook_length[&crook];

            for sheep in self.shepherd_sheep[&shepherd].iter() {
                if let Some(sheep) = entities.sheep.verify(*sheep) {
                    let distance = self.sheep_position[&sheep];
                    if distance.magnitude() > length.0 {
                        lost_sheep.push(sheep.entity);
                    }
                }
            }
        }

        for sheep in lost_sheep {
            let shepherd = self.sheep_shepherd[&VerifiedEntity::assert_valid(sheep)];
            self.remove(&VerifiedEntity::assert_valid(shepherd), sheep);
            entities.sheep.kill(sheep);
        }
    }

    pub fn count_sheep(&self, id: &VerifiedEntity<ShepherdId>) -> usize {
        self.shepherd_sheep[id].len()
    }
}

pub struct Flock {
    shepherd: String,
    crook: CrookRow,
    sheep: Vec<SheepRow>,
}

impl Flock {
    pub fn create(self, state: &mut State, entities: &mut Entities) -> ShepherdId {
        let shepherd = state.create(self.shepherd, &mut entities.shepherds);

        let crook = state.create(self.crook, &mut entities.crooks);
        state.shepherd_crook.insert(&shepherd, crook.entity);

        for row in self.sheep.into_iter() {
            let sheep = state.create(row, &mut entities.sheep);
            state.link(&shepherd, &sheep);
        }

        shepherd.entity
    }
}

fn main() {
    let mut game = Game::default();

    let shepherd = Flock {
        shepherd: String::from("Little Bo-Peep"),
        crook: (Length(1.25), Material::Wood),
        sheep: vec![
            (Position(0.0, 0.0), Wool(0.2)),
            (Position(1.0, 0.0), Wool(1.2)),
            (Position(0.0, 1.0), Wool(0.8)),
            (Position(1.0, 1.0), Wool(1.8))
        ]
    };

    let shepherd = shepherd.create(&mut game.state, &mut game.entities);

    let little_bo_peep = game.entities.shepherds.verify(shepherd).unwrap();
    assert_eq!(4, game.state.count_sheep(&little_bo_peep));
    assert_eq!(4, game.entities.sheep.ids().collect::<Vec<_>>().len());

    game.state.lose_distant_sheep(&mut game.entities);

    let little_bo_peep = game.entities.shepherds.verify(shepherd).unwrap();
    assert_eq!(3, game.state.count_sheep(&little_bo_peep));
    assert_eq!(3, game.entities.sheep.ids().collect::<Vec<_>>().len());
}