use futures::Stream;

use crate::client::Request;
use crate::models::streams::auto_paginated_stream;
use crate::models::{Account, Fandom};
use crate::requests::account::profile::{
    ChangeFollowRequest, GetCuratedFandomsRequest, GetFollowsRequest, GetModeratedFandomsRequest,
    GetSubscriptionsRequest,
};
use crate::{Client, Result};

impl Account {
    /// Follows this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to follow
    /// your own account, or [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn follow(&self, client: &Client) -> Result<&Self> {
        ChangeFollowRequest::new_follow(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Unfollows this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to
    /// unfollow your own account, or [`Error`][crate::Error] if any other error occurs during the
    /// request.
    pub async fn unfollow(&self, client: &Client) -> Result<&Self> {
        ChangeFollowRequest::new_unfollow(self.id)
            .send_request(client)
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
    pub fn get_follows<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                GetFollowsRequest::new_follows(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetFollowsRequest::PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of accounts that are following this account.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of accounts from the beginning
    /// of the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the
    /// stream will yield that single error and then terminate.
    pub fn get_followers<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                GetFollowsRequest::new_followers(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetFollowsRequest::PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of fandoms this account is subscribed to.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of fandoms from the beginning of
    /// the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_subscriptions<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                GetSubscriptionsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetSubscriptionsRequest::PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of fandoms this account moderates.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of fandoms from the beginning of
    /// the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_moderated_fandoms<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                GetModeratedFandomsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetModeratedFandomsRequest::PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of fandoms this account curates.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of fandoms from the beginning of
    /// the list. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_curated_fandoms<'a>(
        &'a self,
        client: &'a Client,
        offset: usize,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                GetCuratedFandomsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetCuratedFandomsRequest::PAGE_SIZE,
        )
    }
}
