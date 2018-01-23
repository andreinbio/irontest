use iron::method;
use router::Router;
use mount::{Mount, OriginalUrl};
use staticfile::Static;
use std::path::Path;
use controllers::Controllers;
use content::Content;

pub fn new(controllers: Controllers) -> Mount {
    let mut router = Router::new();
    let content = Content::new();

    router.route(method::Get, "/admin", controllers.admin, "admin");

    /* iterate over the content routes if any and add them to the existing router*/
    for x in content.get_routers() {
        router.route(x.method, x.glob, x.handler, x.route_id);
    }

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static/", Static::new(Path::new("src/admin/static")));

    mount
}