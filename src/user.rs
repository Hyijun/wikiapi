pub(crate) struct User{
    name: String,
    gender: Gender,
    edit_times: u32,
    groups: Vec<String>

}
enum Gender{
    Male,
    Female,
    Unknown
}

