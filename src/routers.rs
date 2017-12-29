use router::Router;
use controllers::Controllers;

pub fn new(controllers: Controllers) -> Router {
    let mut router = Router::new();
    router.get("/", controllers.index, "index");
    router.get("/:query", controllers.home, "query");
    router
}
