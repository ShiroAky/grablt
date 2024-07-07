use clap::{Arg, Command};
use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
// use std::env;
// mod update;
// use update;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // let token_api = std::env::var("GITHUB_TOKEN_API")
    //     .expect("No se encontró la variable de entorno GITHUB_TOKEN_API");

    // let latest_version = update::get_latest_version(&token_api);

    let matches = Command::new("GrabIt")
        .version("3.3.0")
        .about("Descarga archivos desde URL")
        .arg(
            Arg::new("url")
                .required(true)
                .num_args(1)
                .help("La URL del archivo a descargar"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("ouput")
                .num_args(1)
                .help("El nombre personalizado para guardar el archivo"),
        )
        .get_matches();

    let url = matches
        .get_one::<String>("url")
        .expect("Debe proporcionar una URL");
    let origin = matches.get_one::<String>("origin").cloned();

    let url_path = url.split('/').last().unwrap_or("archivo_descargado");
    let extension = if let Some(ext) = url_path.split('.').last() {
        format!(".{}", ext)
    } else {
        "".to_string() // Si no hay extensión
    };

    let filename = if let Some(origin) = origin {
        if origin.contains('.') {
            PathBuf::from_str(&origin).unwrap()
        } else {
            PathBuf::from_str(&format!("{}{}", origin, extension)).unwrap()
        }
    } else {
        PathBuf::from_str(url_path).unwrap()
    };

    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Error al descargar el archivo");

    if !response.status().is_success() {
        println!("Error al descargar el archivo: {}", response.status());
        return;
    }

    let content_length = response.content_length().unwrap_or(0);

    let progress_bar = ProgressBar::new(content_length);
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{msg} {spinner:20.white/gray} {bytes}/{total_bytes} ({eta})")
            .expect("Error al configurar la barra de progreso"),
    );
    progress_bar.set_message("Descargando");

    let file = File::create(&filename)
        .await
        .expect("No se pudo crear el archivo");
    let mut writer = BufWriter::new(file);

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("Error al leer datos del servidor");
        writer
            .write_all(&chunk)
            .await
            .expect("Error al escribir en el archivo");
        progress_bar.inc(chunk.len() as u64);
    }

    progress_bar.finish_with_message("Descarga completada");
    println!("Archivo descargado y guardado como {:?}", filename);
}
