use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// -----------------------------------------------------------------------------
pub fn draw_regular_shapes(
    mut commands: Commands,
) {
    let triangle = RegularPolygon {
        sides: 3,
        center: Vec2::new(-500.0, 400.0),
        feature: RegularPolygonFeature::Radius(50.0),
    };
    let square = RegularPolygon {
        sides: 4,
        center: Vec2::new(-300.0, 400.0),
        feature: RegularPolygonFeature::Radius(50.0),
    };
    let pentagon = RegularPolygon {
        sides: 5,
        center: Vec2::new(-100.0, 400.0),
        feature: RegularPolygonFeature::Radius(50.0),
    };
    let hexagon = RegularPolygon {
        sides: 6,
        center: Vec2::new(100.0, 400.0),
        feature: RegularPolygonFeature::Radius(50.0),
    };
    let heptagon = RegularPolygon {
        sides: 7,
        center: Vec2::new(300.0, 400.0),
        feature: RegularPolygonFeature::Radius(50.0),
    };
    let octagon = RegularPolygon {
        sides: 8,
        center: Vec2::new(500.0, 400.0),
        feature: RegularPolygonFeature::Radius(50.0),
    };

    let mut shapes = Vec::new();
    shapes.push(triangle);
    shapes.push(square);
    shapes.push(pentagon);
    shapes.push(hexagon);
    shapes.push(heptagon);
    shapes.push(octagon);

    for shape in shapes {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::TURQUOISE),
            Stroke::new(Color::BLACK, 10.0),
        ));
    }
}

pub fn draw_circle(
    mut commands: Commands,
) {
    let circle = shapes::Circle {
        radius: 50.0,
        center: Vec2::new(-500.0, 200.0),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
            ..default()
        },
        Fill::color(Color::ORANGE),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

pub fn draw_ellipse(
    mut commands: Commands,
) {
    let ellipse = shapes::Ellipse {
        radii: Vec2::new(60.0, 40.0),
        center: Vec2::new(-250.0, 200.0),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&ellipse),
            ..default()
        },
        Fill::color(Color::NAVY),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

pub fn draw_line(
    mut commands: Commands,
) {
    let line = shapes::Line(Vec2::new(-500.0, -200.0), Vec2::new(500.0, -200.0));

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line),
            ..default()
        },
        Fill::color(Color::WHITE),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

pub fn draw_rectangle(
    mut commands: Commands,
) {
    let rectangle = shapes::Rectangle {
        extents: Vec2::new(120.0, 80.0),
        origin: RectangleOrigin::CustomCenter(Vec2::new(0.0, 200.0)),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rectangle),
            ..default()
        },
        Fill::color(Color::CRIMSON),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

pub fn draw_polygon(
    mut commands: Commands,
) {
    let polygon = shapes::Polygon {
        points: vec![
            Vec2::new(190.0, 150.0),
            Vec2::new(310.0, 150.0),
            Vec2::new(290.0, 250.0),
            Vec2::new(210.0, 250.0),
        ],
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&polygon),
            ..default()
        },
        Fill::color(Color::LIME_GREEN),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

pub fn draw_rounded_polygon(
    mut commands: Commands,
) {
    let rounded_polygon = shapes::RoundedPolygon {
        points: vec![
            Vec2::new(440.0, 150.0),
            Vec2::new(560.0, 150.0),
            Vec2::new(540.0, 250.0),
            Vec2::new(460.0, 250.0),
        ],
        radius: 20.0,
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rounded_polygon),
            ..default()
        },
        Fill::color(Color::BISQUE),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

pub fn draw_path(
    mut commands: Commands,
) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0., 0.));
    path_builder.cubic_bezier_to(
        Vec2::new(70., 70.),
        Vec2::new(175., -35.),
        Vec2::new(0., -140.),
    );
    path_builder.cubic_bezier_to(
        Vec2::new(-175., -35.),
        Vec2::new(-70., 70.),
        Vec2::new(0., 0.),
    );
    path_builder.close();
    let path = path_builder.build();

    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_xyz(0., 75., 0.),
            ..default()
        },
        Fill::color(Color::ALICE_BLUE),
        Stroke::new(Color::BLACK, 10.0),
    ));
}

