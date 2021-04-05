use std::fmt::Formatter;
use crate::group::Group;
use crate::std::ven::Vec;

#[derive(Clone, Eq, PartialEq)]
pub struct User {
    name: String,
    gender: Gender,
    edit_times: u32,
    groups: Vec<Group>,
}

impl User {
    pub fn new(name: String, gender: Gender, edit_times:u32, groups:Vec<Group>) -> User{
        User{name, gender, edit_times, groups }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_gender(&self) -> Gender{
        self.gender.clone()
    }

    pub fn get_edit_times(&self) -> u32{
        self.edit_times
    }

    pub fn get_groups(&self) -> Vec<Group>{
        self.groups.clone()
    }

}

impl std::fmt::Display for Vec<Group>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i :Vec<String>= Vec::new();
        for each in self.groups{
            i.push(each)
        }
        write!(f, "{:?}", i.to_string())
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.to_owned())
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.name, self.edit_times, self.gender.to_string(), )
    }
}


#[derive(Clone, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Unknown => write!(f, "Unknown")
        }
    }
}

