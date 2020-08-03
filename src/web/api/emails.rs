use uuid::Uuid;
use crate::schema::emails;

#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Debug, juniper::GraphQLObject)]
#[table_name="emails"]
#[graphql(description="An email of an RCOS user.")]
pub struct Email {
    pub email: String,
    pub userid: Uuid
}