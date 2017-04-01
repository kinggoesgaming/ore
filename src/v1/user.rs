use chrono::{DateTime, UTC};
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

/// The result type for `User` creation.
pub type UserResult<'a> = Result<User<'a>, super::super::Err<'a>>;

/// Represents a Ore user or organization.
#[derive(Clone, Debug)]
pub struct User<'a> {
    /// The avatar for the user.
    avatar: &'a Avatar<'a>,

    /// The date when the user was created.
    created_at: &'a DateTime<UTC>,

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
    pub fn get_avatar(&self) -> &'a Avatar<'a> {
        self.avatar
    }

    /// Gets the date and time when the user was created at.
    pub fn get_creation_date_time(&self) -> &DateTime<UTC> {
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

    // TODO: replace with from_json function
    /// Create a new instance of User.
    pub fn new(avatar: &'a Avatar<'a>,
               created_at: &'a DateTime<UTC>,
               id: &'a u32,
               roles: Vec<&'a str>,
               starred: Vec<&'a str>,
               username: &'a str)
               -> UserResult<'a> {
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

    pub fn from_json(json: &'a str) -> UserResult<'a> {
        // TODO: implement
        unimplemented!()
    }
}

impl<'a> Display for User<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               "User (avatar: {}, created at: {}, id: {}, projects: {}, roles: {:?}, starred: {:?},\
                username: {})",
               self.get_avatar(),
               self.get_creation_date_time(),
               self.get_id(),
               "", // TODO: apply the projects
               self.get_roles(),
               self.get_starred(),
               self.get_username())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Avatar<'a> {
    Avatar(&'a str),
    AvatarTemplate(&'a str),
}

impl<'a> Display for Avatar<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Avatar::Avatar(x) => write!(f, "Avatar: {}", x),
            Avatar::AvatarTemplate(x) => write!(f, "Avatar Template: {}", x),
        }
    }
}
