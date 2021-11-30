use webbrowser;

fn main() {
    println!("Openning...");
    if webbrowser::open("https://www.youtube.com/watch?v=dQw4w9WgXcQ").is_ok() {
        // ...
    }

    // ถ้าต้องการใช้ให้มันวน (Want to loop using)
    // loop {
    //     if webbrowser::open("https://www.youtube.com/watch?v=dQw4w9WgXcQ").is_ok() {
    //         // ...
    //     }
    // }
}
