use iron::method;
use router::Router;
use controllers::Controllers;
use content::Content;

pub fn new(controllers: Controllers) -> Router {
    let mut router = Router::new();
    let content = Content::new();
    router.route(method::Get, "/admin", controllers.admin, "admin");
    // router.get("/", controllers.home, "homepage");
    // router.get("/*", controllers.page, "storefront");

    /* iterate over the content routes if any and add them to the existing router*/
    println!("is empty? {}", content.get_routers().is_empty());

    router
}