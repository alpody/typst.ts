use crate::utils::{console_log, AbsExt, CanvasStateGuard};
use crate::{sk, CanvasRenderTask};
use js_sys::Promise;
use typst::geom::Size;
use typst::image::{Image, ImageFormat, RasterFormat, VectorFormat};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlImageElement;

impl<'a> CanvasRenderTask<'a> {
    /// Render a raster or SVG image into the canvas.
    // todo: error handling, refactor, and verify correctness
    pub(crate) async fn render_image(
        &mut self,
        ts: sk::Transform,
        image: &Image,
        size: Size,
    ) -> Option<()> {
        let image_elem = rasterize_image(image).unwrap();
        self.load_image_cached(image, &image_elem).await;

        let state = CanvasStateGuard::new(self.canvas);
        self.canvas.reset_transform().unwrap();

        let view_width = size.x.to_f32();
        let view_height = size.y.to_f32();

        let aspect = (image.width() as f32) / (image.height() as f32);
        let scale = ts.sx.max(ts.sy);

        let x = ts.tx;
        let y = ts.ty;
        let w = (scale * view_width.max(aspect * view_height)).ceil() as u32;
        let h = ((w as f32) / aspect).ceil() as u32;
        self.canvas
            .draw_image_with_html_image_element_and_dw_and_dh(
                &image_elem,
                x as f64,
                y as f64,
                w as f64,
                h as f64,
            )
            .unwrap();
        drop(state);

        Some(())
    }

    async fn load_image_cached(&mut self, image: &Image, image_elem: &HtmlImageElement) {
        let image_loaded = image_elem.get_attribute("data-typst-loaded-image");
        match image_loaded {
            Some(t) if t == "true" => {
                console_log!("cached image loading {}x{}", image.width(), image.height());
            }
            _ => {
                self.load_image_slow(image, image_elem).await;
                image_elem
                    .set_attribute("data-typst-loaded-image", "true")
                    .unwrap();
            }
        }
    }

    async fn load_image_slow(&mut self, image: &Image, image_elem: &HtmlImageElement) {
        let u = js_sys::Uint8Array::new_with_length(image.data().len() as u32);
        u.copy_from(image.data());

        let parts = js_sys::Array::new();
        parts.push(&u);
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
            &parts,
            web_sys::BlobPropertyBag::new().type_(match image.format() {
                ImageFormat::Raster(e) => match e {
                    RasterFormat::Jpg => "image/jpeg",
                    RasterFormat::Png => "image/png",
                    RasterFormat::Gif => "image/gif",
                },
                ImageFormat::Vector(e) => match e {
                    VectorFormat::Svg => "image/svg+xml",
                },
            }),
        )
        .unwrap();

        let data_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

        let img_load_promise = Promise::new(
            &mut move |complete: js_sys::Function, _reject: js_sys::Function| {
                let data_url = data_url.clone();
                let data_url2 = data_url.clone();
                let complete2 = complete.clone();

                image_elem.set_src(&data_url);

                // simulate async callback from another thread
                let a = Closure::<dyn Fn()>::new(move || {
                    web_sys::Url::revoke_object_url(&data_url).unwrap();
                    complete.call0(&complete).unwrap();
                });

                image_elem.set_onload(Some(a.as_ref().unchecked_ref()));
                a.forget();

                let a = Closure::<dyn Fn(JsValue)>::new(move |e: JsValue| {
                    web_sys::Url::revoke_object_url(&data_url2).unwrap();
                    complete2.call0(&complete2).unwrap();
                    // let end = std::time::Instant::now();
                    console_log!(
                        "err image loading in {:?} {:?} {:?} {}",
                        // end - begin,
                        0,
                        js_sys::Reflect::get(&e, &"type".into()).unwrap(),
                        js_sys::JSON::stringify(&e).unwrap(),
                        data_url2,
                    );
                });

                image_elem.set_onerror(Some(a.as_ref().unchecked_ref()));
                a.forget();
            },
        );

        wasm_bindgen_futures::JsFuture::from(img_load_promise)
            .await
            .unwrap();
    }
}

#[comemo::memoize]
fn rasterize_image(_image: &Image) -> Option<HtmlImageElement> {
    let window = web_sys::window().unwrap();
    window
        .document()
        .unwrap()
        .create_element("img")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .ok()
}
