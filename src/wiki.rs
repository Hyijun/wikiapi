extern crate reqwest;
extern crate tokio;

use crate::page::Page;
use crate::user::User;
use crate::group::Group;

struct Wiki{
    pages:Vec<Page>,
    users:Vec<User>,
    groups:Vec<Group>
}
