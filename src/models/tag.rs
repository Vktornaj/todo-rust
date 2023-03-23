use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

use super::super::schema::_tag;


#[derive(Queryable, Insertable, Serialize, Deserialize, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = _tag)]
pub struct Tag {
    pub id: i32,
    pub tag_value: String,
}

impl Tag {
    pub fn attach(self) -> TagJson {
        TagJson {
            tag_value: self.tag_value
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagJson {
    pub tag_value: String,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = _tag)]
pub struct NewTag {
    pub tag_value: String,
}

impl NewTag {
    pub fn attach(self) -> NewTagJson {
        NewTagJson {
            tag_value: self.tag_value,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTagJson {
    pub tag_value: String,
}

impl NewTagJson {
    pub fn attach(self) -> NewTag {
        NewTag {
            tag_value: self.tag_value,
        }
    }
}