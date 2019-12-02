use relational_ecs::id_type;
use relational_ecs::prelude::*;
use crate::state::*;
use crate::entities::*;
use crate::components::*;

#[derive(Debug, Default)]
pub struct Galaxy {
    pub state: State,
    pub entities: entities::Allocators,
}

mod state {
    use super::*;

    #[derive(Debug, Default)]
    pub struct State {
        pub system_name: IndexedVec<SystemId, String>,
        pub system_position: IndexedVec<SystemId, LightYears>,
        pub system_locations: IndexedVec<SystemId, EntitySet<LocationId>>,

        pub location_system: IndexedVec<LocationId, SystemId>,
        pub location_position: IndexedVec<LocationId, Position>,
        pub location_orbit: EntityMap<LocationId, OrbitId>,
        pub location_transit: EntityMap<LocationId, TransitId>,

        pub orbit_location: IndexedVec<OrbitId, LocationId>,
        pub orbit_radius: IndexedVec<OrbitId, Radius>,
        pub orbit_period: IndexedVec<OrbitId, Period>,
        pub orbit_angle_offset: IndexedVec<OrbitId, Angle>,
        pub orbit_relative_position: IndexedVec<OrbitId, Position>,

        pub transit_location: IndexedVec<TransitId, LocationId>,
        pub transit_ends: IndexedVec<TransitId, Ends>,
        pub transit_duration: IndexedVec<TransitId, Seconds>,
    }

    pub type SystemRow = (String, LightYears);
    pub type LocationRow = (Position);
    pub type OrbitRow = (Radius, Period, Angle);
    pub struct TransitRow { pub ends: Ends, pub duration: Seconds }

    impl Insert<TransitId, TransitRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<TransitId>, value: TransitRow) {
            self.transit_ends.insert(id, value.ends);
            self.transit_duration.insert(id, value.duration);
        }
    }

    impl<'a> Create<'a, TransitId, TransitRow> for State {}

    impl Insert<TransitId, LocationId> for State {
        fn insert(&mut self, id: &VerifiedEntity<TransitId>, value: LocationId) {
            self.transit_location.insert(id, value);
        }
    }

    impl Insert<LocationId, TransitId> for State {
        fn insert(&mut self, id: &VerifiedEntity<LocationId>, value: TransitId) {
            self.location_transit.insert(id, value);
        }
    }

    impl Link<LocationId, TransitId> for State {}

    impl Insert<SystemId, SystemRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<SystemId>, value: (String, LightYears)) {
            self.system_name.insert(id, value.0);
            self.system_position.insert(id, value.1);
            self.system_locations.insert(id, EntitySet::new());
        }
    }

    impl<'a> Create<'a, SystemId, SystemRow> for State {}

    impl Insert<LocationId, LocationRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<LocationId>, value: LocationRow) {
            self.location_position.insert(id, value);
        }
    }

    impl<'a> Create<'a, LocationId, LocationRow> for State {}

    impl Insert<SystemId, LocationId> for State {
        fn insert(&mut self, id: &VerifiedEntity<SystemId>, value: LocationId) {
            let locations = &mut self.system_locations[id];
            locations.insert(value);
        }
    }

    impl Remove<SystemId, LocationId> for State {
        fn remove(&mut self, id: &VerifiedEntity<SystemId>, value: LocationId) -> Option<LocationId> {
            let values = &mut self.system_locations[id];
            values.remove(&value)
        }
    }

    impl Insert<LocationId, SystemId> for State {
        fn insert(&mut self, id: &VerifiedEntity<LocationId>, value: SystemId) {
            self.location_system.insert(id, value);
        }
    }

    impl Link<SystemId, LocationId> for State {}

    impl Insert<LocationId, OrbitId> for State {
        fn insert(&mut self, id: &VerifiedEntity<LocationId>, value: OrbitId) {
            self.location_orbit.insert(id, value);
        }
    }

    impl Insert<OrbitId, LocationId> for State {
        fn insert(&mut self, id: &VerifiedEntity<OrbitId>, value: LocationId) {
            self.orbit_location.insert(id, value);
        }
    }

    impl Link<LocationId, OrbitId> for State {}

    impl Insert<OrbitId, OrbitRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<OrbitId>, value: (Radius, Period, Angle)) {
            self.orbit_radius.insert(id, value.0);
            self.orbit_period.insert(id, value.1);
            self.orbit_angle_offset.insert(id, value.2);
            self.orbit_relative_position.insert(id, Default::default());
        }
    }

    impl<'a> Create<'a, OrbitId, OrbitRow> for State {}

    pub struct LocationCreator {
        pub system: SystemId,
        pub location: LocationRow,
        pub orbit: Option<OrbitRow>,
        pub transit: Option<TransitRow>,
    }

    impl LocationCreator {
        pub fn create(self, state: &mut State, allocators: &mut Allocators) -> LocationId {
            let system = allocators.systems.verify(self.system).expect("LocationCreator: invalid system id");

            let location = state.create(self.location, &mut allocators.locations);

            state.link(&system, &location);

            if let Some(orbit) = self.orbit {
                let orbit = state.create(orbit, &mut allocators.orbits);
                state.link(&location, &orbit);
            }

            if let Some(transit) = self.transit {
                let transit = state.create(transit, &mut allocators.transits);
                state.link(&location, &transit);
            }

            location.entity
        }
    }
}

mod entities {
    use super::*;

    id_type!(SystemId);

    id_type!(LocationId);
    id_type!(OrbitId);
    id_type!(TransitId);

    #[derive(Debug, Default)]
    pub struct Allocators {
        pub systems: Allocator<SystemId>,

        pub locations: Allocator<LocationId>,
        pub orbits: Allocator<OrbitId>,
        pub transits: Allocator<TransitId>,
    }
}

mod components {
    use super::entities::LocationId;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct LightYears(f64, f64);

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Position(f64, f64);

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Period(f64);

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Radius(f64);

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Angle(f64);

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Seconds(f64);

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Ends(LocationId, LocationId);
}

fn main() {

}

#[test]
fn quick_test() {
    let mut galaxy = Galaxy::default();

    let system: SystemRow = ("Sol".to_string(), LightYears::default());
    let system = galaxy.state.create(system, &mut galaxy.entities.systems).entity;

    let center = LocationCreator {
        system,
        location: Position::default(),
        orbit: None,
        transit: None,
    };
    let center = center.create(&mut galaxy.state, &mut galaxy.entities);

    {
        let system = galaxy.entities.systems.verify(system).unwrap();
        let center = galaxy.entities.locations.verify(center).unwrap();

        assert_eq!(system.entity, galaxy.state.location_system[&center]);
        assert_eq!(Some(&center.entity), galaxy.state.system_locations[&system].iter().nth(0));
        assert!(galaxy.entities.orbits.ids().next().is_none());
        assert!(galaxy.entities.transits.ids().next().is_none());
    }
}