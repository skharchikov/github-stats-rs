use graphql_client::GraphQLQuery;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/graphql/schema.graphql",
    query_path = "resources/graphql/contribution_years.graphql",
    response_derives = "Debug"
)]
pub struct ContributionYears;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/graphql/schema.graphql",
    query_path = "resources/graphql/repos_overview.graphql",
    response_derives = "Debug"
)]
pub struct ReposOverview;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/graphql/schema.graphql",
    query_path = "resources/graphql/contributions_by_year.graphql",
    response_derives = "Debug"
)]
pub struct ContributionsByYear;
