mod abtest;
mod challenges;
mod colors;
mod common;
mod debug;
mod devtools;
mod edit;
mod game;
mod helpers;
mod managed;
mod obj_actions;
mod options;
mod pregame;
mod render;
mod sandbox;
mod ui;

use crate::ui::Flags;
use abstutil::CmdArgs;
use sim::SimFlags;

fn main() {
    let mut args = CmdArgs::new();

    if args.enabled("--prebake") {
        challenges::prebake_all();
        return;
    }

    let mut flags = Flags {
        sim_flags: SimFlags::from_args(&mut args),
        kml: args.optional("--kml"),
        draw_lane_markings: !args.enabled("--dont_draw_lane_markings"),
        num_agents: args.optional_parse("--num_agents", |s| s.parse()),
    };
    let mut opts = options::Options::default();
    if args.enabled("--dev") {
        opts.dev = true;
        flags.sim_flags.rng_seed = Some(42);
    }

    // No random in wasm
    #[cfg(target_arch = "wasm32")]
    {
        flags.sim_flags.rng_seed = Some(42);
    }

    if let Some(x) = args.optional("--color_scheme") {
        opts.color_scheme = Some(format!("../data/system/{}", x));
    }
    let mut settings = ezgui::Settings::new("A/B Street", "../data/system/fonts");
    if args.enabled("--enable_profiler") {
        settings.enable_profiling();
    }
    if args.enabled("--dump_raw_events") {
        settings.dump_raw_events();
    }
    if let Some(n) = args.optional_parse("--font_size", |s| s.parse::<usize>()) {
        settings.default_font_size(n);
    }

    let mut mode = None;
    if let Some(x) = args.optional("--challenge") {
        let mut aliases = Vec::new();
        'OUTER: for (_, stages) in challenges::all_challenges(true) {
            for challenge in stages {
                if challenge.alias == x {
                    flags.sim_flags.load = challenge.gameplay.map_path();
                    mode = Some(challenge.gameplay);
                    break 'OUTER;
                } else {
                    aliases.push(challenge.alias);
                }
            }
        }
        if mode.is_none() {
            panic!(
                "Don't know --challenge={}. Choices: {}",
                x,
                aliases.join(", ")
            );
        }
    }
    // TODO Stage only, not part
    if let Some(n) = args.optional_parse("--tutorial", |s| s.parse::<usize>()) {
        mode = Some(sandbox::GameplayMode::Tutorial(
            sandbox::TutorialPointer::new(n - 1, 0),
        ));
    }

    args.done();

    ezgui::run(settings, |ctx| game::Game::new(flags, opts, mode, ctx));
}
