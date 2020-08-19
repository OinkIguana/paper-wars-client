use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/paper-wars.graphql",
    query_path = "queries/auth.graphql"
)]
pub struct Authenticate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schemas/paper-wars.graphql",
    query_path = "queries/auth.graphql"
)]
pub struct Reauthenticate;
