use futures::Stream;

use crate::client::Request;
use crate::models::Account;
use crate::models::streams::auto_paginated_stream;
use crate::requests::account::blocklist::{
    BlockAccountRequest, CheckAccountBlockedRequest, GetBlockedAccountsRequest,
    UnblockAccountRequest,
};
use crate::requests::fandom::blocklist::GetBlockedFandomIdsRequest;
use crate::{Client, Result};

impl Account {
    /// Blocks this account, hiding all its publications and disallowing direct messages from it.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to block
    /// your own account, or [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn block(&self, client: &Client) -> Result<&Self> {
        BlockAccountRequest::new(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Unblocks this account, making its publications reappear and allowing it to send you direct
    /// messages.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn unblock(&self, client: &Client) -> Result<&Self> {
        UnblockAccountRequest::new(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Checks if this account is currently blocked by you.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to check
    /// the block status of your own account, or [`Error`][crate::Error] if any other error occurs
    /// during the request.
    pub async fn check_blocked(&self, client: &Client) -> Result<bool> {
        Ok(CheckAccountBlockedRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }

    /// Retrieves a [`Stream`] of accounts that are currently blocked by this account.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of accounts from the beginning
    /// of the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the
    /// stream will yield that single error and then terminate.
    pub fn get_blocked_accounts<'a>(
        &'a self,
        client: &'a Client,
        offset: u64,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                GetBlockedAccountsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetBlockedAccountsRequest::PAGE_SIZE,
        )
    }

    /// Retrieves a list of IDs of fandoms that are currently blocked by this account.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_blocked_fandom_ids(&self, client: &Client) -> Result<Vec<u64>> {
        Ok(GetBlockedFandomIdsRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }
}
