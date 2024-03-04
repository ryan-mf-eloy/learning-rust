use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("{:-^50}","WEATHER CLI");

    const API_KEY: &str = "akyihGJbY7v0WIauhZPUFH6qG0BXE2dZ";

    let url = format!("http://dataservice.accuweather.com/forecasts/v1/minute?q=-23.6814347,-46.9249433&apikey={}&language=pt-br", API_KEY);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let info = &body[
            body
            .find("\"Phrase\":")
            .unwrap() + 9..body
            .find(",")
            .unwrap()
        ];

        println!("Clima: {}", info);
    } else {
        println!("Erro na requisição: {}", response.status());
    }

    println!("{:-^50}","-");

    Ok(())
}