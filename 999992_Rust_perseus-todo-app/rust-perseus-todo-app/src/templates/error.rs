use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| match err {
        ClientError::ServerError { status, message: _ } => match status {
            404 => (
                view! { cx,
                        title { "Page not found"}
                },
                view! { cx,
                    p { "Sorry, that page doesn't seem to exist. "}
                },
            ),
        },
        ClientError::Panic(_) => (
            view! { cx,
                title { "Critical error"}
            },
            view! { cx,
                p { "Sorry, but a critical internal error has occured. This has been automatically reported to our team, who'll get on it as soon as possible. In the mean time, please try reloading the page."}
            },
        ),
        ClientError::FetchError(_) => {
            (view! { cx,
                p { "A network error occured, do you have an internet connection? (If you do, try reloading the page.)"}
            })
        }
        ClientError::PluginError(_) => todo!(),
        ClientError::InvariantError(_) => todo!(),
        ClientError::ThawError(_) => todo!(),
        ClientError::PlatformError(_) => todo!(),
        ClientError::PreloadError(_) => todo!(),
    })
}
