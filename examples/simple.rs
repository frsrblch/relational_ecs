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

impl Split<Entities, State> for Game {
    fn split(&mut self) -> (&mut Entities, &mut State) {
        (&mut self.entities, &mut self.state)
    }
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

    pub crook_length: IndexedVec<CrookId, Length>,
    pub crook_material: IndexedVec<CrookId, Material>,

    pub sheep_shepherd: IndexedVec<SheepId, ShepherdId>,
    pub sheep_position: IndexedVec<SheepId, Position>,
    pub sheep_wool: IndexedVec<SheepId, Wool>,

    pub lost_sheep: EntitySet<SheepId>,
}

impl Insert<ShepherdId, ShepherdRow> for State {
    fn insert(&mut self, id: &VerifiedEntity<ShepherdId>, value: String) {
        self.shepherd_name.insert(id, value);
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

impl Link<ShepherdId, SheepId> for State {
    fn link(&mut self, a: &VerifiedEntity<ShepherdId>, b: &VerifiedEntity<SheepId>) {
        self.insert(b, a.entity);
    }
}

#[derive(Debug, Default)]
pub struct Entities {
    pub shepherds: Allocator<ShepherdId>,
    pub crooks: Allocator<CrookId>,
    pub sheep: Allocator<SheepId>,
}

impl State {
    pub fn distant_sheep_become_lost(&mut self, entities: &mut Entities) {
        for (shepherd, crook) in self.shepherd_crook.verified_both(&entities.shepherds, &entities.crooks) {
            let length = self.crook_length[&crook];

            for sheep in get_shepherds_sheep(&self.sheep_shepherd, &shepherd, &entities.sheep) {
                let distance = self.sheep_position[&sheep];
                if distance.magnitude() > length.0 {
                    self.lost_sheep.insert(sheep.entity);
                }
            }
        }
    }

    pub fn remove_lost_sheep(&mut self, entities: &mut Entities) {
        for sheep in self.lost_sheep.values.drain() {
            entities.sheep.kill(sheep);
        }
    }

    pub fn count_sheep(&self, id: &VerifiedEntity<ShepherdId>, sheep: &Allocator<SheepId>) -> usize {
        get_shepherds_sheep(&self.sheep_shepherd, id, sheep).count()
    }
}

fn get_shepherds_sheep<'a>(
    sheep_shepherd: &'a IndexedVec<SheepId, ShepherdId>,
    id: &'a VerifiedEntity<ShepherdId>,
    sheep: &'a Allocator<SheepId>,
) -> impl Iterator<Item=VerifiedEntity<'a, SheepId>> {
    sheep
        .ids()
        .filter(move |sheep| sheep_shepherd[sheep] == id.entity)
}

pub struct Flock {
    shepherd: String,
    crook: CrookRow,
    sheep: Vec<SheepRow>,
}

impl Construct<ShepherdId, Flock> for Game {
    fn construct(&mut self, flock: Flock) -> ShepherdId {
        let (entities, state) = self.split();

        let shepherd = state.create(flock.shepherd, &mut entities.shepherds);

        let crook = state.create(flock.crook, &mut entities.crooks);
        state.shepherd_crook.insert(&shepherd, crook.entity);

        for row in flock.sheep.into_iter() {
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

    let shepherd = game.construct(shepherd);

    let little_bo_peep = game.entities.shepherds.verify(shepherd).unwrap();
    assert_eq!(4, game.state.count_sheep(&little_bo_peep, &game.entities.sheep));
    assert_eq!(4, game.entities.sheep.ids().collect::<Vec<_>>().len());

    game.state.distant_sheep_become_lost(&mut game.entities);
    game.state.remove_lost_sheep(&mut game.entities);

    let little_bo_peep = game.entities.shepherds.verify(shepherd).unwrap();
    assert_eq!(3, game.state.count_sheep(&little_bo_peep, &game.entities.sheep));
    assert_eq!(3, game.entities.sheep.ids().collect::<Vec<_>>().len());
}