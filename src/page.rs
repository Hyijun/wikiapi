use chrono::NaiveDateTime;

pub(crate) struct Page{
    author: String,
    create_data: NaiveDateTime,
    last_edit_data: NaiveDateTime,
    size: u32,
    
}