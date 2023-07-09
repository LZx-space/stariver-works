use sea_orm::entity::prelude::*;

/// 标签
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    pub create_time: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::article_po::Entity")]
    Article,
}

impl Related<super::article_po::Entity> for Entity {
    fn to() -> RelationDef {
        super::article_tag_po::Relation::Article.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::article_tag_po::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
