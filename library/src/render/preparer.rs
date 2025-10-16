use super::context::*;

use {axum::http::*, std::result::Result};

//
// RenderPreparer
//

/// [RenderedPage](super::RenderedPage) preparer.
#[allow(async_fn_in_trait)]
pub trait RenderPreparer {
    /// Prepare.
    async fn prepare<'this>(&self, context: &mut RenderContext<'this>) -> Result<(), StatusCode>;
}
