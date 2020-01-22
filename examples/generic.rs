use relational_ecs::storage::Component;
use relational_ecs::traits::{Split};
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

    // TODO for compound types, the Construct trait is used to instantiate several ids at once
    let body = BodyRow {
        name: "Earth".to_string(),
        position: Position(0.0, 149.6e9),
        mass: Mass(5.972e24),
    };
    let body: Id<Body> = state.body.create(body, &mut ids.bodies);
    Link::<System, Body>::link(state, &system, &body);


    // for simple links, the row can contain the parent id
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
    pub surfaces: FixedAllocator<Surface>,
    pub atmospheres: FixedAllocator<Atmosphere>,
    pub colonies: GenAllocator<Colony>,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub system: System,
    pub body: Body,
    pub colony: Colony,
    pub surface: Surface,
    pub atmosphere: Atmosphere,
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

    // TODO add optional links to surface and atmosphere
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
    type Allocator = GenAllocator<Self>;
}

impl Insert<'_> for Colony {
    fn insert(&mut self, id: &Valid<Self>, value: Self::Row) {
        self.name.insert(id, value.name);
        self.body.insert(id, value.body);
        self.population.insert(id, value.population);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Surface {
    pub body: Component<Self, Id<Body>>,
    pub area: Component<Self, f64>,
    pub temperature: Component<Self, f64>,
}

pub struct SurfaceRow {
    pub area: f64,
    pub temperature: f64,
}

impl Arena<'_> for Surface {
    type Row = SurfaceRow;
    type Allocator = FixedAllocator<Self>;
}

impl Insert<'_> for Surface {
    fn insert(&mut self, id: &Id<Self>, value: Self::Row) {
        self.area.insert(id, value.area);
        self.temperature.insert(id, value.temperature);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Atmosphere {
    pub body: Component<Self, Id<Body>>,
    pub breathable: Component<Self, bool>,
    pub greenhouse_effect: Component<Self, f64>,
}

pub struct AtmosphereRow {
    pub breathable: bool,
    pub greenhouse_effect: f64,
}

impl Arena<'_> for Atmosphere {
    type Row = AtmosphereRow;
    type Allocator = FixedAllocator<Self>;
}

impl Insert<'_> for Atmosphere {
    fn insert(&mut self, id: &Id<Self>, value: Self::Row) {
        self.breathable.insert(id, value.breathable);
        self.greenhouse_effect.insert(id, value.greenhouse_effect);
    }
}

pub struct Planet {
    pub system: Id<System>,
    pub body: BodyRow,
    pub surface: Option<SurfaceRow>,
    pub atmosphere: Option<AtmosphereRow>,
}

impl Link<'_, System, Body> for State {
    fn link(&mut self, a: &Id<System>, b: &Id<Body>) {
        self.body.system.insert(b, *a);
    }
}

impl Link<'_, Body, Surface> for State {
    fn link(&mut self, a: &Id<Body>, b: &Id<Surface>) {
        self.surface.body.insert(b, *a);
    }
}

impl Link<'_, Body, Atmosphere> for State {
    fn link(&mut self, a: &Id<Body>, b: &Id<Atmosphere>) {
        self.atmosphere.body.insert(b, *a);
    }
}

impl Construct<Body, Planet> for Game {
    type Id = Id<Body>;

    fn construct(&mut self, value: Planet) -> Self::Id {
        let (state, ids) = self.split();

        // TODO add CreateAndLink trait
        let body: Id<Body> = state.body.create(value.body, &mut ids.bodies);

        Link::<System, Body>::link(state, &value.system, &body);

        if let Some(surface) = value.surface {
            let surface = state.surface.create(surface, &mut ids.surfaces);
            Link::<Body, Surface>::link(state, &body, &surface);
        }

        if let Some(atmosphere) = value.atmosphere {
            let atmosphere = state.atmosphere.create(atmosphere, &mut ids.atmospheres);
            Link::<Body, Atmosphere>::link(state, &body, &atmosphere);
        }

        body
    }
}