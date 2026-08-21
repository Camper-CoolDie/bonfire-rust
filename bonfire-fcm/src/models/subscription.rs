use std::collections::VecDeque;

use ece::EcKeyComponents;

#[derive(Clone, Debug)]
pub struct Subscription {
    // Used only in logs to keep track of multiple listeners, usually set to the account's
    // identifier which will have the push token
    pub id: u64,
    pub push_token: String,
    pub key_components: EcKeyComponents,
    pub auth_secret: [u8; 16],
    pub persistent_ids: VecDeque<String>,
}
impl Subscription {
    // https://firebase.google.com/docs/cloud-messaging/android/receive-messages#override-on-deleted-messages
    pub(crate) const PERSISTENT_IDS_MAX_COUNT: usize = 100;
}

#[cfg(feature = "serde")]
mod inner_serde {
    use std::collections::VecDeque;
    use std::fmt;
    use std::result::Result as StdResult;

    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD_NO_PAD;
    use ece::EcKeyComponents;
    use serde::de::{IgnoredAny, MapAccess, Visitor};
    use serde::ser::SerializeStruct as _;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::Subscription;

    impl Serialize for Subscription {
        fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let public_key = STANDARD_NO_PAD.encode(self.key_components.public_key());
            let private_key = STANDARD_NO_PAD.encode(self.key_components.private_key());
            let auth_secret = STANDARD_NO_PAD.encode(self.auth_secret);

            let mut subscription = serializer.serialize_struct("Subscription", 6)?;
            subscription.serialize_field("id", &self.id)?;
            subscription.serialize_field("push_token", &self.push_token)?;
            subscription.serialize_field("public_key", &public_key)?;
            subscription.serialize_field("private_key", &private_key)?;
            subscription.serialize_field("auth_secret", &auth_secret)?;
            subscription.serialize_field("persistent_ids", &self.persistent_ids)?;
            subscription.end()
        }
    }

    impl<'de> Deserialize<'de> for Subscription {
        fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "id",
                "push_token",
                "public_key",
                "private_key",
                "auth_secret",
                "persistent_ids",
            ];
            deserializer.deserialize_struct("Subscription", FIELDS, SubscriptionVisitor)
        }
    }

    fn decode_base64<'de, A: MapAccess<'de>>(
        name: &'static str,
        value: String,
    ) -> StdResult<Vec<u8>, A::Error> {
        STANDARD_NO_PAD.decode(value).map_err(|error| {
            serde::de::Error::custom(format!("failed to decode field `{name}`: {error}"))
        })
    }

    struct SubscriptionVisitor;

    impl<'de> Visitor<'de> for SubscriptionVisitor {
        type Value = Subscription;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct Subscription")
        }

        fn visit_map<A>(self, mut map: A) -> StdResult<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut id = None;
            let mut push_token = None;
            let mut public_key = None;
            let mut private_key = None;
            let mut auth_secret = None;
            let mut persistent_ids = Option::<VecDeque<_>>::None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "id" => {
                        if id.is_some() {
                            return Err(serde::de::Error::duplicate_field("id"));
                        }
                        id = Some(map.next_value()?);
                    }
                    "push_token" => {
                        if push_token.is_some() {
                            return Err(serde::de::Error::duplicate_field("push_token"));
                        }
                        push_token = Some(map.next_value()?);
                    }
                    "public_key" => {
                        if public_key.is_some() {
                            return Err(serde::de::Error::duplicate_field("public_key"));
                        }
                        public_key = Some(decode_base64::<A>("public_key", map.next_value()?)?);
                    }
                    "private_key" => {
                        if private_key.is_some() {
                            return Err(serde::de::Error::duplicate_field("private_key"));
                        }
                        private_key = Some(decode_base64::<A>("private_key", map.next_value()?)?);
                    }
                    "auth_secret" => {
                        if auth_secret.is_some() {
                            return Err(serde::de::Error::duplicate_field("auth_secret"));
                        }
                        auth_secret = Some(decode_base64::<A>("auth_secret", map.next_value()?)?);
                    }
                    "persistent_ids" => {
                        if persistent_ids.is_some() {
                            return Err(serde::de::Error::duplicate_field("persistent_ids"));
                        }
                        persistent_ids = Some(map.next_value()?);
                    }
                    _ => {
                        let _ = map.next_value::<IgnoredAny>()?;
                    }
                }
            }

            Ok(Subscription {
                id: id.ok_or(serde::de::Error::missing_field("id"))?,
                push_token: push_token.ok_or(serde::de::Error::missing_field("push_token"))?,
                key_components: EcKeyComponents::new(
                    private_key.ok_or(serde::de::Error::missing_field("private_key"))?,
                    public_key.ok_or(serde::de::Error::missing_field("public_key"))?,
                ),
                auth_secret: auth_secret
                    .ok_or(serde::de::Error::missing_field("auth_secret"))?
                    .try_into()
                    .map_err(|error| {
                        serde::de::Error::custom(format!(
                            "failed to convert field `auth_secret` into [u8; 16] ({error:?})"
                        ))
                    })?,
                persistent_ids: match persistent_ids {
                    Some(mut ids) => {
                        ids.reserve_exact(Subscription::PERSISTENT_IDS_MAX_COUNT);
                        ids
                    }
                    None => return Err(serde::de::Error::missing_field("persistent_ids")),
                },
            })
        }
    }
}
