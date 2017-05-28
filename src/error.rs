//! Error types and conversion functions.

use std;
use std::error::Error;
use std::fmt;

use gfx;

use image;
use rodio::decoder::DecoderError;
use glutin;
use app_dirs::AppDirsError;
use toml;
use zip;

/// An enum containing all kinds of game framework errors.
#[derive(Debug)]
pub enum GameError {
    FilesystemError(String),
    ConfigError(String),
    ResourceLoadError(String),
    ResourceNotFound(String, Vec<std::path::PathBuf>),
    RenderError(String),
    AudioError(String),
    WindowError(glutin::CreationError),
    ContextError(glutin::ContextError),
    IOError(std::io::Error),
    FontError(String),
    VideoError(String),
    UnknownError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::ResourceNotFound(ref s, ref paths) => {
                write!(f,
                       "Resource not found: {}, searched in paths {:?}",
                       s,
                       paths)
            }
            GameError::ConfigError(ref s) => write!(f, "Config error: {}", s),
            GameError::ResourceLoadError(ref s) => write!(f, "Error loading resource: {}", s),
            _ => write!(f, "GameError {:?}", self),
        }
    }
}

/// A convenient result type consisting of a return type and a `GameError`
pub type GameResult<T> = Result<T, GameError>;

/// Emit a non-fatal warning message
/// Ideally we probably want some sort of real logging interface here...
// fn warn(err: GameError) -> GameResult<()> {
//     println!("WARNING: Encountered error: {:?}", err);
//     Ok(())
// }

impl From<String> for GameError {
    fn from(s: String) -> GameError {
        GameError::UnknownError(s)
    }
}

impl From<glutin::CreationError> for GameError {
    fn from(s: glutin::CreationError) -> GameError {
        GameError::WindowError(s)
    }
}

impl From<glutin::ContextError> for GameError {
    fn from(s: glutin::ContextError) -> GameError {
        GameError::ContextError(s)
    }
}

impl From<AppDirsError> for GameError {
    fn from(e: AppDirsError) -> GameError {
        let errmessage = format!("{}", e);
        GameError::FilesystemError(errmessage)
    }
}
impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> GameError {
        GameError::IOError(e)
    }
}


impl From<toml::de::Error> for GameError {
    fn from(e: toml::de::Error) -> GameError {
        let errstr = format!("TOML decode error: {}", e.description());

        GameError::ConfigError(errstr)
    }
}

impl From<toml::ser::Error> for GameError {
    fn from(e: toml::ser::Error) -> GameError {
        let errstr = format!("TOML error (possibly encoding?): {}", e.description());
        GameError::ConfigError(errstr)
    }
}


impl From<zip::result::ZipError> for GameError {
    fn from(e: zip::result::ZipError) -> GameError {
        let errstr = format!("Zip error: {}", e.description());
        GameError::ResourceLoadError(errstr)
    }
}

impl From<DecoderError> for GameError {
    fn from(e: DecoderError) -> GameError {
        let errstr = format!("Audio decoder error: {:?}", e);
        GameError::AudioError(errstr)
    }
}

impl From<image::ImageError> for GameError {
    fn from(e: image::ImageError) -> GameError {
        let errstr = format!("Image load error: {}", e.description());
        GameError::ResourceLoadError(errstr)
    }
}

impl From<gfx::PipelineStateError<std::string::String>> for GameError {
    fn from(e: gfx::PipelineStateError<std::string::String>) -> GameError {
        let errstr = format!("Error constructing pipeline!\nThis should probably not be \
                              happening; it probably means an error in a shader or \
                              something.\nError was: {:?}",
                             e);
        GameError::VideoError(errstr)
    }
}

impl From<gfx::CombinedError> for GameError {
    fn from(e: gfx::CombinedError) -> GameError {
        let errstr = format!("Texture+view load error: {}", e.description());
        GameError::VideoError(errstr)
    }
}

impl<T> From<gfx::UpdateError<T>> for GameError
    where T: fmt::Debug + fmt::Display + 'static
{
    fn from(e: gfx::UpdateError<T>) -> GameError {
        let errstr = format!("Buffer update error: {}", e);
        GameError::VideoError(errstr)
    }
}
