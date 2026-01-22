use super::Game;
use crate::core::frame_timer::FrameTimer;
use crate::core::{Context, GameBuilder, Time, Window};
use crate::gfx::{Draw, Graphics};
use crate::input::{Gamepads, Keyboard, Mouse};
use crate::prelude::ContextData;
use directories::ProjectDirs;
use dpi::LogicalSize;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

enum AppState<G: Game> {
    Startup {
        opts: GameBuilder,
        cfg: Option<G::Config>,
    },
    Running {
        ctx: Context,
        draw: Draw,
        timer: FrameTimer,
        size: LogicalSize<f64>,
        game: G,
        has_updated: bool,

        #[cfg(feature = "lua")]
        lua_app: crate::core::LuaApp,
    },
}

pub(crate) struct AppHandler<G: Game> {
    state: AppState<G>,
}

impl<G: Game> AppHandler<G> {
    pub(crate) fn new(opts: GameBuilder, cfg: G::Config) -> Self {
        Self {
            state: AppState::Startup {
                opts,
                cfg: Some(cfg),
            },
        }
    }
}

impl<G: Game> ApplicationHandler for AppHandler<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let AppState::Startup { opts, cfg } = &mut self.state else {
            return;
        };

        // create the window
        let size = LogicalSize::new(opts.size.x as f64, opts.size.y as f64);
        let attrs = WindowAttributes::default()
            .with_title(&opts.title)
            .with_inner_size(size);
        let window = Window(Arc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        ));

        // initialize the graphics
        let graphics = Graphics::new(window.clone(), opts);

        // create the drawing context
        let draw = Draw::new(
            graphics.device().clone(),
            graphics.queue().clone(),
            graphics.default_shader().clone(),
            graphics.default_texture().clone(),
        );

        // load the project directories
        let app_name = if opts.app_name.is_empty() {
            opts.title.as_str()
        } else {
            opts.app_name.as_str()
        };
        let dirs = ProjectDirs::from("", &opts.app_organization, app_name)
            .expect("failed to locate system directories");

        // create the game context
        let ctx = Context(Rc::new(ContextData {
            window,
            time: Time::new(),
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            gamepads: Gamepads::new(),
            graphics,

            #[cfg(feature = "lua")]
            lua: opts.lua.weak(),

            #[cfg(feature = "lua")]
            reload_lua: Cell::new(false),

            quit_requested: Cell::new(false),

            dirs,
        }));

        // create the frame timer
        let timer = FrameTimer::new(ctx.time.0.clone());

        #[cfg(feature = "lua")]
        let lua_app = crate::core::LuaApp::new(opts.lua.clone(), &ctx);

        // create the game
        // TODO: propagate error
        let game = G::new(&ctx, cfg.take().unwrap()).unwrap();

        // start running the app loop
        self.state = AppState::Running {
            ctx,
            draw,
            timer,
            size,
            game,
            has_updated: false,

            #[cfg(feature = "lua")]
            lua_app,
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let AppState::Running {
            ctx,
            draw,
            timer,
            size,
            game,
            has_updated,

            #[cfg(feature = "lua")]
            lua_app,
        } = &mut self.state
        else {
            panic!("app not running");
        };

        // likely redundant, winit probably only calls this for windows created by this process
        if ctx.window.0.id() != window_id {
            return;
        };

        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(new_size) => {
                ctx.graphics.resized(new_size);
                *size = new_size.to_logical::<f64>(ctx.window.0.scale_factor());
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { event, .. } => {
                ctx.keyboard.handle_event(event);
            }
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical::<f32>(ctx.window.0.scale_factor());
                ctx.mouse.handle_move(position);
            }
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => {
                ctx.mouse.handle_scroll(delta);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                ctx.mouse.handle_input(button, state);
            }
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                mut inner_size_writer,
            } => {
                // if the scale factor changes, update the window size to match its
                // previous logical size
                inner_size_writer
                    .request_inner_size(size.to_physical(scale_factor))
                    .expect("failed to update window size");
            }
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                let monitor = ctx.window.monitor();

                timer.tick(monitor, || {
                    *has_updated = true;

                    // update gamepad input
                    ctx.gamepads.update(ctx);

                    // update the lua app
                    #[cfg(feature = "lua")]
                    lua_app.update(ctx);

                    // update the game
                    // TODO: propagate this error somewhere
                    game.update(ctx).unwrap();

                    // clear input on-frame events (eg. pressed, released)
                    ctx.mouse.clear_phase();
                    ctx.keyboard.clear_phase();
                    ctx.gamepads.clear_phase();
                });

                // switch to the render phase for input
                ctx.mouse.set_render_phase();
                ctx.keyboard.set_render_phase();
                ctx.gamepads.set_render_phase();

                // begin rendering a frame
                draw.begin_frame(ctx.window.size());

                // only do render callbacks after we've started updating
                if *has_updated {
                    // render the lua app
                    #[cfg(feature = "lua")]
                    lua_app.render(ctx, draw);

                    // render the game
                    // TODO: propagate this error somewhere
                    game.render(ctx, draw).unwrap();
                }

                // finish rendering a frame
                draw.end_frame(timer.time.frame.get(), ctx.graphics.surface(), &ctx.window);

                // clear input on-frame events (eg. pressed, released)
                ctx.mouse.clear_phase();
                ctx.keyboard.clear_phase();
                ctx.gamepads.clear_phase();

                // switch back to the update phase for input
                ctx.mouse.set_update_phase();
                ctx.keyboard.set_update_phase();
                ctx.gamepads.set_update_phase();

                // quit if the user requested it
                if ctx.quit_requested() {
                    event_loop.exit();
                }
            }
        }
    }
}
