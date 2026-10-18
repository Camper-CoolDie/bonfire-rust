use futures::Stream;

use crate::client::Request;
use crate::models::Account;
use crate::requests::account::blocklist::{
    BlockRequest, IsAccountBlockedRequest, ListBlockedAccountsRequest, UnblockRequest,
};
use crate::requests::fandom::blocklist::ListBlockedFandomIdsRequest;
use crate::{Client, Result};

impl Account {
    /// Blocks this account, hiding all its publications and disallowing direct messages from it.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if attempting to block
    /// the authenticated user's own account, or [`Error`][crate::Error] if any other error occurs
    /// during the request.
    pub async fn block(&self, client: &Client) -> Result<&Self> {
        BlockRequest::new(self.id).send(client).await?;
        Ok(self)
    }

    /// Unblocks this account, making its publications reappear and allowing it to send direct
    /// messages to the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn unblock(&self, client: &Client) -> Result<&Self> {
        UnblockRequest::new(self.id).send(client).await?;
        Ok(self)
    }

    /// Checks if this account is currently blocked by the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if attempting to check
    /// the block status of the authenticated user's own account, or [`Error`][crate::Error] if any
    /// other error occurs during the request.
    pub async fn is_blocked(&self, client: &Client) -> Result<bool> {
        IsAccountBlockedRequest::new(self.id).send(client).await
    }

    /// Retrieves a [`Stream`] of accounts that are currently blocked by this account.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of accounts from the beginning
    /// of the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the
    /// stream will yield that single error and then terminate.
    pub fn list_blocked_accounts<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        ListBlockedAccountsRequest::new(self.id, offset).stream(client)
    }

    /// Retrieves a list of IDs of fandoms that are currently blocked by this account.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn list_blocked_fandom_ids(&self, client: &Client) -> Result<Vec<u64>> {
        ListBlockedFandomIdsRequest::new(self.id).send(client).await
    }
}
