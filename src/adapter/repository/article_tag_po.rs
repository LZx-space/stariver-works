use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "article_rel_tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub article_id: Uuid,

    pub tag_id: Uuid,

    pub create_time: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::article_po::Entity",
        from = "Column::ArticleId",
        to = "super::article_po::Column::Id"
    )]
    Article,

    #[sea_orm(
        belongs_to = "super::tag_po::Entity",
        from = "Column::TagId",
        to = "super::tag_po::Column::Id"
    )]
    Tag,
}

impl ActiveModelBehavior for ActiveModel {}
