use bracket_lib::prelude::RandomNumberGenerator;
use crate::camera::*;
use crate::prelude::*;
use TurnState::*;

mod map;
mod map_builder;
mod camera;
mod spawner;
mod components;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::turn_state::*;
    pub use crate::systems::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub const DUNGEON_FONT: &str = "dungeonfont.png";
    pub const TERM8X8_FONT: &str = "terminal8x8.png";
}

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder.rooms.iter().skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

macro_rules! clear_console {
    ($ctx: expr, $console_id: expr) => {
        $ctx.set_active_console($console_id);
        $ctx.cls();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        clear_console!(ctx, 2);
        clear_console!(ctx, 1);
        clear_console!(ctx, 0);

        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state
            = *self.resources.get::<TurnState>().unwrap();
        let system = match current_state {
            AwaitingInput => &mut self.input_systems,
            PlayerTurn => &mut self.player_systems,
            MonsterTurn => &mut self.monster_systems,
        };
        system.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx).expect("Render error");
    }
}


fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(DUNGEON_FONT, 32, 32)
        .with_font(TERM8X8_FONT, 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT)
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, TERM8X8_FONT)
        .build()?;
    main_loop(context, State::new())
}