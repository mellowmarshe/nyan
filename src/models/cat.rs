use std::{
    fmt::{self, Display},
    str::FromStr,
};

use chrono::NaiveDateTime;
use futures::future::LocalBoxFuture;
use sqlx::{FromRow, PgPool};

#[derive(sqlx::Type, Clone, Copy, Debug, PartialEq)]
#[sqlx(type_name = "Color", rename_all = "lowercase")]
pub enum Color {
    Brown,
    Grey,
    White,
}

#[derive(sqlx::Type, Clone, Copy, Debug, PartialEq)]
#[sqlx(type_name = "Area", rename_all = "lowercase")]
pub enum Area {
    Replenish,
    Battle,
    Rest,
}

#[derive(FromRow, Clone, Debug)]
pub struct Cat {
    pub id: i64,
    pub owner: i64,
    pub name: String,
    pub color: Color,
    pub area: Area,
    pub treats: i64,
    pub heterochromia: bool,
    pub created: NaiveDateTime,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Brown => write!(f, "brown"),
            Self::Grey => write!(f, "grey"),
            Self::White => write!(f, "white"),
        }
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Replenish => write!(f, "replenish"),
            Self::Battle => write!(f, "battle"),
            Self::Rest => write!(f, "rest"),
        }
    }
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "brown" => Ok(Self::Brown),
            "grey" => Ok(Self::Grey),
            "white" => Ok(Self::White),
            _ => Err(()),
        }
    }
}

impl FromStr for Area {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "replenish" => Ok(Self::Replenish),
            "battle" => Ok(Self::Battle),
            "rest" => Ok(Self::Rest),
            _ => Err(()),
        }
    }
}

impl Cat {
    pub async fn add_cat(
        pool: &PgPool,
        owner: i64,
        name: &str,
        color: Color,
        heterochromia: bool,
    ) -> Result<Cat, sqlx::Error> {
        sqlx::query_as::<_, Cat>(
            r#"INSERT INTO public.cats("owner", "name", "color", "heterochromia") 
        VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(owner)
        .bind(name)
        .bind(color)
        .bind(heterochromia)
        .fetch_one(pool)
        .await
    }

    pub async fn get_cat(pool: &PgPool, owner: i64) -> Result<Cat, sqlx::Error> {
        sqlx::query_as::<_, Cat>("SELECT * FROM public.cats WHERE owner = $1")
            .bind(owner)
            .fetch_one(pool)
            .await
    }
}
