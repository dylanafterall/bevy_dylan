use super::components::*;
use crate::style::FRAPPE_RED;
use bevy::sprite::Mesh2dHandle;
use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{prelude::*, rapier::prelude::MotorModel};

// -----------------------------------------------------------------------------
pub fn spawn_elastic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const CENTER_POS: Vec2 = Vec2::new(-40.0, 0.0);

    /*
    KEY:
    () = vertex, physics body & collider
    *  = joint
                          (3)

                *c                   *d

          (2)                               (4)


          *b          (CENTER_POS)          *e


          (1)                               (5)

                *a                   *f

                          (0)

                          *g = from (0) to (CENTER_POS)
                          *h = from (2) to (CENTER_POS)
                          *i = from (4) to (CENTER_POS)
     */

    // vertices from mesh file -------------------------------------------------
    let vertices = vec![
        Vec2::new(0.0, -8.0),   // bottom
        Vec2::new(-6.93, -4.0), // bottom left
        Vec2::new(-6.93, 4.0),  // top left
        Vec2::new(0.0, 8.0),    // top
        Vec2::new(6.93, 4.0),   // top right
        Vec2::new(6.93, -4.0),  // bottom right
    ];

    // setup vertices for physics ----------------------------------------------
    let pos_0 = CENTER_POS + vertices[0];
    let pos_1 = CENTER_POS + vertices[1];
    let pos_2 = CENTER_POS + vertices[2];
    let pos_3 = CENTER_POS + vertices[3];
    let pos_4 = CENTER_POS + vertices[4];
    let pos_5 = CENTER_POS + vertices[5];

    // setup vertices for render -----------------------------------------------
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            Vec3::from((Vec2::ZERO, 0.0)),
            Vec3::from((vertices[0], 0.0)),
            Vec3::from((vertices[1], 0.0)),
            Vec3::from((vertices[2], 0.0)),
            Vec3::from((vertices[3], 0.0)),
            Vec3::from((vertices[4], 0.0)),
            Vec3::from((vertices[5], 0.0)),
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1,
    ])));

    // setup joints ------------------------------------------------------------
    let joint_a = generate_spring(pos_0, pos_1);
    let joint_b = generate_spring(pos_1, pos_2);
    let joint_c = generate_spring(pos_2, pos_3);
    let joint_d = generate_spring(pos_3, pos_4);
    let joint_e = generate_spring(pos_4, pos_5);
    let joint_f = generate_spring(pos_5, pos_0);
    let joint_g = generate_spring(pos_0, CENTER_POS);
    let joint_h = generate_spring(pos_2, CENTER_POS);
    let joint_i = generate_spring(pos_4, CENTER_POS);

    // setup entities ----------------------------------------------------------
    let vert_center = generate_body(&mut commands, Elastic::Center);
    commands
        .entity(vert_center)
        .insert(ElasticMesh)
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(mesh)).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_RED)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((CENTER_POS, 0.0)),
        )));

    let vert_0 = generate_body(&mut commands, Elastic::_0);
    commands
        .entity(vert_0)
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((pos_0, 0.0)),
        )));

    let vert_1 = generate_body(&mut commands, Elastic::_1);
    commands
        .entity(vert_1)
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((pos_1, 0.0)),
        )));

    let vert_2 = generate_body(&mut commands, Elastic::_2);
    commands
        .entity(vert_2)
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((pos_2, 0.0)),
        )));

    let vert_3 = generate_body(&mut commands, Elastic::_3);
    commands
        .entity(vert_3)
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((pos_3, 0.0)),
        )));

    let vert_4 = generate_body(&mut commands, Elastic::_4);
    commands
        .entity(vert_4)
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((pos_4, 0.0)),
        )));

    let vert_5 = generate_body(&mut commands, Elastic::_5);
    commands
        .entity(vert_5)
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::from((pos_5, 0.0)),
        )));

    // attach joints -----------------------------------------------------------
    commands
        .entity(vert_center)
        .insert(ImpulseJoint::new(vert_0, joint_g))
        .with_children(|children| {
            children.spawn(ImpulseJoint::new(vert_2, joint_h));
            children.spawn(ImpulseJoint::new(vert_4, joint_i));
        });

    commands
        .entity(vert_0)
        .insert(ImpulseJoint::new(vert_5, joint_f));

    commands
        .entity(vert_1)
        .insert(ImpulseJoint::new(vert_0, joint_a));

    commands
        .entity(vert_2)
        .insert(ImpulseJoint::new(vert_1, joint_b));

    commands
        .entity(vert_3)
        .insert(ImpulseJoint::new(vert_2, joint_c));

    commands
        .entity(vert_4)
        .insert(ImpulseJoint::new(vert_3, joint_d));

    commands
        .entity(vert_5)
        .insert(ImpulseJoint::new(vert_4, joint_e));
}

pub fn update_mesh_positions(
    mut meshes: ResMut<Assets<Mesh>>,
    elastic_query: Query<(&Elastic, &Transform)>,
    elastic_mesh: Query<&mut Mesh2dHandle, With<ElasticMesh>>,
) {
    let elastic_mesh = elastic_mesh.single().0.id();

    for (id, mesh) in meshes.iter_mut() {
        if id == elastic_mesh {
            let mut vertex_vec = vec![Vec3::ZERO; 7];

            for (elastic_id, elastic_transform) in elastic_query.iter() {
                match elastic_id {
                    Elastic::Center => vertex_vec[0] = elastic_transform.translation,
                    Elastic::_0 => vertex_vec[1] = elastic_transform.translation,
                    Elastic::_1 => vertex_vec[2] = elastic_transform.translation,
                    Elastic::_2 => vertex_vec[3] = elastic_transform.translation,
                    Elastic::_3 => vertex_vec[4] = elastic_transform.translation,
                    Elastic::_4 => vertex_vec[5] = elastic_transform.translation,
                    Elastic::_5 => vertex_vec[6] = elastic_transform.translation,
                };
            }

            let center_vertex_position = vertex_vec[0];
            for v in &mut vertex_vec {
                *v -= center_vertex_position;
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_vec);
            mesh.set_indices(Some(Indices::U32(vec![
                0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1,
            ])));
        }
    }
}

fn generate_spring(vertex1: Vec2, vertex2: Vec2) -> PrismaticJointBuilder {
    const JOINT_STIFFNESS: f32 = 25.0;
    const JOINT_DAMPING: f32 = 0.05;
    const JOINT_LIMIT_FACTOR: f32 = 0.25;
    const JOINT_MAX_FORCE: f32 = 5.0;

    let joint_vector = vertex2 - vertex1;
    let joint_limit = joint_vector.length() * JOINT_LIMIT_FACTOR;

    let joint = PrismaticJointBuilder::new(joint_vector)
        .local_anchor1(joint_vector)
        .limits([-joint_limit, joint_limit])
        .motor_model(MotorModel::AccelerationBased)
        .motor_max_force(JOINT_MAX_FORCE)
        .motor_position(0.0, JOINT_STIFFNESS, JOINT_DAMPING);

    joint
}

pub fn generate_body(commands: &mut Commands, elastic: Elastic) -> Entity {
    const COLLIDER_RADIUS: f32 = 1.0;

    commands
        .spawn(elastic)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(COLLIDER_RADIUS))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.0))
        .id()
}
