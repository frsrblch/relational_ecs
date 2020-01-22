use relational_ecs::storage::Component;
use relational_ecs::traits::Split;
use relational_ecs::allocators::*;
use relational_ecs::traits_new::*;
use relational_ecs::ids::*;

fn main() {
    let mut state = Game::default();
    let (state, ids) = state.split();

    let system = SystemRow {
        name: "Sol".to_string(),
        position: Default::default()
    };
    let system: Id<System> = state.system.create(system, &mut ids.systems);

    let body = BodyRow {
        system,
        name: "Earth".to_string(),
        position: Position(0.0, 149.6e9),
        mass: Mass(5.972e24),
    };
    let body: Id<Body> = state.body.create(body, &mut ids.bodies);

    let colony = ColonyRow {
        body,
        name: "Humanity".to_string(),
        population: Population(7.8e9),
    };
    let _colony: GenId<Colony> = state.colony.create(colony, &mut ids.colonies).id();

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
    pub bodies: FixedAllocator<Body>,
    pub colonies: FlexAllocator<Colony>,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub system: System,
    pub body: Body,
    pub colony: Colony,
}

impl State {
    pub fn get_colony_system(&self, colony: &Valid<Colony>) -> Id<System> {
        let body = self.colony.body.get(colony);
        *self.body.system.get(body)
    }
}

#[derive(Debug, Default, Clone)]
pub struct System {
    pub name: Component<Self, String>,
    pub position: Component<Self, Position>,
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
    fn insert(&mut self, id: &Id<Self>, value: SystemRow) {
        self.name.insert(id, value.name);
        self.position.insert(id, value.position);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Body {
    pub system: Component<Self, Id<System>>,
    pub name: Component<Self, String>,
    pub position: Component<Self, Position>,
    pub mass: Component<Self, Mass>,
}

impl Arena<'_> for Body {
    type Row = BodyRow;
    type Allocator = FixedAllocator<Self>;
}

pub struct BodyRow {
    pub system: Id<System>,
    pub name: String,
    pub position: Position,
    pub mass: Mass,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Position(f64, f64);

#[derive(Debug, Default, Copy, Clone)]
pub struct Mass(f64);

impl Insert<'_> for Body {
    fn insert(&mut self, id: &Id<Self>, value: BodyRow) {
        self.name.insert(id, value.name);
        self.position.insert(id, value.position);
        self.mass.insert(id, value.mass);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Colony {
    pub body: Component<Self, Id<Body>>,
    pub name: Component<Self, String>,
    pub population: Component<Self, Population>,
}

pub struct ColonyRow {
    pub body: Id<Body>,
    pub name: String,
    pub population: Population,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Population(f64);

impl Arena<'_> for Colony {
    type Row = ColonyRow;
    type Allocator = FlexAllocator<Self>;
}

impl Insert<'_> for Colony {
    fn insert(&mut self, id: &Valid<Self>, value: Self::Row) {
        self.name.insert(id, value.name);
        self.body.insert(id, value.body);
        self.population.insert(id, value.population);
    }
}