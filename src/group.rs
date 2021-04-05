use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq)]
pub struct Group {
    name: String,
    authority: Vec<Authority>,
}

impl Group {
    pub fn new(name:String, authority:Vec<Authority>) -> Group {
        Group{name, authority}
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_authority(&self) -> Vec<Authority>{
        self.authority.clone()
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.to_owned())
    }
}

// only add some important authorities
#[derive(Copy, Clone, Eq, PartialEq)]
enum Authority {
    Edit,
    CreatePage,
    NoRateLimit,
    Upload,
    Move,
    MoveFile,
    AutoPatrol,
    Bot,
    CheckUser,
    Patrol,
    Rollback,
    Delete,
    Undelete,
    Block,
}

impl std::fmt::Display for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Authority::Edit => write!(f, "Edit"),
            Authority::CreatePage => write!(f, "CreatePage"),
            Authority::NoRateLimit => write!(f, "NoRateLimit"),
            Authority::Upload => write!(f, "Upload"),
            Authority::Move => write!(f, "Move"),
            Authority::MoveFile => write!(f, "MoveFile"),
            Authority::AutoPatrol => write!(f, "AutoPatrol"),
            Authority::Bot => write!(f, "Bot"),
            Authority::CheckUser => write!(f, "CheckUser"),
            Authority::Patrol => write!(f, "Patrol"),
            Authority::Rollback => write!(f, "Rollback"),
            Authority::Delete => write!(f, "Delete"),
            Authority::Undelete => write!(f, "Undelete"),
            Authority::Block => write!(f, "Block")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Groups {
    groups: Vec<Group>
}

impl Groups {
    fn new(groups: Vec<Group>) -> Groups {
        Groups{groups}
    }
}

impl std::fmt::Display for Groups {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = String::new();
        for each in self.groups{
            i = i + "" + each.to_string().as_str()
        }
        write!(f, "{}", i)
    }
}