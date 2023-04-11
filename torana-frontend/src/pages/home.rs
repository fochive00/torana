
use yew::prelude::*;
use torana_common::types::RemonstranceData;
use uuid::Uuid;

// Home page is a functional component
#[function_component(Home)]
pub fn home() -> Html {
    // Create a list of RemonstranceData with default value
    // pub id: Uuid,
    // pub title: String,
    // pub description: String,
    // pub author: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
    // pub comments: Vec<Comment>,
    // pub source_view: Vec<CodeSession>,


    let remonstrances: Vec<RemonstranceData> = vec![
        { 
            let mut remon = RemonstranceData::default();
            remon.id =  Uuid::new_v4();
            remon.title = "Remonstrance 1".to_string();
            remon.description = "Remonstrance 1 description".to_string();
            remon.created_at = chrono::Utc::now();
            remon.updated_at = chrono::Utc::now();

            remon
        },
        { 
            let mut remon = RemonstranceData::default();
            remon.id =  Uuid::new_v4();
            remon.title = "Remonstrance 2".to_string();
            remon.description = "Remonstrance 2 description".to_string();
            remon.created_at = chrono::Utc::now();
            remon.updated_at = chrono::Utc::now();

            remon
        },
    ];

    // Create a list of Remonstrance components.
    let remonstrance_components = remonstrances
        .iter()
        .map(|remonstrance| {
            html! {
                <div>
                    <h1>{ &remonstrance.title }</h1>
                    <p>{ &remonstrance.description }</p>
                    <p>{ &remonstrance.created_at }</p>
                    <p>{ &remonstrance.updated_at }</p>
                </div>
            }
        })
        .collect::<Html>();

    // Return the list of Remonstrance components.
    html! {
        <div>
            <h1>{ "Home" }</h1>
            { remonstrance_components }
        </div>
    }
}