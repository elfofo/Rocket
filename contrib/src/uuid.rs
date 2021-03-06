extern crate uuid as uuid_ext;

use std::fmt;
use std::str::FromStr;
use std::ops::Deref;

use rocket::request::{FromParam, FromFormValue};
use rocket::http::RawStr;

pub use self::uuid_ext::ParseError as UuidParseError;

/// Implements `FromParam` and `FormFormValue` for accepting UUID values from
/// the [uuid](https://github.com/rust-lang-nursery/uuid) crate.
///
/// # Usage
///
/// To use, add the `uuid` feature to the `rocket_contrib` dependencies section
/// of your `Cargo.toml`:
///
/// ```toml
/// [dependencies.rocket_contrib]
/// version = "*"
/// default-features = false
/// features = ["uuid"]
/// ```
///
/// You can use the `UUID` type directly as a target of a dynamic parameter:
///
/// ```rust,ignore
/// #[get("/users/<id>")]
/// fn user(id: UUID) -> String {
///     format!("We found: {}", id)
/// }
/// ```
///
/// You can also use the `UUID` as a form value, including in query strings:
///
/// ```rust,ignore
/// #[derive(FromForm)]
/// struct UserQuery {
///     id: UUID
/// }
///
/// #[post("/user?<user_query>")]
/// fn user(user_query: UserQuery) -> String {
///     format!("User ID: {}", user_query.id)
/// }
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UUID(uuid_ext::Uuid);

impl UUID {
    /// Consumes the UUID wrapper returning the underlying `Uuid` type.
    ///
    /// # Example
    /// ```rust
    /// # extern crate rocket_contrib;
    /// # extern crate uuid;
    /// # use rocket_contrib::UUID;
    /// # use std::str::FromStr;
    /// # use uuid::Uuid;
    /// # fn main() {
    /// let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
    /// let real_uuid = Uuid::from_str(uuid_str).unwrap();
    /// let my_inner_uuid = UUID::from_str(uuid_str).unwrap().into_inner();
    /// assert_eq!(real_uuid, my_inner_uuid);
    /// # }
    /// ```
    #[inline(always)]
    pub fn into_inner(self) -> uuid_ext::Uuid {
        self.0
    }
}

impl fmt::Display for UUID {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> FromParam<'a> for UUID {
    type Error = UuidParseError;

    /// A value is successfully parsed if `param` is a properly formatted UUID.
    /// Otherwise, a `UuidParseError` is returned.
    #[inline(always)]
    fn from_param(param: &'a RawStr) -> Result<UUID, Self::Error> {
        param.parse()
    }
}

impl<'v> FromFormValue<'v> for UUID {
    type Error = &'v RawStr;

    /// A value is successfully parsed if `form_value` is a properly formatted
    /// UUID. Otherwise, the raw form value is returned.
    #[inline(always)]
    fn from_form_value(form_value: &'v RawStr) -> Result<UUID, &'v RawStr> {
        form_value.parse().map_err(|_| form_value)
    }
}

impl FromStr for UUID {
    type Err = UuidParseError;

    #[inline]
    fn from_str(s: &str) -> Result<UUID, Self::Err> {
        Ok(UUID(try!(s.parse())))
    }
}

impl Deref for UUID {
    type Target = uuid_ext::Uuid;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}

impl PartialEq<uuid_ext::Uuid> for UUID {
    #[inline(always)]
    fn eq(&self, other: &uuid_ext::Uuid) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod test {
    use super::uuid_ext;
    use super::{UUID, UuidParseError};
    use super::FromParam;
    use super::FromStr;

    #[test]
    fn test_from_str() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = UUID::from_str(uuid_str).unwrap();
        assert_eq!(uuid_str, uuid_wrapper.to_string())
    }

    #[test]
    fn test_from_param() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = UUID::from_param(uuid_str.into()).unwrap();
        assert_eq!(uuid_str, uuid_wrapper.to_string())
    }

    #[test]
    fn test_into_inner() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = UUID::from_param(uuid_str.into()).unwrap();
        let real_uuid: uuid_ext::Uuid = uuid_str.parse().unwrap();
        let inner_uuid: uuid_ext::Uuid = uuid_wrapper.into_inner();
        assert_eq!(real_uuid, inner_uuid)
    }

    #[test]
    fn test_partial_eq() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = UUID::from_param(uuid_str.into()).unwrap();
        let real_uuid: uuid_ext::Uuid = uuid_str.parse().unwrap();
        assert_eq!(uuid_wrapper, real_uuid)
    }

    #[test]
    fn test_from_param_invalid() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2p";
        let uuid_result = UUID::from_param(uuid_str.into());
        assert_eq!(uuid_result, Err(UuidParseError::InvalidLength(37)));
    }
}
