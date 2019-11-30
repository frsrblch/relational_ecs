use relational_ecs::id_type;
use relational_ecs::prelude::*;

#[derive(Debug, Default)]
pub struct Galaxy {
    pub state: State,
    pub entities: Entities,
}

#[derive(Debug, Default)]
pub struct State {
    pub system_name: IndexedVec<SystemId, String>,
    pub system_position: IndexedVec<SystemId, LightYears>,
    pub system_locations: IndexedVec<SystemId, EntitySet<LocationId>>,

    pub location_system: IndexedVec<LocationId, SystemId>,
    pub location_position: IndexedVec<LocationId, Meters>,

}

type SystemRow = (String, LightYears);

impl Insert<SystemId, SystemRow> for State {
    fn insert(&mut self, id: &VerifiedEntity<SystemId>, value: (String, LightYears)) {
        self.system_name.insert(id, value.0);
        self.system_position.insert(id, value.1);
    }
}

impl<'a> Create<'a, SystemId, SystemRow> for State {}

impl Insert<SystemId, LocationId> for State {
    fn insert(&mut self, id: &VerifiedEntity<SystemId>, value: LocationId) {
        let locations = &mut self.system_locations[id];
        locations.insert(value);
    }
}

impl Remove<SystemId, LocationId> for State {
    fn remove(&mut self, id: &VerifiedEntity<SystemId>, value: LocationId) -> Option<LocationId> {
        let locations = &mut self.system_locations[id];
        locations.remove(&value)
    }
}

impl Insert<LocationId, SystemId> for State {
    fn insert(&mut self, id: &VerifiedEntity<LocationId>, value: SystemId) {
        self.location_system.insert(id, value);
    }
}

impl Link<SystemId, LocationId> for State {}

id_type!(SystemId);

id_type!(LocationId);
id_type!(OrbitId);

id_type!(CelestialId);
id_type!(SurfaceId);
id_type!(AtmosphereId);

#[derive(Debug, Default)]
pub struct Entities {
    pub systems: Allocator<SystemId>,

    pub locations: Allocator<LocationId>,
    pub orbits: Allocator<OrbitId>,

    pub celestials: Allocator<CelestialId>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LightYears(f32, f32);

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Meters(f32, f32);

fn main() {

}