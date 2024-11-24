use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/graphql/schema.graphql",
    query_path = "resources/graphql/contribution_years.graphql",
    response_derives = "Debug"
)]
pub struct ContributionYears;
