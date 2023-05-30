use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tyme_db::reminders::reminder::Reminder;

use crate::repository::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialReminder {
    pub time: NaiveDateTime,
    pub message: String,
}

#[post("/reminders")]
pub async fn create_reminder(
    db: web::Data<Database>,
    new_reminder: web::Json<PartialReminder>,
) -> HttpResponse {
    let r = new_reminder.into_inner();

    match r.create(&db.pool).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[get("/reminders/{id}")]
// pub async fn get_reminder_by_id(db: web::Data<Database>, id:
// web::Path<String>) -> HttpResponse {     let reminder =
// db.get_reminder_by_id(&id);

//     match reminder {
//         Some(reminder) => HttpResponse::Ok().json(reminder),
//         None => HttpResponse::NotFound().body("Reminder not found"),
//     }
// }

// #[get("/reminders")]
// pub async fn get_reminders(db: web::Data<Database>) -> HttpResponse {
//     let reminders = db.get_reminders();

//     HttpResponse::Ok().json(reminders)
// }

// #[delete("/reminders/{id}")]
// pub async fn delete_reminder_by_id(db: web::Data<Database>, id:
// web::Path<String>) -> HttpResponse {     let reminder =
// db.delete_reminder_by_id(&id);

//     match reminder {
//         Some(reminder) => HttpResponse::Ok().json(reminder),
//         None => HttpResponse::NotFound().body("Reminder not found"),
//     }
// }

// #[put("/reminders/{id}")]
// pub async fn update_reminder_by_id(
//     db: web::Data<Database>,
//     id: web::Path<String>,
//     updated_reminder: web::Json<Reminder>,
// ) -> HttpResponse {
//     let reminder = db.update_reminder_by_id(&id,
// updated_reminder.into_inner());

//     match reminder {
//         Some(reminder) => HttpResponse::Ok().json(reminder),
//         None => HttpResponse::NotFound().body("Reminder not found"),
//     }
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(create_reminder), /* .service(get_reminder_by_id)
                                                      * .service(get_reminders)
                                                      * .service(delete_reminder_by_id)
                                                      * .service(update_reminder_by_id), */
    );
}
