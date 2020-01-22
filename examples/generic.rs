use relational_ecs::storage::Component;
use relational_ecs::traits::Split;
use relational_ecs::allocators::*;
use relational_ecs::traits_new::*;
use relational_ecs::ids::Id;

fn main() {
//    let mut state = Game::default();
//    let (state, ids) = state.split();


}

#[derive(Debug, Default, Clone)]
pub struct Game {
    pub state: State,
    pub ids: Ids,
}

impl Split<State, Ids> for Game {
    fn split(&mut self) -> (&mut State, &mut Ids) {
        (&mut self.state, &mut self.ids)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ids {
    pub systems: FixedAllocator<System>,
    pub bodies: FlexAllocator<Body>,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub system: System,
    pub body: Body,
}

#[derive(Debug, Default, Clone)]
pub struct System {
    pub name: Component<System, String>,
    pub position: Component<System, Position>,
}

impl Arena<'_> for System {
    type Row = SystemRow;
    type Allocator = FixedAllocator<Self>;
}
pub struct SystemRow {
    pub name: String,
    pub position: Position,
}

impl Insert<'_> for System {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: SystemRow) {
        self.name.insert(id, value.name);
        self.position.insert(id, value.position);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Body {
    pub name: Component<Body, String>,
    pub position: Component<Body, Position>,
    pub mass: Component<Body, Mass>,
}

impl Arena<'_> for Body {
    type Row = BodyRow;
    type Allocator = FixedAllocator<Self>;
}

pub struct BodyRow {
    pub name: String,
    pub position: Position,
    pub mass: Mass,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Position(f64, f64);

#[derive(Debug, Default, Copy, Clone)]
pub struct Mass(f64);

impl Insert<'_> for Body {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: BodyRow) {
        self.name.insert(id, value.name);
        self.position.insert(id, value.position);
        self.mass.insert(id, value.mass);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Colony {
    pub name: Component<Self, String>,
    pub body: Component<Self, Id<Body>>,
    pub population: Component<Self, Population>,
}

pub struct ColonyRow {
    pub name: String,
    pub body: Id<Body>,
    pub population: Population,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Population(f64);

impl Arena<'_> for Colony {
    type Row = ColonyRow;
    type Allocator = FlexAllocator<Self>;
}

impl Insert<'_> for Colony {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: Self::Row) {
        self.name.insert(id, value.name);
        self.body.insert(id, value.body);
        self.population.insert(id, value.population);
    }
}