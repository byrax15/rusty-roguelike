add_wasm_support!();
use std::collections::HashSet;
use bracket_lib::prelude::RandomNumberGenerator;

use TurnState::*;

use crate::camera::*;
use crate::prelude::*;

mod map;
mod camera;
mod spawner;
mod systems;
mod turn_state;
mod map_builder;
mod components;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;

    pub use crate::camera::*;
    pub use crate::map_builder::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::components::*;
    pub use crate::components::items::*;
    pub use crate::components::behaviour::*;

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

macro_rules! clear_console {
    ($ctx: expr, $console_id: expr) => {
        $ctx.set_active_console($console_id);
        $ctx.cls();
    }
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        Self::set_game_state(&mut ecs, &mut resources);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        Self::set_game_state(&mut self.ecs, &mut self.resources);
    }

    fn set_game_state(ecs: &mut World, resources: &mut Resources) {
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(ecs, map_builder.player_start);
        // spawn_amulet_of_yala(ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        Self::update_resources(ecs, resources, &mut rng, map_builder);
    }

    fn update_resources(ecs: &mut World, resources: &mut Resources, rng: &mut RandomNumberGenerator, map_builder: MapBuilder) {
        spawn_level(ecs, rng, 0, &map_builder.monster_spawns);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(AwaitingInput);
        resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK,
                                 "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK,
                                 "Slain by a monster, your hero's journey has come to a premature end.");
        ctx.print_color_centered(5, WHITE, BLACK,
                                 "The Amulet of Yala remains unclaimed, and your home town is not saved.");
        ctx.print_color_centered(8, YELLOW, BLACK,
                                 "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK,
                                 "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK,
                                 "You have won!");
        ctx.print_color_centered(4, WHITE, BLACK,
                                 "You put on the Amulet of Yala and feel its power course through your veins.");
        ctx.print_color_centered(5, WHITE, BLACK,
                                 "Your town is saved, and you can return to your normal life.");
        ctx.print_color_centered(7, GREEN, BLACK,
                                 "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    pub fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(e, _)| *e)
            .for_each(|e| { entities_to_keep.insert(e); });

        let mut cb = CommandBuffer::new(&self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        Self::update_resources(&mut self.ecs, &mut self.resources, &mut rng, map_builder);
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
            GameOver => {
                self.game_over(ctx);
                return;
            }
            Victory => {
                self.victory(ctx);
                return;
            }
            NextLevel => {
                self.advance_level();
                return;
            }
        };
        system.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx).expect("Render error");
    }
}

pub fn main() -> BError {
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