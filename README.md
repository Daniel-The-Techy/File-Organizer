 Rust File Organizer CLI

A simple command-line file organizer built in Rust. This tool helps you quickly organize files in a directory by sorting them into categorized folders based on their extensions.

 Features

Organizes files into folders like Images, Documents, Videos, Music, and more

Uses Clap
 for clean and simple CLI argument parsing

Lightweight, fast, and reliable thanks to Rust

Cross-platform: Works on Linux, macOS, and Windows

🚀 Installation

Clone the repository and build it:

git clone https://github.com/yourusername/file-organizer.git
cd file-organizer
cargo build --release


This will create a binary in target/release/.

🛠 Usage

Run the tool with the path to the directory you want to organize:

cargo run ./Downloads


If no path is given, it will organize the current directory:

cargo run .

Example

Before:

Downloads/
 ├── photo1.jpg
 ├── document.pdf
 ├── video.mp4
 └── song.mp3


After running:

Downloads/
 ├── Images/
 │    └── photo1.jpg
 ├── Documents/
 │    └── document.pdf
 ├── Videos/
 │    └── video.mp4
 └── Music/
      └── song.mp3

📦 How It Works

The tool uses file extensions to determine categories:

.jpg, .png, .gif → Images/

.mp4, .avi, .mkv → Videos/

.mp3, .wav → Music/

.pdf, .docx, .txt → Documents/

Other files → Others/

(You can easily extend these rules in the source code.)

 Roadmap / Future Improvements

Config file for custom rules

Duplicate file handling

Logging and verbose output

Option to move or copy files

📜 License

This project is licensed under the MIT License
