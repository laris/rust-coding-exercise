use crate::data::AppDatabase;
use crate::service;
use crate::service::{action/*, ask*/};
use crate::web::{ctx, form, renderer::Renderer, PageError, PASSWORD_COOKIE/*, hitcounter*/};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;
use rocket::response::{status, Redirect};
use rocket::{uri, State};
//use rocket::http::uri::fmt::UriQueryArgument::Raw;
//use crate::web::ctx::PasswordRequired;
use crate::web::hitcounter::HitCounter;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

#[rocket::post("/", data = "<form>")]
pub async fn new_clip(
    form: Form<Contextual<'_, form::NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::ask::NewClip {
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        };
        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("internal error: {e}");
                Err((
                    Status::InternalServerError,
                    RawHtml(renderer.render(
                        ctx::Home::default(),
                        &["A server error occurred. Please try again"]
                    )),
                    ))
            }
        }
    } else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;
                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("unhandled error: {err}");
                    "An error occurred, please try again"
                }
            })
            .collect::<Vec<_>>();
        Err((
            Status::BadRequest,
            RawHtml(renderer.render_with_data(
                ctx::Home::default(),
                ("clip", &form.context),
                &errors)),
            ))
    }
}
#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    hit_counter: &State<HitCounter>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    fn render_with_status<T: ctx::PageContext + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer
        ) -> Result<status::Custom<RawHtml<String>>, PageError> {
        Ok(status::Custom(status, RawHtml(renderer.render(context, &[]))))
    }
    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        },
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            },
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_string())),
            _ => Err(PageError::Internal("server error".to_owned())),
        }
    }
}

#[rocket::post("/clip/<shortcode>", data = "<form>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::NewClip>>,
    shortcode: ShortCode,
    hit_counter: &State<HitCounter>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<RawHtml<String>, PageError> {
    if let Some(form) = &form.value {
        let req = service::ask::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };
        match action::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                hit_counter.hit(shortcode.clone(), 1);
                let context = ctx::ViewClip::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default()
                ));
                Ok(RawHtml(renderer.render(context, &[])))
            },
            Err(e) => match e {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                },
                ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
                _ => Err(PageError::Internal("server error".to_owned())),
            }
        }
    } else {
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(RawHtml(renderer.render(
            context,
            &["A password is required to view this clip"],
        )))
    }
}

#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: &str,
    hit_counter: &State<HitCounter>,
    database: &State<AppDatabase>
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;
    let req = service::ask::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_password| Password::new(raw_password.to_string()).ok())
            .flatten()
            .unwrap_or_else(||Password::default()),
    };
    match action::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.into(), 1);
            Ok(status::Custom(Status::Ok, clip.content.into_inner()))
        },
        Err(e) => match e {
            ServiceError::PermissionError(msg) => Ok(status::Custom(Status::Unauthorized, msg)),
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError)
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip, submit_clip_password, get_raw_clip]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {req:?}");
        "something went wrong..."
    }
    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {req:?}");
        "internal server error"
    }
    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}

#[cfg(test)]
pub mod test {
    use crate::data::AppDatabase;
    use crate::test::async_runtime;
    use crate::web::test::client;
    use rocket::http::Status;
    use crate::web::api::API_KEY_HEADER;

    #[test]
    fn gets_home() {
        let client = client();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn error_on_missing_clip() {
        let client = client();
        let response = client.get("/clip/notvalidpath").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn requires_password_when_applicable() {
        use crate::domain::clip::field::{Content, Expires, Password, Title};
        use crate::service;
        use rocket::http::{ContentType, Cookie, Header};

        let rt = async_runtime();

        let client = client();
        let db = client.rocket().state::<AppDatabase>().unwrap();

        let req = service::ask::NewClip {
            content: Content::new("content").unwrap(),
            expires: Expires::default(),
            password: Password::new("123".to_owned()).unwrap(),
            title: Title::default(),
        };
        let clip = rt
            .block_on(async move { service::action::new_clip(req, db.get_pool()).await})
            .unwrap();
        let response = client
            .get(format!("/clip/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .post(format!("/clip/{}", clip.shortcode.as_str()))
            .header(ContentType::Form)
            .body("password=123")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "123"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "abcd"))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        // api test, need fix
        /*
        let response = client
            //.post(format!("/api/clip/{}", clip.shortcode.as_str()))
            .get(format!("/api/clip/key"))
            //.header(ContentType::Form)
            //.header(Header::new(API_KEY_HEADER, api_key.to_base64))
            .dispatch();
        eprintln!("----------- header----------{:#?}", response.headers().get_one(API_KEY_HEADER));
        //assert_eq!(response.status(), Status::Ok);
        */
    }
}