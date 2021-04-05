use chrono::NaiveDateTime;

#[derive(Clone, Eq, PartialEq)]
pub struct Page {
    name:String,
    id: u32,
    author: String,
    create_data: NaiveDateTime,
    last_edit_data: NaiveDateTime,
    size: u32,
    text: String,
    protection: Protection,
    tsid: TalkOrSubject,
    watched: bool,
    watchers_number: u32
}
// visitingwatchers
// notificationtimestamp

impl Page{
    pub fn new(
        name:String,
        id:u32,
        author: String,
        create_data: NaiveDateTime,
        last_edit_data: NaiveDateTime,
        size: u32,
        text: String,
        protection: Protection,
        tsid: TalkOrSubject,
        watched: bool,
        watchers_number: u32,
    ) -> Page{
        Page{
            name,
            id,
            author,
            create_data,
            last_edit_data,
            size,
            text,
            protection,
            tsid,
            watched,
            watchers_number
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum Protection {
    None,
    Whole(NaiveDateTime),
    Half(NaiveDateTime),
    White(NaiveDateTime),
    Move(NaiveDateTime),
}


#[derive(Eq, PartialEq, Clone)]
pub enum TalkOrSubject {
    Subject(u32),
    Talk(u32),
}

impl Page {
    pub fn get_author(&self) -> String {
        (&self.author).to_owned()
    }

    pub fn get_create_time(&self) -> NaiveDateTime {
        self.create_data
    }

    pub fn get_last_edit_time(&self) -> NaiveDateTime {
        self.last_edit_data
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_text(&self) -> String {
        (&self.text).to_owned()
    }
}