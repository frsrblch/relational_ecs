use relational_ecs::storage::Component;
use relational_ecs::traits::Split;
use relational_ecs::allocators::*;
use relational_ecs::traits_new::*;

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
    type Allocator = FixedAllocator<Self>;
}

#[derive(Debug, Default, Clone)]
pub struct Body {
    pub name: Component<Body, String>,
    pub position: Component<Body, Position>,
    pub mass: Component<Body, Mass>,
}

impl Arena<'_> for Body {
    type Allocator = FixedAllocator<Self>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Position(f64, f64);

#[derive(Debug, Default, Copy, Clone)]
pub struct Mass(f64);

pub struct SystemRow {
    pub name: String,
    pub position: Position,
}

impl Insert<SystemRow> for System {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: SystemRow) {
        self.name.insert(id, value.name);
        self.position.insert(id, value.position);
    }
}

impl Create<'_, System, SystemRow> for System {}

pub struct BodyRow {
    pub name: String,
    pub position: Position,
    pub mass: Mass,
}

impl Insert<BodyRow> for Body {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: BodyRow) {
        self.name.insert(id, value.name);
        self.position.insert(id, value.position);
        self.mass.insert(id, value.mass);
    }
}

impl Create<'_, Body, BodyRow> for Body {}