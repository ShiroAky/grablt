// Librería para obtener la versión más reciente de la aplicación desde GitHub
use reqwest;
use reqwest::Response;

pub async fn get_latest_version(token_api: &str) -> Result<String, reqwest::Error> {
    // Construir un cliente reqwest con la cabecera de autorización
    let client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::USER_AGENT,
                reqwest::header::HeaderValue::from_static("My User Agent"),
            );
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("token {}", token_api)).unwrap(),
            );
            headers
        })
        .build()?;

    // Hacer una solicitud GET a la API de GitHub
    let response = client
        .get("https://api.github.com/repos/ShiroAky/GrabLt/releases/latest")
        .send()
        .await?;

    // Verificar si la solicitud fue exitosa
    if response.status().is_success() {
        // Obtener el cuerpo de la respuesta como JSON
        let json = response.json::<serde_json::Value>().await?;
        // Obtener el número de versión más reciente del JSON
        let latest_version = json["tag_name"].as_str().unwrap_or_default().to_string();
        Ok(latest_version)
    } else {
        Err(reqwest::Error::new(
            reqwest::Error::Kind::Status,
            format!("Error: {}", response.status()),
        ))
    }
}
