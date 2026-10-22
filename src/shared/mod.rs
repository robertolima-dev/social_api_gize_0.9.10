//! Who may change what. Written by hand: Gize generates resources where any signed-in user may
//! write any row (ADR-021); in a social network every row has an owner.

use crate::auth::Claims;

/// The caller may not touch this row. Each resource maps it to a 403.
#[derive(Debug)]
pub struct Forbidden;

/// The signed-in user's id. `require_auth` only lets through tokens this API issued, whose
/// subject is a user id, so anything else is refused rather than trusted.
pub fn caller(claims: &Claims) -> Result<uuid::Uuid, Forbidden> {
    claims.sub.parse().map_err(|_| Forbidden)
}

/// The caller owns the row, or is an admin (moderation).
pub fn ensure_owner(claims: &Claims, owner: uuid::Uuid) -> Result<(), Forbidden> {
    if claims.is_admin || caller(claims)? == owner {
        Ok(())
    } else {
        Err(Forbidden)
    }
}
