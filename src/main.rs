use clap::Parser;
use colored::*;

use std::{ fs, io, path::{Path, PathBuf}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};


#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Organize files into folders", long_about = None, color = clap::ColorChoice::Always)]
struct Args {

   // #[arg(short, long)]
    path: Option<PathBuf>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(long)]
    dry_run:bool,
}

fn main() {


 let file=file_organize();


println!("{:?}", file);

}


  fn file_organize() -> io::Result<()>{
      
        let cli=Args::parse();

           let clean_folder=match cli.path {
             Some(paths) => paths,
             None => std::env::current_dir().unwrap(),
         };


       let new_path=Path::new(&clean_folder);

       let entry=fs::read_dir(&new_path)?;


            for entries in entry {
                  let entries=entries?;

                  let path=entries.path();

                    if path.is_file() {

                        let extension=path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase());

                        let extensions=   match extension {
                           Some(ext) if ext == "mp4" || ext == "mkv" => "videos",
                           Some(ext) if ext == "mp3" || ext == "wav" => "Audio",
                           Some(ext) if ext == "jpg" || ext == "png" => "image",
                           Some(ext) if ext == "doc" || ext == "txt" => "Document",
                           Some(ext) if ext == "js" || ext == "css" ||  ext == "html" => "Web",
                           _=>"Others"

               };
       

                        let updated_folders=new_path.join(extensions);


                           if !updated_folders.is_dir() {
                              let _ext_dir=fs::create_dir(&updated_folders);
                           }

                           let new_file=updated_folders.join(path.file_name().unwrap());


                           fs::rename(&path, &new_file);

                             if let Some(file) = path.file_name() {
                                  println!("{} -> {:?} -> {:?}", "Moving".green(), file, new_file);
                                    thread::sleep(Duration::from_secs(1));
                             }

                          

                           
                    }


            }

      

       Ok(())
  }