use sqlx::PgPool;
use tokio::sync::Mutex;

pub struct Database {
    pub pool: Mutex<PgPool>,
}

// impl Database {
//     pub fn new() -> Self {
//         let reminders = Arc::new(Mutex::new(vec![]));
//         Database { reminders }
//     }

//     pub fn create_reminder(&self, reminder: Reminder) -> Result<Reminder,
// Error> {         let mut reminders = self.reminders.lock().unwrap();

//         let id = uuid::Uuid::new_v4().to_string();
//         let created_at = Utc::now();
//         let updated_at = Utc::now();

//         let reminder = Reminder {
//             id: Some(id),
//             created_at: Some(created_at),
//             updated_at: Some(updated_at),
//             ..reminder
//         };

//         reminders.push(reminder.clone());

//         Ok(reminder)
//     }

//     pub fn get_reminders(&self) -> Vec<Reminder> {
//         let reminders = self.reminders.lock().unwrap();

//         reminders.to_vec()
//     }

//     pub fn get_reminder_by_id(&self, id: &str) -> Option<Reminder> {
//         let reminders = self.reminders.lock().unwrap();

//         reminders
//             .iter()
//             .find(|reminder| reminder.id == Some(id.to_string()))
//             .cloned()
//     }

//     pub fn update_reminder_by_id(&self, id: &str, reminder: Reminder) ->
// Option<Reminder> {         let mut reminders =
// self.reminders.lock().unwrap();

//         let updated_at = Utc::now();

//         let reminder = Reminder {
//             id: Some(id.to_string()),
//             updated_at: Some(updated_at),
//             ..reminder
//         };

//         let index = reminders
//             .iter()
//             .position(|reminder| reminder.id == Some(id.to_string()))?;

//         reminders[index] = reminder.clone();

//         Some(reminder)
//     }

//     pub fn delete_reminder_by_id(&self, id: &str) -> Option<Reminder> {
//         let mut reminders = self.reminders.lock().unwrap();

//         let index = reminders
//             .iter()
//             .position(|reminder| reminder.id == Some(id.to_string()))?;

//         Some(reminders.remove(index))
//     }
// }
