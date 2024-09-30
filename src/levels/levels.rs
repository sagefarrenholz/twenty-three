use crate::{
    foundation::{Entity, EntityPhysicalProperties},
    physics::collisions::{BoundingBox, CollisionParameters},
    v2,
};

use super::level::{Level, LevelMetadata};

pub fn level_1() -> Level<'static> {
    let player_start = v2!(0., 0.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let block = Entity::custom_block(v2!(0., -48.), 32., 32.);
    let goal = Entity::default_goal(v2!(30., -16.));

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 1" },
        entities: vec![player, block, goal],
        next: Some(Box::new(level_2)),
        player_start,
    }
}

pub fn level_2() -> Level<'static> {
    let player_start = v2!(0., -12.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::default_block(v2!(0., -20.)),
        Entity::default_block(v2!(0., -20.)),
        Entity::default_block(v2!(4., -20.)),
        Entity::default_block(v2!(6., -18.)),
        Entity::default_block(v2!(6., -16.)),
        Entity::default_block(v2!(6., -14.)),
        Entity::default_block(v2!(6., -12.)),
        Entity::default_block(v2!(6., -10.)),
        Entity::default_block(v2!(-2., -18.)),
        Entity::default_block(v2!(-2., -16.)),
        Entity::default_block(v2!(-2., -14.)),
        Entity::default_block(v2!(-2., -12.)),
        Entity::default_block(v2!(-2., -10.)),
        Entity::default_block(v2!(10., -30.)),
    ];

    let goal = Entity::default_goal(v2!(30., -32.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 2" },
        entities: special,
        next: Some(Box::new(level_3)),
        player_start,
    }
}

pub fn level_3() -> Level<'static> {
    let player_start = v2!(0., -30.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::default_block(v2!(0., -32.)),
        Entity::default_block(v2!(2., -30.)),
        Entity::default_block(v2!(4., -28.)),
        Entity::default_block(v2!(6., -26.)),
        Entity::default_block(v2!(8., -24.)),
        Entity::default_block(v2!(10., -22.)),
        Entity::default_block(v2!(12., -20.)),
        Entity::default_block(v2!(16., -18.)),
        Entity::default_block(v2!(20., -17.)),
        Entity::default_block(v2!(28., -16.)),
    ];

    let goal = Entity::default_goal(v2!(30., -16.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 3" },
        entities: special,
        next: Some(Box::new(level_4)),
        player_start,
    }
}

pub fn level_4() -> Level<'static> {
    let player_start = v2!(15., -12.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // V shape
        Entity::default_block(v2!(10., -10.)),
        Entity::default_block(v2!(12., -12.)),
        Entity::default_block(v2!(14., -14.)),
        Entity::default_block(v2!(16., -12.)),
        Entity::default_block(v2!(18., -10.)),
        // ^ shape
        Entity::default_block(v2!(10., -24.)),
        Entity::default_block(v2!(12., -22.)),
        Entity::default_block(v2!(14., -20.)),
        Entity::default_block(v2!(16., -22.)),
        Entity::default_block(v2!(18., -24.)),
    ];

    let goal = Entity::default_goal(v2!(14., -29.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 4" },
        entities: special,
        next: Some(Box::new(level_5)),
        player_start,
    }
}

pub fn level_5() -> Level<'static> {
    let player_start = v2!(0., -15.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // tunnel
        Entity::default_block(v2!(-2., -16.)),
        Entity::default_block(v2!(0., -14.)),
        Entity::default_block(v2!(0., -18.)),
        Entity::default_block(v2!(6., -18.)),
        Entity::default_block(v2!(6., -14.)),
        Entity::default_block(v2!(12., -18.)),
        Entity::default_block(v2!(12., -14.)),
        Entity::default_block(v2!(18., -18.)),
        Entity::default_block(v2!(18., -14.)),
        Entity::default_block(v2!(24., -18.)),
        Entity::default_block(v2!(24., -14.)),
        Entity::default_block(v2!(26., -18.)),
        Entity::default_block(v2!(26., -14.)),
        Entity::default_block(v2!(28., -18.)),
        Entity::default_block(v2!(28., -14.)),
    ];

    let goal = Entity::default_goal(v2!(30., -16.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 6" },
        entities: special,
        next: Some(Box::new(level_5)),
        player_start,
    }
}

pub fn level_6() -> Level<'static> {
    let player_start = v2!(15., -30.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // tunnel
        Entity::default_block(v2!(14., -32.)),
        Entity::default_block(v2!(16., -32.)),
        Entity::default_block(v2!(18., -32.)),
        Entity::default_block(v2!(12., -30.)),
        Entity::default_block(v2!(20., -30.)),
        Entity::small_block(v2!(2., -28.)),
        Entity::small_block(v2!(6., -28.)),
        Entity::small_block(v2!(8.1, -28.)),
        Entity::small_block(v2!(10., -28.)),
        Entity::small_block(v2!(12., -28.)),
        Entity::small_block(v2!(14., -28.)),
        Entity::small_block(v2!(15., -28.)),
        Entity::small_block(v2!(16., -28.)),
        Entity::small_block(v2!(17., -28.)),
        Entity::small_block(v2!(19., -28.)),
    ];

    let goal = Entity::default_goal(v2!(-1., -23.25));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 6" },
        entities: special,
        next: Some(Box::new(level_7)),
        player_start,
    }
}

pub fn level_7() -> Level<'static> {
    let player_start = v2!(0., -14.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::custom_block(v2!(0., -48.), 32., 32.),
        Entity::default_enemy(v2!(18., -16.)),
        Entity::small_enemy(v2!(18., -14.5)),
        Entity::small_enemy(v2!(19., -14.5)),
    ];

    let goal = Entity::default_goal(v2!(30., -16.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 7" },
        entities: special,
        next: Some(Box::new(level_8)),
        player_start,
    }
}

pub fn level_8() -> Level<'static> {
    let player_start = v2!(15., 0.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::small_block(v2!(15., -2.)),
        Entity::default_enemy(v2!(0., -14.5)),
        Entity::default_enemy(v2!(2., -14.5)),
        Entity::default_enemy(v2!(4., -14.5)),
        Entity::default_enemy(v2!(6., -14.5)),
        Entity::default_enemy(v2!(8., -14.5)),
        Entity::default_enemy(v2!(10., -14.5)),
        Entity::default_enemy(v2!(12., -14.5)),
        Entity::default_enemy(v2!(14., -14.5)),
        Entity::default_enemy(v2!(16., -14.5)),
        Entity::default_enemy(v2!(18., -14.5)),
        Entity::default_enemy(v2!(18., -16.5)),
        Entity::default_enemy(v2!(22., -16.5)),
        Entity::default_enemy(v2!(22., -14.5)),
        Entity::default_enemy(v2!(24., -14.5)),
        Entity::default_enemy(v2!(26., -14.5)),
        Entity::default_enemy(v2!(28., -14.5)),
        Entity::default_enemy(v2!(30., -14.5)),
    ];

    let goal = Entity::default_goal(v2!(30., -33.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 8" },
        entities: special,
        next: Some(Box::new(level_9)),
        player_start,
    }
}

pub fn level_9() -> Level<'static> {
    let player_start = v2!(15., 0.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::small_block(v2!(15., -2.)),
        // layer 1
        Entity::default_enemy(v2!(0., -9.5)),
        Entity::default_enemy(v2!(2., -9.5)),
        Entity::default_enemy(v2!(4., -9.5)),
        Entity::default_enemy(v2!(6., -9.5)),
        Entity::default_enemy(v2!(8., -9.5)),
        Entity::default_enemy(v2!(10., -9.5)),
        Entity::default_enemy(v2!(12., -9.5)),
        Entity::default_enemy(v2!(14., -9.5)),
        Entity::default_enemy(v2!(16., -9.5)),
        Entity::default_enemy(v2!(18., -9.5)),
        Entity::default_enemy(v2!(22., -9.5)),
        Entity::default_enemy(v2!(24., -9.5)),
        Entity::default_enemy(v2!(26., -9.5)),
        Entity::default_enemy(v2!(28., -9.5)),
        Entity::default_enemy(v2!(30., -9.5)),
        // layer 2
        Entity::default_enemy(v2!(0., -14.5)),
        Entity::default_enemy(v2!(2., -14.5)),
        Entity::default_enemy(v2!(4., -14.5)),
        Entity::default_enemy(v2!(6., -14.5)),
        Entity::default_enemy(v2!(8., -14.5)),
        Entity::default_enemy(v2!(10., -14.5)),
        Entity::default_enemy(v2!(12., -14.5)),
        Entity::default_enemy(v2!(14., -14.5)),
        Entity::default_enemy(v2!(16., -14.5)),
        Entity::default_enemy(v2!(16.75, -14.5)),
        Entity::default_enemy(v2!(21., -14.5)),
        Entity::default_enemy(v2!(22., -14.5)),
        Entity::default_enemy(v2!(24., -14.5)),
        Entity::default_enemy(v2!(26., -14.5)),
        Entity::default_enemy(v2!(28., -14.5)),
        Entity::default_enemy(v2!(30., -14.5)),
        // layer 3
        Entity::small_block(v2!(23., -26.)),
        Entity::default_enemy(v2!(0., -28.5)),
        Entity::default_enemy(v2!(8., -28.5)),
        Entity::default_enemy(v2!(10., -28.5)),
        Entity::default_enemy(v2!(13.3, -28.5)),
        Entity::default_enemy(v2!(18., -28.5)),
        Entity::default_enemy(v2!(20., -28.5)),
        Entity::default_enemy(v2!(22., -28.5)),
        Entity::default_enemy(v2!(24., -28.5)),
        Entity::default_enemy(v2!(26., -28.5)),
        Entity::default_enemy(v2!(28., -28.5)),
        Entity::default_enemy(v2!(30., -28.5)),
    ];

    let goal = Entity::default_goal(v2!(12., -33.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 9" },
        entities: special,
        next: Some(Box::new(level_10)),
        player_start,
    }
}

pub fn level_10() -> Level<'static> {
    let player_start = v2!(0., -13.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::default_block(v2!(0., -16.)),
        Entity::default_block(v2!(2., -14.)),
        Entity::default_block(v2!(0., -12.)),
        Entity::small_block(v2!(2., -24.)),
        Entity::small_block(v2!(11., -24.)),
        Entity::small_block(v2!(17., -23.)),
        Entity::small_block(v2!(14., -20.)),
        Entity::small_block(v2!(22., -19.)),
        Entity::small_block(v2!(28., -19.)),
        Entity::small_block(v2!(29., -19.)),
        Entity::small_block(v2!(30., -19.)),
        Entity::small_block(v2!(31., -19.)),
        Entity::small_block(v2!(29., -16.)),
        Entity::small_block(v2!(30., -16.)),
        Entity::small_block(v2!(31., -16.)),
        Entity::default_block(v2!(27., -15.)),
        // layer 1
    ];

    let goal = Entity::default_goal(v2!(29., -15.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 10" },
        entities: special,
        next: None,
        player_start,
    }
}
