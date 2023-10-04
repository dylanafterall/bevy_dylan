use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// -----------------------------------------------------------------------------
pub fn draw_regular_shapes(mut commands: Commands) {
    let triangle = RegularPolygon {
        sides: 3,
        center: Vec2::new(-115.0, -40.0),
        feature: RegularPolygonFeature::Radius(5.0),
    };
    let square = RegularPolygon {
        sides: 4,
        center: Vec2::new(-105.0, -40.0),
        feature: RegularPolygonFeature::Radius(5.0),
    };
    let pentagon = RegularPolygon {
        sides: 5,
        center: Vec2::new(-95.0, -40.0),
        feature: RegularPolygonFeature::Radius(5.0),
    };
    let hexagon = RegularPolygon {
        sides: 6,
        center: Vec2::new(-85.0, -40.0),
        feature: RegularPolygonFeature::Radius(5.0),
    };
    let heptagon = RegularPolygon {
        sides: 7,
        center: Vec2::new(-75.0, -40.0),
        feature: RegularPolygonFeature::Radius(5.0),
    };
    let octagon = RegularPolygon {
        sides: 8,
        center: Vec2::new(-65.0, -40.0),
        feature: RegularPolygonFeature::Radius(5.0),
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
            Stroke::new(Color::BLACK, 0.5),
        ));
    }
}

pub fn draw_circle(mut commands: Commands) {
    let circle = shapes::Circle {
        radius: 5.0,
        center: Vec2::new(-115.0, -55.0),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
            ..default()
        },
        Fill::color(Color::ORANGE),
        Stroke::new(Color::BLACK, 0.5),
    ));
}

pub fn draw_ellipse(mut commands: Commands) {
    let ellipse = shapes::Ellipse {
        radii: Vec2::new(6.0, 4.0),
        center: Vec2::new(-100.0, -55.0),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&ellipse),
            ..default()
        },
        Fill::color(Color::NAVY),
        Stroke::new(Color::BLACK, 0.5),
    ));
}

pub fn draw_line(mut commands: Commands) {
    let line = shapes::Line(Vec2::new(-120.0, -65.0), Vec2::new(-60.0, -65.0));

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line),
            ..default()
        },
        Fill::color(Color::WHITE),
        Stroke::new(Color::BLACK, 0.5),
    ));
}

pub fn draw_rectangle(mut commands: Commands) {
    let rectangle = shapes::Rectangle {
        extents: Vec2::new(12.0, 8.0),
        origin: RectangleOrigin::CustomCenter(Vec2::new(-85.0, -55.0)),
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rectangle),
            ..default()
        },
        Fill::color(Color::CRIMSON),
        Stroke::new(Color::BLACK, 0.5),
    ));
}

pub fn draw_polygon(mut commands: Commands) {
    let polygon = shapes::Polygon {
        points: vec![
            Vec2::new(-6.0, -5.0),
            Vec2::new(6.0, -5.0),
            Vec2::new(4.0, 5.0),
            Vec2::new(-4.0, 5.0),
        ],
        closed: true,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&polygon),
                ..default()
            },
            Fill::color(Color::LIME_GREEN),
            Stroke::new(Color::BLACK, 0.5),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -70.0, -55.0, 0.0,
        )));
}

pub fn draw_rounded_polygon(mut commands: Commands) {
    let rounded_polygon = shapes::RoundedPolygon {
        points: vec![
            Vec2::new(-6.0, -5.0),
            Vec2::new(6.0, -5.0),
            Vec2::new(4.0, 5.0),
            Vec2::new(-4.0, 5.0),
        ],
        radius: 2.0,
        closed: true,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&rounded_polygon),
                ..default()
            },
            Fill::color(Color::BISQUE),
            Stroke::new(Color::BLACK, 0.5),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(70.0, -55.0, 0.0)));
}

pub fn draw_path(mut commands: Commands) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0.0, 0.0));
    path_builder.cubic_bezier_to(
        Vec2::new(7.0, 7.0),
        Vec2::new(17.5, -3.5),
        Vec2::new(0.0, -14.0),
    );
    path_builder.cubic_bezier_to(
        Vec2::new(-17.5, -3.5),
        Vec2::new(-7.0, 7.0),
        Vec2::new(0.0, 0.0),
    );
    path_builder.close();
    let path = path_builder.build();

    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_xyz(90.0, -50.0, 0.0),
            ..default()
        },
        Fill::color(Color::PINK),
        Stroke::new(Color::BLACK, 0.5),
    ));
}
