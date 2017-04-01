/// Represents a Ore user or organization.
#[derive(Clone, Debug)]
pub struct User<'a> {

    avatar: Avatar<'a>,

    // TODO: switch to a chrono time struct?
    /// The date when the user was created.
    created_at: &'a str,

    /// The id of the user.
    id: &'a u32,

    // TODO: add project field

    /// The roles for the user.
    roles: Vec<&'a str>,

    /// The projects starred by the user.
    starred: Vec<&'a str>,

    /// The username of the user.
    username: &'a str,
}

impl<'a> User<'a> {

    /// Gets the avatar for the user.
    pub fn get_avatar(&self, size: u32) -> Avatar<'a> {
        match self.avatar {
            Avatar::Avatar(_) => self.avatar,
            Avatar::AvatarTemplate(x) => {
                let tmp: &'a str = x;
                tmp.replace("{size}", size.to_string().as_str()).as_str();
                Avatar::AvatarTemplate(tmp)
            }
        }
    }

    /// Gets the date and time when the user was created at.
    pub fn get_creation_date_time(&self) -> &str {
        self.created_at
    }

    /// Gets the id of the user.
    pub fn get_id(&self) -> &u32 {
        self.id
    }

    /// Gets the roles of the user.
    pub fn get_roles(&self) -> Vec<&str> {
        self.roles.to_vec()
    }

    /// Gets the starred projects by the user.
    pub fn get_starred(&self) -> Vec<&str> {
        self.starred.to_vec()
    }

    /// Gets the username of the user.
    pub fn get_username(&self) -> &str {
        self.username
    }

    /// Create a new instance of User.
    pub fn new(avatar: Avatar<'a>,
               created_at: &'a str,
               id: &'a u32,
               roles: Vec<&'a str>,
               starred: Vec<&'a str>,
               username: &'a str)
               -> Result<User<'a>, super::super::Err<'a>> {
        // TODO: add validation code
        Ok(User {
            avatar: avatar,
            created_at: created_at,
            id: id,
            roles: roles,
            starred: starred,
            username: username,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Avatar<'a> {
    Avatar(&'a str),
    AvatarTemplate(&'a str)
}
