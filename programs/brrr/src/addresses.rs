//! Issue authority/withdraw authority addresses.
#![deny(missing_docs)]

use anchor_lang::prelude::*;

/// Issue authority address.
mod issue_authority {
    use anchor_lang::declare_id;

    declare_id!("BJ9L3jNu6tvrUxPHTMfwyA8Lgw2X6ky5bVNyDqiXSxgA");
}

/// Address of the issue authority to use for this Crate.
pub static ISSUE_AUTHORITY_ADDRESS: Pubkey = issue_authority::ID;

/// Bump seed of the above address.
pub const ISSUE_AUTHORITY_ADDRESS_BUMP: u8 = 255;

/// Signer seeds of the [ISSUE_AUTHORITY_ADDRESS].
pub static ISSUE_AUTHORITY_SIGNER_SEEDS: &[&[&[u8]]] =
    &[&[b"print", &[ISSUE_AUTHORITY_ADDRESS_BUMP]]];

/// Withdraw authority address.
mod withdraw_authority {
    use anchor_lang::declare_id;

    declare_id!("7Twx9JYz3gB4rF3h2cyUMnQWj9QEtmwviTvVD7xjAGEw");
}

/// Address of the withdraw authority to use for this Crate.
pub static WITHDRAW_AUTHORITY_ADDRESS: Pubkey = withdraw_authority::ID;

/// Bump seed of the above address.
pub const WITHDRAW_AUTHORITY_ADDRESS_BUMP: u8 = 255;

/// Signer seeds of the [WITHDRAW_AUTHORITY_ADDRESS].
pub static WITHDRAW_AUTHORITY_SIGNER_SEEDS: &[&[&[u8]]] =
    &[&[b"burn", &[WITHDRAW_AUTHORITY_ADDRESS_BUMP]]];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_issue_authority_address() {
        let (key, bump) = Pubkey::find_program_address(&[b"print"], &crate::ID);
        assert_eq!(key, ISSUE_AUTHORITY_ADDRESS);
        assert_eq!(bump, ISSUE_AUTHORITY_ADDRESS_BUMP);
    }

    #[test]
    fn test_withdraw_authority_address() {
        let (key, bump) = Pubkey::find_program_address(&[b"burn"], &crate::ID);
        assert_eq!(key, WITHDRAW_AUTHORITY_ADDRESS);
        assert_eq!(bump, WITHDRAW_AUTHORITY_ADDRESS_BUMP);
    }
}
