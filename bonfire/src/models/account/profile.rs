use futures::Stream;

use crate::client::Request;
use crate::models::{Account, Fandom};
use crate::requests::account::profile::{
    ChangeFollowRequest, ListCuratedFandomsRequest, ListFollowsRequest,
    ListModeratedFandomsRequest, ListSubscriptionsRequest,
};
use crate::{Client, Result};

impl Account {
    pub const STICKER_PACKS_MAX_COUNT: usize = 50;

    /// Follows this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if attempting to follow
    /// the authenticated user's own account, or [`Error`][crate::Error] if any other error occurs
    /// during the request.
    pub async fn follow(&self, client: &Client) -> Result<&Self> {
        ChangeFollowRequest::new_follow(self.id)
            .send(client)
            .await?;
        Ok(self)
    }

    /// Unfollows this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if attempting to
    /// unfollow the authenticated user's own account, or [`Error`][crate::Error] if any other error
    /// occurs during the request.
    pub async fn unfollow(&self, client: &Client) -> Result<&Self> {
        ChangeFollowRequest::new_unfollow(self.id)
            .send(client)
            .await?;
        Ok(self)
    }

    /// Retrieves a [`Stream`] of accounts this account is following.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of accounts from the beginning
    /// of the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the
    /// stream will yield that single error and then terminate.
    pub fn list_follows<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        ListFollowsRequest::new_follows(self.id, offset).stream(client)
    }

    /// Retrieves a [`Stream`] of accounts that are following this account.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of accounts from the beginning
    /// of the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the
    /// stream will yield that single error and then terminate.
    pub fn list_followers<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        ListFollowsRequest::new_followers(self.id, offset).stream(client)
    }

    /// Retrieves a [`Stream`] of fandoms this account is subscribed to.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of fandoms from the beginning of
    /// the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn subscriptions<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        ListSubscriptionsRequest::new(self.id, offset).stream(client)
    }

    /// Retrieves a [`Stream`] of fandoms this account moderates.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of fandoms from the beginning of
    /// the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn list_moderated_fandoms<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        ListModeratedFandomsRequest::new(self.id, offset).stream(client)
    }

    /// Retrieves a [`Stream`] of fandoms this account curates.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of fandoms from the beginning of
    /// the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn list_curated_fandoms<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        ListCuratedFandomsRequest::new(self.id, offset).stream(client)
    }
}
