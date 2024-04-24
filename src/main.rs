use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_hanabi::prelude::*;

#[derive(Resource)]
struct MyAssets {
    effect: Handle<EffectAsset>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HanabiPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update.run_if(on_event::<MouseButtonInput>()))
        .run();
}

fn setup(mut effects: ResMut<Assets<EffectAsset>>, mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scaling_mode = ScalingMode::FixedVertical(640.);
    commands.spawn(bundle);

    commands.insert_resource(MyAssets {
        effect: effects.add(build_explosion_effect()),
    });
}

fn update(
    my_assets: Res<MyAssets>,
    mut events: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    for event in events.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            commands.spawn(ParticleEffectBundle::new(my_assets.effect.clone()));
        }
    }
}

fn build_explosion_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    let age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
    let lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        (writer.rand(ValueType::Scalar(ScalarType::Float)) * writer.lit(2.5)).expr(),
    );

    let position = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(0., 0., 0.)).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(0.01).expr(),
        dimension: ShapeDimension::Surface,
    };
    let velocity = SetVelocityCircleModifier {
        center: writer.lit(Vec3::new(0., 0., 0.)).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: (writer.rand(ValueType::Scalar(ScalarType::Float)) * writer.lit(100.)).expr(),
    };

    let drag = LinearDragModifier::new(writer.rand(ValueType::Scalar(ScalarType::Float)).expr());

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.0));
    gradient.add_key(1.0, Vec4::new(1., 1., 1., 0.0));

    let module = writer.finish();

    EffectAsset::new(4096, Spawner::once(200f32.into(), true), module)
        .init(position)
        .init(age)
        .init(lifetime)
        .init(velocity)
        .update(drag)
        .render(SetSizeModifier {
            size: Vec2::splat(1.).into(),
            ..default()
        })
        .render(ColorOverLifetimeModifier { gradient })
}
