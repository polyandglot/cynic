pub fn send_query<'a, ResponseData: 'a, Root: cynic::QueryRoot>(
    selection_set: cynic::SelectionSet<'a, ResponseData, Root>,
) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let query = cynic::Query::new(selection_set);

    let response = reqwest::blocking::Client::new()
        .post("https://swapi-graphql.netlify.com/.netlify/functions/index")
        .json(&query)
        .send()
        .unwrap();

    let response_data = query.decode_response(response.json()?)?;
    if let Some(_errors) = response_data.errors {
        Err("GraphQL server returned errors")?
        // TODO: Better errors here
        //Err(errors)?
    }

    Ok(response_data.data.expect("response data"))
}