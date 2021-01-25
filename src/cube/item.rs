use serde::{Deserialize, Serialize};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Internal
pub use crate::cube::{Result, Error};

pub type TaskItemId = String;

#[derive(Debug)]
pub enum Identifier {
    Id(TaskItemId),
    Index(usize)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskItem {
    name: String,
    detail: String,
    status: TaskStatus,
    created_stamp: (u64, u32)
    // Todo: history
}

impl TaskItem {
    fn __timestamp() -> (u64, u32) {
        // Using nano_secs as timestamp.
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        (time.as_secs(), time.subsec_nanos())
    }
    fn created(&self) -> SystemTime {
        let sec = self.created_stamp.0;
        let sub_nano = self.created_stamp.1;
        UNIX_EPOCH + Duration::from_nanos(sec*1e9 as u64 + sub_nano as u64)
    }
}

impl std::fmt::Display for TaskItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f,
"\
[{}]─┬─ {}
    └─ {}\
"
                 , if let TaskStatus::Done = self.status {'x'} else {' '}, self.name, self.detail)
    }
}

impl From<String> for TaskItem {
    fn from(name: String) -> Self {
        TaskItem {
            name,
            detail: String::from(""),
            status: TaskStatus::Pending,
            created_stamp: TaskItem::__timestamp()
        }
    }
}

impl From<&str> for TaskItem {
    fn from(name: &str) -> Self {
        TaskItem {
            name: String::from(name),
            detail: String::from(""),
            status: TaskStatus::Pending,
            created_stamp: TaskItem::__timestamp()
        }
    }
}

impl From<(String, String)> for TaskItem {
    fn from((name, detail): (String, String)) -> Self {
        TaskItem {
            name,
            detail,
            status: TaskStatus::Pending,
            created_stamp: TaskItem::__timestamp()
        }
    }
}

impl From<(&str, &str)> for TaskItem {
    fn from((name, detail): (&str, &str)) -> Self {
        TaskItem {
            name: String::from(name),
            detail: String::from(detail),
            status: TaskStatus::Pending,
            created_stamp: TaskItem::__timestamp()
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_variables)]

    use crate::cube::item::*;

    // #[test]
    fn task_item_init() {
        println!("test task_item: {:#?}", TaskItem::from(("Wait...", "for me...")));
    }

    // #[test]
    fn timestamp() {
        let item = TaskItem::from("");
        println!(
"Time created:
    {:?}
, item is:
{:#?}"
            , item.created(), item
        );
    }
}