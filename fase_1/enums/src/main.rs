enum Message {
    Text(String),
    Image { url: String, width: u32, height: u32 },
    Video(String, u32), // (url, duração em segundos)
}

fn process_message(msg: Message) {
    match msg {
        Message::Text(texto) => {
            println!("Mensagem de texto: {}", texto);
        }
        Message::Image { url, width, height } => {
            println!("Imagem: {} ({}x{})", url, width, height);
        }
        Message::Video(url, duracao) => {
            println!("Vídeo: {} ({}s)", url, duracao);
        }
    }
}

fn main() {
    let msg1 = Message::Text(String::from("Olá, mundo!"));
    let msg2 = Message::Image {
        url: String::from("https://exemplo.com/imagem.png"),
        width: 800,
        height: 600,
    };
    let msg3 = Message::Video(String::from("https://exemplo.com/video.mp4"), 120);

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
}
