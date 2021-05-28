use serenity::prelude::TypeMapKey;

use crate::database::ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = ConnectionPool;
}
