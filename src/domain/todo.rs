use chrono::{DateTime, Utc};


pub enum Status {
    PENDING,
    STARTED,
    DONE,
    PAUSED,
    ABORTED,
}

pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub create_date: DateTime<Utc>,
    pub done_date: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

// impl Todo {
//     pub fn set_title(&mut self, title: String) {
//         self.title = title;
//     }
    
//     pub fn set_description(&mut self, description: String) {
//         self.description = Some(description);
//     }

//     pub fn set_status(&mut self, status: Status) {
//         self.status = status;
//     }
    
//     pub fn set_done_date(&mut self, done_date: DateTime<Utc>) {
//         self.done_date = Some(done_date);
//     }
    
//     pub fn set_deadline(&mut self, deadline: DateTime<Utc>) {
//         self.deadline = Some(deadline);
//     }

//     pub fn add_tag(&mut self, tag: String) {
//         self.tags.push(tag);
//     }

//     pub fn remove_tag(&mut self, tag: String) -> Option<usize> {
//         let index = self.tags.iter().position(|x| x == &tag)?;
//         self.tags.remove(index);
//         Some(index)
//     }
// }