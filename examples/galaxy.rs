use relational_ecs::*;
use relational_ecs::prelude::*;
use crate::state::*;
use crate::entities::*;
use crate::components::*;

#[derive(Debug, Default)]
pub struct Galaxy {
    pub state: State,
    pub entities: entities::Allocators,
}

impl Split<Allocators, State> for Galaxy {
    fn split(&mut self) -> (&mut Allocators, &mut State) {
        (&mut self.entities, &mut self.state)
    }
}

pub mod state {
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
        pub location_body: EntityMap<LocationId, BodyId>,

        pub orbit_location: IndexedVec<OrbitId, LocationId>,
        pub orbit_radius: IndexedVec<OrbitId, Radius>,
        pub orbit_period: IndexedVec<OrbitId, Period>,
        pub orbit_angle_offset: IndexedVec<OrbitId, Angle>,
        pub orbit_relative_position: IndexedVec<OrbitId, Position>,
        pub orbit_parent: IndexedVec<OrbitId, Option<OrbitId>>,

        pub transit_location: IndexedVec<TransitId, LocationId>,
        pub transit_ends: IndexedVec<TransitId, Ends>,
        pub transit_duration: IndexedVec<TransitId, Seconds>,

        pub body_radius: IndexedVec<BodyId, Radius>,
        pub body_mass: IndexedVec<BodyId, Mass>,
        pub body_surface: EntityMap<BodyId, SurfaceId>,
        pub body_atmosphere: EntityMap<BodyId, AtmosphereId>,
        pub body_location: IndexedVec<BodyId, LocationId>,

        pub surface_body: IndexedVec<SurfaceId, BodyId>,
        pub surface_albedo: IndexedVec<SurfaceId, Albedo>,
        pub surface_area: IndexedVec<SurfaceId, Area>,

        pub atmosphere_body: IndexedVec<AtmosphereId, BodyId>,
        pub atmosphere_greenhouse: IndexedVec<AtmosphereId, Greenhouse>,
        pub atmosphere_pressure: IndexedVec<AtmosphereId, Pressure>,
    }

    pub type SystemRow = (String, LightYears);
    pub type LocationRow = (Position);

    pub struct OrbitRow {
        pub radius: Radius,
        pub period: Period,
        pub angle: Angle,
        pub parent: Option<OrbitId>,
    }

    pub struct TransitRow {
        pub ends: Ends,
        pub duration: Seconds
    }

    pub struct BodyRow {
        pub radius: Radius,
        pub mass: Mass
    }

    pub struct SurfaceRow {
        pub albedo: Albedo,
        pub area: Area
    }

    pub struct AtmosphereRow {
        pub greenhouse: Greenhouse,
        pub pressure: Pressure
    }

    impl<'a> OrbitRow {
        pub fn from_parent(radius: Radius, period: Period, angle: Angle, galaxy: &Galaxy, parent: BodyId) -> Self {
            let parent_orbit = galaxy.state
                .lookup2(parent, &galaxy.entities.bodies, &galaxy.entities.locations, &galaxy.entities.orbits)
                .expect("OrbitRow::from_parent: invalid parent id");

            OrbitRow {
                radius,
                period,
                angle,
                parent: Some(parent_orbit.entity),
            }
        }
    }

    link_to_many!(SystemId, system_locations, LocationId, location_system);

    link!(LocationId, location_transit, TransitId, transit_location);
    link!(LocationId, location_orbit, OrbitId, orbit_location);
    link!(LocationId, location_body, BodyId, body_location);

    link!(BodyId, body_surface, SurfaceId, surface_body);
    link!(BodyId, body_atmosphere, AtmosphereId, atmosphere_body);

    impl Insert<BodyId, BodyRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<BodyId>, value: BodyRow) {
            self.body_radius.insert(id, value.radius);
            self.body_mass.insert(id, value.mass);
        }
    }
    impl Create<'_, BodyId, BodyRow> for State {}

    impl Insert<SurfaceId, SurfaceRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<SurfaceId>, value: SurfaceRow) {
            self.surface_albedo.insert(id, value.albedo);
            self.surface_area.insert(id, value.area);
        }
    }
    impl Create<'_, SurfaceId, SurfaceRow> for State {}

    impl Insert<TransitId, TransitRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<TransitId>, value: TransitRow) {
            self.transit_ends.insert(id, value.ends);
            self.transit_duration.insert(id, value.duration);
        }
    }
    impl Create<'_, TransitId, TransitRow> for State {}

    impl Insert<AtmosphereId, AtmosphereRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<AtmosphereId>, value: AtmosphereRow) {
            self.atmosphere_greenhouse.insert(id, value.greenhouse);
            self.atmosphere_pressure.insert(id, value.pressure);
        }
    }
    impl Create<'_, AtmosphereId, AtmosphereRow> for State {}

    impl Insert<SystemId, SystemRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<SystemId>, value: (String, LightYears)) {
            self.system_name.insert(id, value.0);
            self.system_position.insert(id, value.1);
            self.system_locations.insert(id, EntitySet::new());
        }
    }
    impl Create<'_, SystemId, SystemRow> for State {}

    impl Insert<LocationId, LocationRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<LocationId>, value: LocationRow) {
            self.location_position.insert(id, value);
        }
    }
    impl Create<'_, LocationId, LocationRow> for State {}

    impl Insert<OrbitId, OrbitRow> for State {
        fn insert(&mut self, id: &VerifiedEntity<OrbitId>, value: OrbitRow) {
            self.orbit_radius.insert(id, value.radius);
            self.orbit_period.insert(id, value.period);
            self.orbit_angle_offset.insert(id, value.angle);
            self.orbit_relative_position.insert(id, Position::default());
            self.orbit_parent.insert(id, value.parent);
        }
    }
    impl Create<'_, OrbitId, OrbitRow> for State {}

    /// Complicated entity graphs constructed in one flat "layer", rather than a nested structure.
    /// This keeps allocator borrows simple
    /// And keeps traits like Create and Link generic
    pub struct Planet {
        pub system: SystemId,
        pub orbit: OrbitRow,
        pub body: BodyRow,
        pub surface: Option<SurfaceRow>,
        pub atmosphere: Option<AtmosphereRow>,
    }
    
    impl Construct<BodyId, Planet> for Galaxy {
        fn construct(&mut self, planet: Planet) -> BodyId {
            let (entities, state) = self.split();
            
            let system = entities.systems.verify(planet.system)
                .expect("invalid system id");

            if let Some(parent) = &planet.orbit.parent {
                let _parent = entities.orbits.verify(*parent)
                    .expect("invalid parent orbit");
            }

            let location = state.create_and_link(&system, Position::default(), &mut entities.locations);

            let _orbit = state.create_and_link(&location, planet.orbit, &mut entities.orbits);

            let body = state.create_and_link(&location, planet.body, &mut entities.bodies);

            if let Some(surface) = planet.surface {
                let _surface = state.create_and_link(&body, surface, &mut entities.surfaces);
            }

            if let Some(atmosphere) = planet.atmosphere {
                let _atmosphere = state.create_and_link(&body, atmosphere, &mut entities.atmospheres);
            }

            body.entity
        }
    }

    pub struct Moon {
        pub system: SystemId,
        pub orbit: OrbitRow,
        pub body: BodyRow,
        pub surface: SurfaceRow,
    }

    /// Similar entities with different requirements can be built from different types.
    /// Planet and Moon are both used to Construct a BodyId, but have different requirements.
    impl Construct<BodyId, Moon> for Galaxy {
        fn construct(&mut self, moon: Moon) -> BodyId {
            let system = self.entities.systems.verify(moon.system)
                .expect("invalid system id");

            // This requirement could be tested when initializing Moon, but is here for the sake of example.
            let parent = &moon.orbit.parent.expect("moons must have a parent orbit");
            let _parent = self.entities.orbits.verify(*parent)
                .expect("Planet::create - invalid parent orbit");

            let location = self.state.create_and_link(&system, Position::default(), &mut self.entities.locations);

            let _orbit = self.state.create_and_link(&location, moon.orbit, &mut self.entities.orbits);

            let body = self.state.create_and_link(&location, moon.body, &mut self.entities.bodies);

            let _surface = self.state.create_and_link(&body, moon.surface, &mut self.entities.surfaces);

            body.entity
        }
    }

    impl Remove<BodyId, SurfaceId> for State {
        fn remove(&mut self, id: &VerifiedEntity<BodyId>) {
            self.body_surface.remove(id);
        }
    }
    impl Delete<BodyId, SurfaceId> for State {}

    impl Remove<BodyId, AtmosphereId> for State {
        fn remove(&mut self, id: &VerifiedEntity<BodyId>) {
            self.body_atmosphere.remove(id);
        }
    }
    impl Delete<BodyId, AtmosphereId> for State {}

    impl Deconstruct<BodyId> for Galaxy {
        fn deconstruct(&mut self, id: BodyId) {
            let (e, s) = self.split();

            if let Some(body) = e.bodies.verify(id) {
                s.delete(&body, &mut e.surfaces);
                s.delete(&body, &mut e.atmospheres);
                e.bodies.kill(id);
            }
        }
    }

    pub struct OrbitPosition;

    impl Update<Galaxy> for OrbitPosition {
        fn update(galaxy: &mut Galaxy) {
            let (e, s) = galaxy.split();

            e.orbits.ids()
                .for_each(|orbit| {
                    update_relative_position(s, &orbit);
                });
        }
    }

    fn update_relative_position(_state: &mut State, _orbit: &VerifiedEntity<OrbitId>) {
        unimplemented!()
    }

    pub struct LocationPosition;

    impl Update<Galaxy> for LocationPosition {
        fn update(state: &mut Galaxy) {
            let (e, s) = state.split();

            s.location_orbit
                .verified_both(&e.locations, &e.orbits)
                .for_each(|(_loc, orbit)| {
                    let mut _position = s.orbit_relative_position[&orbit];
                    let mut orbit = orbit;

                    while let Some(parent) = s.orbit_parent[&orbit] {
                        if let Some(parent) = e.orbits.verify(parent) {
                            orbit = parent;
                            dbg!(&orbit);
                            unimplemented!()
                        }
                    }
                });
        }
    }
}

mod entities {
    use super::*;

    id_type!(SystemId);

    id_type!(LocationId);
    id_type!(OrbitId);
    id_type!(TransitId);

    id_type!(BodyId);
    id_type!(SurfaceId);
    id_type!(AtmosphereId);

    #[derive(Debug, Default)]
    pub struct Allocators {
        pub systems: Allocator<SystemId>,

        pub locations: Allocator<LocationId>,
        pub orbits: Allocator<OrbitId>,
        pub transits: Allocator<TransitId>,

        pub bodies: Allocator<BodyId>,
        pub surfaces: Allocator<SurfaceId>,
        pub atmospheres: Allocator<AtmosphereId>,
    }
}

mod components {
    use super::entities::LocationId;

    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct LightYears(f64, f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Mass(f64, f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Position(f64, f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Period(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Radius(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Angle(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Seconds(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Area(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Albedo(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Greenhouse(f64);
    #[derive(Debug, Default, Copy, Clone, PartialEq)] pub struct Pressure(f64);

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Ends {
        pub from: LocationId,
        pub to: LocationId
    }
}

fn main() {

}

//#[test]
//fn quick_test() {
//    let mut galaxy = Galaxy::default();
//
//    let system: SystemRow = ("Sol".to_string(), LightYears::default());
//    let system = galaxy.state.create(system, &mut galaxy.entities.systems).entity;
//
//    let center = LocationCreator {
//        system,
//        location: Position::default(),
//        orbit: None,
//        transit: None,
//    };
//    let center = center.create(&mut galaxy.state, &mut galaxy.entities);
//
//    {
//        let system = galaxy.entities.systems.verify(system).unwrap();
//        let center = galaxy.entities.locations.verify(center).unwrap();
//
//        assert_eq!(system.entity, galaxy.state.location_system[&center]);
//        assert_eq!(Some(&center.entity), galaxy.state.system_locations[&system].iter().nth(0));
//        assert!(galaxy.entities.orbits.ids().next().is_none());
//        assert!(galaxy.entities.transits.ids().next().is_none());
//    }
//}