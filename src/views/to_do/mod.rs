use actix_web::web;
mod utils;
mod create;
mod get;
mod edit;
mod delete;


/// This function adds the to do item api routes to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn item_factory(app: &mut web::ServiceConfig) {
    app.route("api/item/{title}", web::post().to(create::create));

    app.route("api/item", web::get().to(get::get));

    app.route("api/item", web::put().to(edit::edit));

    app.route("api/item", web::delete().to(delete::delete));
}
