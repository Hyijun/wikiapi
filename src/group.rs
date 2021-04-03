pub(crate) struct Group{
    name: String,
    authority: Vec<Authority>
}


// only add some important authorities
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
    Block
}