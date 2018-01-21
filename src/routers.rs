use iron::method;
use router::Router;
use controllers::Controllers;
use content::Content;

pub fn new(controllers: Controllers) -> Router {
    let mut router = Router::new();
    let content = Content::new();
    router.route(method::Get, "/admin", controllers.admin, "admin");

    /* iterate over the content routes if any and add them to the existing router*/
    for x in content.get_routers() {
        router.route(x.method, x.glob, x.handler, x.route_id);
    }

    router
}