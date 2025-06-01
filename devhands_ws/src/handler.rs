use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Read;
use std::time::{Duration, Instant};

use hex;
use md5;

// Структура для параметров запроса задержки
#[derive(Deserialize)]
struct CpuDelayParams {
    delay_ms: u64,
}
//
// Обработчик GET запроса для "/"
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix-web!")
}
// Новый обработчик GET запроса с задержкой CPU
#[get("/cpu-load")]
async fn cpu_load(query: web::Query<CpuDelayParams>) -> impl Responder {
    // Получаем значение задержки из параметров запроса
    let delay_ms = query.delay_ms;
    let mut hash_hex: String = "".to_string();

    // Создаем нагрузку на CPU на указанное время
    let start = Instant::now();
    let mut bytes_hex: String = "".to_string();
    while start.elapsed() < Duration::from_millis(delay_ms) {
        // Бесполезные вычисления для загрузки CPU
        let random_bytes = read_random_bytes(128usize).unwrap();
        // Compute MD5 hash of the random bytes
        let hash_result = md5::compute(&random_bytes);
          hash_hex = hex::encode(hash_result.as_ref());

        // Преобразуем случайные байты в hex для отображения
       // bytes_hex = hex::encode(&random_bytes);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "message": "CPU load completed",
        "delay_ms": delay_ms,
        "hash_hex": hash_hex,
        "actual_duration_ms": start.elapsed().as_millis()
    }))
}

// Функция для чтения случайных байтов из /dev/urandom
fn read_random_bytes(bytes: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open("/dev/urandom")?;
    let mut buffer = vec![0u8; bytes];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}
