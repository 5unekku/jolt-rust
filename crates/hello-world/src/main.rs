// someday:
// #![forbid(unsafe_code)]

// everything prefixed with `JPC_` comes from the joltc_sys crate
use joltc_sys::*;

use rolt::{
    box_shape, sphere_shape, BodyCreationSettings, BroadPhaseLayer, BroadPhaseLayerInterface,
    CastShapeArgs, CastShapeCollectorImpl, ClosestHitCastShapeCollector, IntoJolt, ObjectLayer,
    ObjectLayerPairFilter, ObjectVsBroadPhaseLayerFilter, RShapeCast, RVec3, TempAllocator,
    ThreadPoolJobSystem, Vec3, JPC_MOTION_TYPE_DYNAMIC, JPC_MOTION_TYPE_STATIC,
};

const OL_NON_MOVING: JPC_ObjectLayer = 0;
const OL_MOVING: JPC_ObjectLayer = 1;

const BPL_NON_MOVING: JPC_BroadPhaseLayer = 0;
const BPL_MOVING: JPC_BroadPhaseLayer = 1;
const BPL_COUNT: JPC_BroadPhaseLayer = 2;

struct BroadPhaseLayers;

impl BroadPhaseLayerInterface for BroadPhaseLayers {
    fn get_num_broad_phase_layers(&self) -> u32 {
        BPL_COUNT as u32
    }

    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer {
        match layer.raw() {
            OL_NON_MOVING => BroadPhaseLayer::new(BPL_NON_MOVING),
            OL_MOVING => BroadPhaseLayer::new(BPL_MOVING),
            _ => unreachable!(),
        }
    }
}

struct ObjectVsBroadPhase;

impl ObjectVsBroadPhaseLayerFilter for ObjectVsBroadPhase {
    fn should_collide(&self, layer1: ObjectLayer, layer2: BroadPhaseLayer) -> bool {
        match layer1.raw() {
            OL_NON_MOVING => layer2.raw() == BPL_MOVING,
            OL_MOVING => true,
            _ => unreachable!(),
        }
    }
}

struct ObjectLayerPair;

impl ObjectLayerPairFilter for ObjectLayerPair {
    fn should_collide(&self, layer1: ObjectLayer, layer2: ObjectLayer) -> bool {
        match layer1.raw() {
            OL_NON_MOVING => layer2.raw() == OL_MOVING,
            OL_MOVING => true,
            _ => unreachable!(),
        }
    }
}

fn main() {
    rolt::register_default_allocator();
    rolt::factory_init();
    rolt::register_types();

    let temp_allocator = TempAllocator::new(10 * 1024 * 1024);
    let job_system =
        ThreadPoolJobSystem::new(JPC_MAX_PHYSICS_JOBS as _, JPC_MAX_PHYSICS_BARRIERS as _);

    let mut physics_system = rolt::PhysicsSystem::new();
    physics_system.init(
        1024,
        0,
        1024,
        1024,
        BroadPhaseLayers,
        ObjectVsBroadPhase,
        ObjectLayerPair,
    );

    let body_interface = physics_system.body_interface();

    let floor_shape = box_shape(Vec3::new(100.0, 1.0, 100.0), 0.0).unwrap();
    let mut floor_settings = BodyCreationSettings::new(
        &floor_shape,
        ObjectLayer::new(OL_NON_MOVING),
        JPC_MOTION_TYPE_STATIC,
    );
    floor_settings.set_position(RVec3::new(0.0, -1.0, 0.0));
    let floor = body_interface.create_body(&floor_settings).unwrap();
    let floor_id = floor.id();
    body_interface.add_body(floor_id, JPC_ACTIVATION_DONT_ACTIVATE);

    let sphere_shape = sphere_shape(0.5).unwrap();
    let mut sphere_settings = BodyCreationSettings::new(
        &sphere_shape,
        ObjectLayer::new(OL_MOVING),
        JPC_MOTION_TYPE_DYNAMIC,
    );
    sphere_settings.set_position(RVec3::new(0.0, 2.0, 0.0));
    let sphere = body_interface.create_body(&sphere_settings).unwrap();
    let sphere_id = sphere.id();

    body_interface.add_body(sphere_id, JPC_ACTIVATION_ACTIVATE);
    body_interface.set_linear_velocity(sphere_id, Vec3::new(0.0, -5.0, 0.0));

    physics_system.optimize_broad_phase();

    let delta_time = 1.0 / 60.0;
    let collision_steps = 1;

    let mut step = 0;
    while body_interface.is_active(sphere_id) {
        step += 1;

        let position = body_interface.center_of_mass_position(sphere_id);
        let velocity = body_interface.linear_velocity(sphere_id);
        println!(
            "Step {step}: Position = ({}, {}, {}), Velocity = ({}, {}, {})",
            position.x, position.y, position.z, velocity.x, velocity.y, velocity.z
        );

        physics_system.update(delta_time, collision_steps, &temp_allocator, &job_system);
    }

    let narrow_phase = physics_system.narrow_phase_query();

    let mut collector = ClosestHitCastShapeCollector::new();
    narrow_phase.cast_shape(CastShapeArgs {
        shapecast: RShapeCast {
            shape: sphere_shape.clone(),
            scale: Vec3::ONE,
            center_of_mass_start: rmat44_translation(RVec3::new(-5.0, 0.0, 0.0).into_jolt()),
            direction: Vec3::new(10.0, 0.0, 0.0),
        },
        base_offset: RVec3::ZERO,
        settings: Default::default(),
        collector: Some(CastShapeCollectorImpl::new_borrowed(&mut collector)),
        broad_phase_layer_filter: None,
        object_layer_filter: None,
        body_filter: None,
        shape_filter: None,
    });

    println!("Hit: {}", collector.result.is_some());

    body_interface.remove_body(floor_id);
    body_interface.destroy_body(floor_id);

    body_interface.remove_body(sphere_id);
    body_interface.destroy_body(sphere_id);

    drop(physics_system);

    rolt::unregister_types();
    rolt::factory_delete();

    println!("Hello, world!");
}

fn rvec3(x: Real, y: Real, z: Real) -> JPC_RVec3 {
    JPC_RVec3 { x, y, z, _w: z }
}

fn vec4(x: f32, y: f32, z: f32, w: f32) -> JPC_Vec4 {
    JPC_Vec4 { x, y, z, w }
}

// If 'double-precision' is set, there is padding in this struct
#[allow(clippy::needless_update)]
fn rmat44_identity() -> JPC_RMat44 {
    unsafe {
        JPC_RMat44 {
            col: [
                vec4(1.0, 0.0, 0.0, 0.0),
                vec4(0.0, 1.0, 0.0, 0.0),
                vec4(0.0, 0.0, 1.0, 0.0),
            ],
            col3: rvec3(0.0, 0.0, 0.0),
            ..std::mem::zeroed()
        }
    }
}

fn rmat44_translation(col3: JPC_RVec3) -> JPC_RMat44 {
    JPC_RMat44 {
        col3,
        ..rmat44_identity()
    }
}

#[test]
fn run_main() {
    main();
}
