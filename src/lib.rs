#[cfg(feature = "reader")]
mod bufreader;
#[cfg(feature = "writer")]
mod bufwriter;

#[cfg(feature = "reader")]
pub use bufreader::StackBufReader;
#[cfg(feature = "writer")]
pub use bufwriter::StackBufWriter;
