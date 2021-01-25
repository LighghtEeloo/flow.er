use serde::{Deserialize, Serialize};
use serde_json::{json};
use std::borrow::{BorrowMut};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// Internal
use crate::cube::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    item_map: HashMap<TaskItemId, TaskItem>,
    unite_model: Vec<TaskItemId>,
}

impl Collection {
    pub fn new() -> Self {
        Collection {
            item_map: HashMap::new(),
            unite_model: Vec::new(),
        }
    }
    pub fn from_json(json_str: String) -> Result<Self> {
        let json_res: Collection = serde_json::from_str(json_str.as_str())?;
        json_res.check_validity()?;
        Ok(json_res)
    }
    pub fn export_json(&self) -> String {
        let obj = json!(self);
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        obj.serialize(&mut ser).unwrap();
        let res = String::from_utf8(ser.into_inner()).unwrap();
        res
    }

    pub fn check_validity(&self) -> Result<()> {
        let valid = self.item_map.len() == self.unite_model.len()
            && self
                .unite_model
                .as_slice()
                .into_iter()
                .rfold(true, |b, x| self.item_map.contains_key(&*x) && b);
        if !valid {
            Err(Error::validity_fail())
        } else {
            Ok(())
        }
    }

    fn __timestamp_hash() -> String {
        // Using nano_secs as hash_key.
        let key = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        format!("{:x}", s.finish())
    }
    fn to_id(&self, id: Identifier) -> Result<TaskItemId> {
        match id {
            Identifier::Index(i) => {
                if let Some(id_) = self.unite_model.get(i) {
                    Ok(id_.clone())
                } else {
                    Err(Error::illegal_position(i, self.unite_model.len()))
                }
            }
            Identifier::Id(id_) => Ok(id_)
        }
    }

    pub fn add_item_index(&mut self, index: usize, item: TaskItem) -> Result<()> {
        let items = self.item_map.borrow_mut();
        let indices: &mut Vec<String> = self.unite_model.borrow_mut();
        let key = Collection::__timestamp_hash();
        match items.get(key.as_str()) {
            Some(it) => {
                println!("Can't add {:?} on the base of: {:?}", item, it);
                Err(Error::entry_exist())
            }
            _ => {
                if index > indices.len() {
                    println!("Can't add: {:?}", item);
                    Err(Error::illegal_position(index, indices.len()))
                } else {
                    println!("Added: {:?}", &item);
                    items.entry(key.to_string()).or_insert(item); // should always insert
                    indices.insert(index, key);
                    Ok(())
                }
            }
        }
    }
    pub fn add_last_item(&mut self, item: TaskItem) -> Result<()> {
        self.add_item_index(self.unite_model.len(), item)
    }
    pub fn query_item(&self, id: Identifier) -> Result<TaskItem> {
        let id = self.to_id(id)?;
        match self.item_map.get(&*id) {
            Some(it) => {
                Ok(it.clone())
            }
            None => {
                Err(Error::invalid_id())
            }
        }
    }
    pub fn list_item(&self) -> Result<Vec<TaskItem>> {
        self.check_validity()?;
        Ok(self.unite_model.clone()
            .into_iter()
            .map(|i| {self.item_map.get(&*i).expect("Check failed").clone()})
            .collect())
    }
    pub fn delete_item(&mut self, id: Identifier) -> Result<()> {
        let id = self.to_id(id)?;
        let items = self.item_map.borrow_mut();
        let indices: &mut Vec<String> = self.unite_model.borrow_mut();
        match items.remove(id.as_str()) {
            Some(d_item) => {
                let index = indices.into_iter().position(|x| x.as_str() == id.as_str());
                match index {
                    Some(i) => {
                        indices.remove(i);
                        println!("Deleted: {:?}", d_item);
                        Ok(())
                    }
                    _ => Err(Error::invalid_id()),
                }
            }
            _ => Err(Error::invalid_id()),
        }
    }
    pub fn delete_last_item(&mut self) -> Result<()> {
        self.delete_item(Identifier::Index(self.unite_model.len() - 1))
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_variables)]

    use crate::cube::collection::{TaskItem, Collection};

    // #[test]
    fn add_delete() {
        let mut tr: Collection = Collection::new();
        // tr.add_last_item(TaskItem::from_name("aaa".parse().unwrap())).expect("Add last item");
        // tr.add_last_item(TaskItem::from_name("aaa".parse().unwrap())).expect("Add last item");
    }

    use std::collections::HashSet;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn __timestamp_hash() -> String {
        // Using nano_secs as hash_key.
        // hash test.
        let key = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let key = key.as_nanos();
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let key = s.finish();
        format!("{:x}", key)
    }

    // Open this test if needed (which I think will not)
    // #[test]
    fn hash_test() {
        let mut ls: HashSet<String> = HashSet::new();
        let mut cnt = 0;
        for _x in 1..(1e7_f64 as u64) {
            if !ls.insert(__timestamp_hash().as_str()[..8].parse().unwrap()) {
                cnt += 1;
            }
        }
        println!("Total collision: {}.", cnt);
    }
}


