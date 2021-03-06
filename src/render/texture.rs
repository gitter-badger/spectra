pub use luminance::pixel::{Depth32F, R32F, RGB32F, RGBA32F};
pub use luminance::texture::{Dim2, Flat, MagFilter, MinFilter, Sampler, Texture, Wrap};
use image;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use sys::resource::{CacheKey, Load, LoadError, LoadResult, Store, StoreKey};

// Common texture aliases.
pub type TextureRGB32F = Texture<Flat, Dim2, RGB32F>;
pub type TextureRGBA32F = Texture<Flat, Dim2, RGBA32F>;
pub type TextureR32F = Texture<Flat, Dim2, R32F>;
pub type TextureDepth32F = Texture<Flat, Dim2, Depth32F>;

/// Load an RGBA texture from an image at a path.
///
/// The `linearizer` argument is an option that gives the factor to apply to linearize if needed. Pass
/// `None` if the texture is already linearized.
pub fn load_rgba_texture<P>(path: P) -> Result<TextureRGBA32F, LoadError> where P: AsRef<Path> {
  let img = image::open(path).map_err(|e| LoadError::ConversionFailed(format!("{:?}", e)))?.flipv().to_rgba();
  let (w, h) = img.dimensions();
  let raw: Vec<f32> = img.into_raw().into_iter().map(|x| {
    x as f32 / 255.
  }).collect();

  let tex = Texture::new([w, h], 0, &Sampler::default()).map_err(|e| LoadError::ConversionFailed(format!("{:?}", e)))?;
  tex.upload_raw(false, &raw);

  Ok(tex)
}

/// Save an RGBA image on disk.
pub fn save_rgba_texture<P>(texture: &TextureRGBA32F, path: P) where P: AsRef<Path> {
  info!("saving texture image to: \x1b[35m{:?}", path.as_ref());

  let texels = texture.get_raw_texels();
  let [w, h] = texture.size();
  let mut output = Vec::with_capacity((w * h) as usize);

  for texel in &texels {
    output.push((texel * 255.) as u8);
  }

  let _ = image::save_buffer(path, &output, w, h, image::ColorType::RGBA(8));
}

#[derive(Debug)]
pub struct TextureImage(pub TextureRGBA32F);

impl Deref for TextureImage {
  type Target = TextureRGBA32F;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextureKey(pub String);

impl TextureKey {
  pub fn new(key: &str) -> Self {
    TextureKey(key.to_owned())
  }
}

impl<'a> From<&'a str> for TextureKey {
  fn from(key: &str) -> Self {
    TextureKey::new(key)
  }
}

impl CacheKey for TextureKey {
  type Target = TextureImage;
}

impl StoreKey for TextureKey {
  fn key_to_path(&self) -> PathBuf {
    self.0.clone().into()
  }
}

impl Load for TextureImage {
  type Key = TextureKey;

  fn load(key: &Self::Key, _: &mut Store) -> Result<LoadResult<Self>, LoadError> {
    let result = load_rgba_texture(key.key_to_path()).map(TextureImage)?.into();
    Ok(result)
  }
}
