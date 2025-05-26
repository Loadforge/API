use actix_web::web;
use crate::handlers::project_handler::{
    get_projects_handler,
    create_project_handler
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/project")
            .route("", web::get().to(get_projects_handler)) 
            .route("", web::get().to(create_project_handler))  

           
    );
}
