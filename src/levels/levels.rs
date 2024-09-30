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
        next: Some(Box::new(level_6)),
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
        next: Some(Box::new(level_11)),
        player_start,
    }
}

pub fn level_11() -> Level<'static> {
    let player_start = v2!(0., -16.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::custom_block(v2!(0., -48.), 32., 32.), // layer 1
        Entity::default_enemy(v2!(4., -16.)),
        Entity::default_enemy(v2!(10., -16.)),
        Entity::default_enemy(v2!(16., -16.)),
        Entity::default_enemy(v2!(22., -16.)),
        // Ladder
        Entity::small_block(v2!(30., -14.)),
        Entity::small_block(v2!(30., -11.)),
        Entity::small_block(v2!(30., -8.)),
        Entity::small_block(v2!(24., -7.)),
        Entity::small_block(v2!(10., -7.)),
    ];

    let goal = Entity::default_goal(v2!(0., -6.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 11" },
        entities: special,
        next: Some(Box::new(level_12)),
        player_start,
    }
}

pub fn level_12() -> Level<'static> {
    let player_start = v2!(15., -15.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::small_block(v2!(15., -16.)),
        Entity::small_enemy(v2!(16., -17.)),
        Entity::small_enemy(v2!(14., -17.)),
        Entity::small_enemy(v2!(13., -17.)),
        Entity::small_enemy(v2!(17., -17.)),
        Entity::small_enemy(v2!(18., -17.)),
        Entity::small_enemy(v2!(12., -17.)),
        Entity::small_enemy(v2!(15., -17.)),
        Entity::small_enemy(v2!(13., -24.)),
        Entity::small_enemy(v2!(14., -24.)),
        Entity::small_enemy(v2!(15., -25.)),
        Entity::small_enemy(v2!(15., -26.)),
        Entity::small_enemy(v2!(15., -27.)),
        Entity::small_enemy(v2!(15., -28.)),
        Entity::small_enemy(v2!(15., -29.)),
        Entity::small_enemy(v2!(17., -24.)),
        Entity::small_enemy(v2!(18., -24.)),
        Entity::small_enemy(v2!(19., -24.)),
        Entity::small_enemy(v2!(20., -24.)),
        Entity::small_enemy(v2!(21., -24.)),
        Entity::small_enemy(v2!(11., -24.)),
        Entity::small_enemy(v2!(12., -24.)),
        Entity::small_enemy(v2!(11., -25.)),
        Entity::small_enemy(v2!(11., -26.)),
        Entity::small_enemy(v2!(11., -27.)),
        Entity::small_enemy(v2!(12.5, -29.)),
        Entity::small_enemy(v2!(13., -29.)),
        Entity::small_enemy(v2!(14., -29.)),
        Entity::small_block(v2!(12.3, -31.)),
        Entity::small_block(v2!(13., -31.)),
        Entity::small_block(v2!(14., -31.)),
        Entity::small_block(v2!(15., -31.)),
        Entity::small_block(v2!(16., -31.)),
        Entity::small_block(v2!(15., -23.)),
    ];

    let goal = Entity::default_goal(v2!(12., -28.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 12" },
        entities: special,
        next: Some(Box::new(level_12)),
        player_start,
    }
}

pub fn level_13() -> Level<'static> {
    let player_start = v2!(0., -28.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::small_enemy(v2!(0., -25.)),
        Entity::default_block(v2!(0., -30.)),
        Entity::default_block(v2!(2., -30.)),
        Entity::default_block_moving(v2!(4., -32.), v2!(0., 2.5)),
    ];

    let goal = Entity::default_goal(v2!(8., -4.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 13" },
        entities: special,
        next: Some(Box::new(level_14)),
        player_start,
    }
}

pub fn level_14() -> Level<'static> {
    let player_start = v2!(0., -2.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::default_block(v2!(0., -4.)),
        Entity::default_block_moving(v2!(2., -0.), v2!(0., -2.)),
        Entity::default_block_moving(v2!(6., -40.), v2!(0., 2.)),
        Entity::default_block(v2!(-2., -2.)),
        Entity::default_enemy(v2!(4., -2.)),
        Entity::default_enemy(v2!(4., -4.)),
        Entity::default_enemy(v2!(4., -6.)),
        Entity::default_enemy(v2!(4., -8.)),
        Entity::default_enemy(v2!(4., -10.)),
        Entity::default_enemy(v2!(4., -12.)),
        Entity::default_enemy(v2!(4., -14.)),
        Entity::default_enemy(v2!(6., -2.)),
        Entity::default_enemy(v2!(4., -2.)),
        Entity::default_enemy(v2!(8., -8.)),
        Entity::default_enemy(v2!(8., -10.)),
        Entity::default_enemy(v2!(8., -12.)),
        Entity::default_enemy(v2!(8., -14.)),
        Entity::default_block_moving(v2!(54., -8.), v2!(-2., 0.)),
    ];

    let goal = Entity::default_goal(v2!(26., -4.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 14" },
        entities: special,
        next: Some(Box::new(level_15)),
        player_start,
    }
}

pub fn level_15() -> Level<'static> {
    let player_start = v2!(4., -2.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::default_block(v2!(0., -4.)),
        Entity::default_block(v2!(2., -4.)),
        Entity::default_block(v2!(4., -4.)),
        Entity::default_block(v2!(6., -4.)),
        Entity::default_block(v2!(8., -4.)),
        Entity::default_block(v2!(10., -4.)),
        Entity::default_block(v2!(12., -4.)),
        Entity::default_block(v2!(14., -4.)),
        Entity::default_block(v2!(16., -4.)),
        Entity::default_block(v2!(18., -4.)),
        Entity::default_block(v2!(20., -4.)),
        Entity::default_block(v2!(22., -4.)),
        Entity::default_block(v2!(24., -4.)),
        Entity::default_enemy_moving(v2!(-2., -2.), v2!(3.0, 0.)),
        Entity::default_block(v2!(0., -8.)),
        Entity::default_block(v2!(2., -8.)),
        Entity::default_block(v2!(6., -8.)),
        Entity::default_block(v2!(8., -8.)),
        Entity::default_block(v2!(10., -8.)),
        Entity::default_block(v2!(12., -8.)),
        Entity::default_block(v2!(14., -8.)),
        Entity::default_block(v2!(16., -8.)),
        Entity::default_block(v2!(18., -8.)),
        Entity::default_block(v2!(20., -8.)),
        Entity::default_block(v2!(22., -8.)),
        Entity::default_block(v2!(24., -8.)),
        Entity::default_block(v2!(26., -8.)),
        Entity::default_block(v2!(28., -8.)),
        Entity::default_block(v2!(30., -8.)),
        Entity::default_enemy_moving(v2!(44., -6.), v2!(-4.0, 0.)),
        Entity::default_block_moving(v2!(-16., -12.), v2!(3.0, 0.)),
        Entity::default_enemy(v2!(28., -11.5)),
    ];

    let goal = Entity::default_goal(v2!(30., -12.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 15" },
        entities: special,
        next: Some(Box::new(level_16)),
        player_start,
    }
}

pub fn level_16() -> Level<'static> {
    let player_start = v2!(0., -30.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        Entity::small_enemy(v2!(0., -27.)),
        Entity::default_block(v2!(0., -32.)),
        Entity::default_enemy_moving(v2!(26., -32.), v2!(0.0, 1.8)),
        Entity::default_block_moving(v2!(4., -34.), v2!(1.0, 2.)),
    ];

    let goal = Entity::default_goal_moving(v2!(30., -32.), v2!(0., 1.8));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 16" },
        entities: special,
        next: Some(Box::new(level_17)),
        player_start,
    }
}

pub fn level_17() -> Level<'static> {
    let player_start = v2!(0., -14.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // Entity::small_enemy(v2!(0., -10.)),
        Entity::default_block(v2!(0., -16.)),
        Entity::default_enemy_moving(v2!(0., -6.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(2., -6.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(4., -6.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(6., -30.), v2!(0.0, 3.8)),
        Entity::default_enemy_moving(v2!(8., -30.), v2!(0.0, 3.8)),
        Entity::default_enemy_moving(v2!(10., -30.), v2!(0.0, 3.8)),
        Entity::default_block_moving(v2!(20., -30.), v2!(0.0, 1.8)),
        Entity::default_enemy_moving(v2!(22., 18.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(24., 18.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(26., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(26., -32.), v2!(0.0, 1.8)),
    ];

    let goal = Entity::default_goal(v2!(30., -4.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 17" },
        entities: special,
        next: Some(Box::new(level_18)),
        player_start,
    }
}

pub fn level_18() -> Level<'static> {
    let player_start = v2!(0., -14.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // Entity::small_enemy(v2!(0., -10.)),
        Entity::default_block(v2!(0., -16.)),
        Entity::default_enemy_moving(v2!(0., -6.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(0., 6.), v2!(0.0, -1.8)),
        Entity::small_block(v2!(2., -12.)),
        Entity::default_enemy_moving(v2!(3., -20.), v2!(0.0, 3.8)),
        Entity::default_enemy_moving(v2!(5., -20.), v2!(0.0, 3.8)),
        Entity::default_enemy_moving(v2!(7., -20.), v2!(0.0, 3.8)),
        Entity::small_block(v2!(2., -9.)),
        Entity::small_block(v2!(2., -6.)),
        // Entity::default_block_moving(v2!(11., -30.), v2!(0.0, 1.8)),
        // Entity::default_enemy_moving(v2!(13., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(14., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(26., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(26., -32.), v2!(0.0, 1.8)),
    ];

    let goal = Entity::default_goal(v2!(2., -2.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 18" },
        entities: special,
        next: Some(Box::new(level_19)),
        player_start,
    }
}

pub fn level_19() -> Level<'static> {
    let player_start = v2!(0., -28.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // Entity::small_enemy(v2!(0., -10.)),
        Entity::default_block(v2!(0., -30.)),
        // Entity::default_enemy_moving(v2!(0., -6.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(0., 6.), v2!(0.0, -1.8)),
        Entity::default_enemy_moving(v2!(14., -32.), v2!(0.0, 3.8)),
        Entity::default_enemy_moving(v2!(15., -32.), v2!(0.0, 3.8)),
        Entity::default_block_moving(v2!(15., -36.), v2!(0.0, 3.8)),
        Entity::default_enemy_moving(v2!(-6., -15.), v2!(3.8, 0.0)),
        Entity::default_enemy_moving(v2!(-6., -16.), v2!(3.8, 0.0)),
        Entity::default_enemy_moving(v2!(-6., -18.), v2!(3.8, 0.0)),
        // Entity::default_block_moving(v2!(-8., -18.), v2!(3.8, 0.0)),
        // Entity::default_enemy_moving(v2!(13., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(14., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(26., 18.), v2!(0.0, -1.8)),
        // Entity::default_enemy_moving(v2!(26., -32.), v2!(0.0, 1.8)),
    ];

    let goal = Entity::default_goal(v2!(15., -2.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 19" },
        entities: special,
        next: Some(Box::new(level_20)),
        player_start,
    }
}

pub fn level_20() -> Level<'static> {
    let player_start = v2!(15., -0.);
    let player = Entity::Player {
        health: 100,
        physical_properties: EntityPhysicalProperties::new(
            player_start,
            CollisionParameters::Enabled(BoundingBox::Rect { w: 1., h: 1. }),
        ),
    };
    let mut blocks = vec![
        // Entity::small_enemy(v2!(0., -10.)),
        Entity::default_block(v2!(12., -16.)),
        Entity::default_block(v2!(14., -14.)),
        Entity::default_block(v2!(16., -16.)),
        Entity::default_block(v2!(16., -18.)),
        Entity::default_block(v2!(14., -20.)),
        Entity::default_block(v2!(12., -22.)),
        Entity::default_block(v2!(12., -24.)),
        Entity::default_block(v2!(14., -24.)),
        Entity::default_block(v2!(16., -24.)),
        Entity::default_block(v2!(20., -16.)),
        Entity::default_block(v2!(22., -16.)),
        Entity::default_block(v2!(24., -16.)),
        Entity::default_block(v2!(24., -18.)),
        Entity::default_block(v2!(20., -20.)),
        Entity::default_block(v2!(22., -20.)),
        Entity::default_block(v2!(24., -20.)),
        Entity::default_block(v2!(24., -22.)),
        Entity::default_block(v2!(20., -24.)),
        Entity::default_block(v2!(22., -24.)),
        Entity::default_block(v2!(24., -24.)),
    ];

    let goal = Entity::default_goal(v2!(22., -22.));
    let mut special = vec![player, goal];
    special.append(&mut blocks);

    Level {
        level_metadata: LevelMetadata { name: "LEVEL 20" },
        entities: special,
        // next: Some(Box::new(level_1)),
        next: None,
        player_start,
    }
}
