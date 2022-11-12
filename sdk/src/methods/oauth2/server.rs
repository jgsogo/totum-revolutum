use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use reqwest;
use tokio::sync::oneshot::Sender;
use url::Url;

use crate::data::app_client_data::AppClientData;

use super::oauth2_token::{exchange_oauth2_token, OAuth2Token};

const CALLBACK_ENDPOINT: &str = "/callback";

struct AppContext {
    app: AppClientData,
    oauth2_token: Option<OAuth2Token>,
    tx: Option<Sender<()>>,
}

impl AppContext {
    fn new(app: AppClientData, tx: Sender<()>) -> AppContext {
        AppContext {
            app,
            oauth2_token: None,
            tx: Some(tx),
        }
    }
}

async fn dispatcher(
    http_client: reqwest::Client,
    req: Request<Body>,
    data: Arc<Mutex<AppContext>>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("Hello /"))),
        (&Method::GET, CALLBACK_ENDPOINT) => {
            let params: HashMap<String, String> = req
                .uri()
                .query()
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new);

            let oauth2_token = {
                let ctx = data.lock().unwrap().app.clone();
                let hostname = params.get("hostname").unwrap().clone();
                let code = params.get("code").unwrap().clone();
                exchange_oauth2_token(http_client, ctx, hostname, code).await?
            };
            {
                let mut data = data.lock().unwrap();
                data.oauth2_token = Some(oauth2_token);
                data.tx.take().unwrap().send(()).unwrap();
            }
            Ok(Response::new(Body::from(format!(
                "Hello /redirect_url qs:{params:#?}"
            ))))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn visit_url(app: &AppClientData, callback_url: String) -> String {
    let mut url = Url::parse("https://my.pcloud.com/oauth2/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &app.client_id);
    url.query_pairs_mut()
        .append_pair("redirect_uri", &callback_url);
    url.query_pairs_mut().append_pair("response_type", "code");
    if app.force_reapprove {
        url.query_pairs_mut().append_pair("force_reapprove", "true");
    }
    //url.query_pairs_mut().append_pair("state", "25");
    url.as_str().to_string()
}

pub(crate) async fn serve(
    http_client: reqwest::Client,
    app: AppClientData,
    addr: SocketAddr,
) -> Result<OAuth2Token, Box<dyn std::error::Error + Send + Sync>> {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let visit_url = visit_url(&app, format!("http://{addr}{CALLBACK_ENDPOINT}"));
    let app_context = Arc::new(Mutex::new(AppContext::new(app, tx)));

    let graceful = {
        let data = app_context.clone();
        let service = make_service_fn(move |_| {
            let data = data.clone();
            let http_client = http_client.clone();
            async move {
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(service_fn(move |req| {
                    dispatcher(http_client.clone(), req, data.clone())
                }))
            }
        });
        let server = Server::bind(&addr).serve(service);
        server.with_graceful_shutdown(async {
            rx.await.ok();
        })
    };

    println!("Listening on http://{addr}");
    println!("Visit {visit_url}");

    // Await the `server` receiving the signal...
    if let Err(e) = graceful.await {
        eprintln!("server error: {e}");
    }

    let xx = &mut app_context.lock().unwrap();
    Ok(xx.oauth2_token.take().unwrap())
}
