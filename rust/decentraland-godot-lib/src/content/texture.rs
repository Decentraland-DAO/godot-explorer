use godot::{
    builtin::{meta::ToGodot, Dictionary, GString},
    engine::{file_access::ModeFlags, global::Error, DirAccess, FileAccess, Image, ImageTexture},
    obj::Gd,
};

use crate::{
    godot_classes::promise::Promise,
    http_request::request_response::{RequestOption, ResponseType},
};

use super::{
    content_provider::ContentProviderContext,
    thread_safety::{reject_promise, resolve_promise},
};

pub async fn load_png_texture(
    url: String,
    absolute_file_path: String,
    get_promise: impl Fn() -> Option<Gd<Promise>>,
    ctx: ContentProviderContext,
) {
    if !FileAccess::file_exists(GString::from(&absolute_file_path)) {
        let request = RequestOption::new(
            0,
            url.clone(),
            http::Method::GET,
            ResponseType::ToFile(absolute_file_path.clone()),
            None,
            None,
            None,
        );

        match ctx.http_queue_requester.request(request, 0).await {
            Ok(_response) => {}
            Err(err) => {
                reject_promise(
                    get_promise,
                    format!(
                        "Error downloading png texture {url} ({absolute_file_path}): {:?}",
                        err
                    ),
                );
                return;
            }
        }
    }

    let Some(file) = FileAccess::open(GString::from(&absolute_file_path), ModeFlags::READ) else {
        reject_promise(
            get_promise,
            format!("Error opening png file {}", absolute_file_path),
        );
        return;
    };

    let bytes = file.get_buffer(file.get_length() as i64);
    drop(file);

    let mut image = Image::new();
    let err = image.load_png_from_buffer(bytes);
    if err != Error::OK {
        DirAccess::remove_absolute(GString::from(&absolute_file_path));
        let err = err.to_variant().to::<i32>();
        reject_promise(
            get_promise,
            format!("Error loading texture {absolute_file_path}: {}", err),
        );
        return;
    }

    let Some(texture) = ImageTexture::create_from_image(image.clone()) else {
        reject_promise(
            get_promise,
            format!("Error creating texture from image {}", absolute_file_path),
        );
        return;
    };

    let Some(promise) = get_promise() else {
        return;
    };

    let Ok(mut dict) = promise.bind().get_data().try_to::<Dictionary>() else {
        reject_promise(
            get_promise,
            format!("Error creating texture from image {}", absolute_file_path),
        );
        return;
    };

    dict.insert("image", image.to_variant());
    dict.insert("texture", texture.to_variant());
    resolve_promise(get_promise, None);
}
