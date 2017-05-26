
use glutin;
use image::{self, GenericImage};

use std::fmt;
use std::io::Read;

use audio;
use conf;
use filesystem::Filesystem;
use graphics;
use timer;
use GameError;
use GameResult;


/// A `Context` is an object that holds on to global resources.
/// It basically tracks hardware state such as the screen, audio
/// system, timers, and so on.  Generally this type is **not** thread-
/// safe and only one `Context` can exist at a time.  Trying to create
/// another one will fail.
///
/// Most functions that interact with the hardware, for instance
/// drawing things, playing sounds, or loading resources (which then
/// need to be transformed into a format the hardware likes) will need
/// to access the `Context`.
pub struct Context {
    pub conf: conf::Conf,
    pub filesystem: Filesystem,
    pub gfx_context: graphics::GraphicsContext,
    pub event_context: glutin::EventsLoop,
    pub timer_context: timer::TimeContext,
    pub audio_context: audio::AudioContext,

    pub default_font: graphics::Font,
}

impl fmt::Debug for Context {
    // TODO: Make this include more information?
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Context: {:p}>", self)
    }
}

/// Sets the window icon from the Conf `window_icon` field.
/// An empty string in the conf's `window_icon`
/// means to do nothing.
fn set_window_icon(context: &mut Context) -> GameResult<()> {
    Err(GameError::UnknownError("Current implementation does not allow setting a window icon".to_string()))
}

impl Context {
    /// Tries to create a new Context using settings from the given config file.
    /// Usually called by `Context::load_from_conf()`.
    fn from_conf(conf: conf::Conf, fs: Filesystem) -> GameResult<Context> {

        let audio_context = audio::AudioContext::new()?;
        let event_context = glutin::EventsLoop::new();
        let timer_context = timer::TimeContext::new();
        let font = graphics::Font::default_font()?;
        let graphics_context = graphics::GraphicsContext::new(&event_context,
                                                              &conf.window_title,
                                                              conf.window_width,
                                                              conf.window_height,
                                                              conf.vsync)?;

        let mut ctx = Context {
            conf: conf,
            filesystem: fs,
            gfx_context: graphics_context,
            event_context: event_context,
            timer_context: timer_context,
            audio_context: audio_context,
            
            default_font: font,
        };

        // set_window_icon(&mut ctx)?;

        Ok(ctx)
    }

    /// Tries to create a new Context loading a config
    /// file from its default path, using the given `Conf`
    /// object as a default if none is found.
    ///
    /// The `id` and `author` are game-specific strings that 
    /// are used to locate the default storage locations for the
    /// platform it looks in; for instance, on Linux, it will
    /// look for `~/.config/id/conf.toml`
    pub fn load_from_conf(id: &'static str,
                          author: &'static str,
                          default_config: conf::Conf)
                          -> GameResult<Context> {

        let mut fs = Filesystem::new(id, author)?;

        let config = fs.read_config().unwrap_or(default_config);

        Context::from_conf(config, fs)
    }

    /// Prints out information on the resources subsystem.
    pub fn print_resource_stats(&mut self) {
        if let Err(e) = self.filesystem.print_all() {
            println!("Error printing out filesystem info: {}", e)
        }
    }

    /// Triggers a Quit event.
    pub fn quit(&mut self) {
        self.event_context.interrupt();
    }
}
