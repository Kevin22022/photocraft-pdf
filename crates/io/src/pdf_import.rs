//! PDF import: renders the first page to a PNG, which the flat importer then opens.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::vello_cpu::color::palette::css::WHITE;
use hayro::{render, RenderCache, RenderSettings};

use crate::IoError;

/// Renders page 1 of a PDF to PNG bytes. `scale` 1.0 = 72 DPI, 3.0 = about 216 DPI.
pub(crate) fn first_page_png(bytes: &[u8], scale: f32) -> Result<Vec<u8>, IoError> {
    let pdf = Pdf::new(bytes.to_vec()).map_err(|e| IoError::Read(format!("PDF: {e:?}")))?;
    let pages = pdf.pages();
    let page = pages.first().ok_or_else(|| IoError::Read("PDF: no pages".into()))?;
    let settings = RenderSettings { x_scale: scale, y_scale: scale, bg_color: WHITE, ..Default::default() };
    let cache = RenderCache::new();
    let pixmap = render(page, &cache, &InterpreterSettings::default(), &settings);
    pixmap.into_png().map_err(|e| IoError::Read(format!("PDF: {e:?}")))
}
