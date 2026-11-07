use ece::EcKeyComponents;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::As;
#[cfg(feature = "serde")]
use serde_with::base64::Base64;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Credentials {
    pub android_id: u64,
    pub security_token: u64,
    pub gcm_token: String,
    #[cfg_attr(feature = "serde", serde(flatten, with = "serde_key_components"))]
    pub key_components: EcKeyComponents,
    #[cfg_attr(feature = "serde", serde(with = "As::<Base64>"))]
    pub auth_secret: [u8; 16],
    pub installation_id: String,
    pub installation_auth_token: String,
    pub installation_refresh_token: String,
}

#[cfg(feature = "serde")]
mod serde_key_components {
    use std::result::Result as StdResult;

    use ece::EcKeyComponents;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_with::base64::Base64;
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Deserialize, Serialize)]
    struct Inner {
        #[serde_as(as = "Base64")]
        public_key: Vec<u8>,
        #[serde_as(as = "Base64")]
        private_key: Vec<u8>,
    }

    pub(super) fn serialize<S: Serializer>(
        components: &EcKeyComponents,
        serializer: S,
    ) -> StdResult<S::Ok, S::Error> {
        let inner = Inner {
            public_key: components.public_key().to_vec(),
            private_key: components.private_key().to_vec(),
        };
        inner.serialize(serializer)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> StdResult<EcKeyComponents, D::Error> {
        let inner = Inner::deserialize(deserializer)?;
        Ok(EcKeyComponents::new(inner.private_key, inner.public_key))
    }
}
