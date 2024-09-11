use rexiv2::Metadata;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use tokio;
use tokio::task;
use chrono::{NaiveDateTime, FixedOffset, DateTime, TimeZone};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let dryrun = args.contains(&"--dry-run".to_string());
    let folder_path = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let entries = fs::read_dir(&folder_path)?;

    let mut tasks = vec![];

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let allowed_extensions = ["jpg", "CR3", "mp4", "MP4", "JPG"];


        if path.is_file() && path.extension().map(|ext| {
                allowed_extensions.contains(&ext.to_string_lossy().as_ref())
                })
        .unwrap_or(false) {
            println!("Found CR3 file: {:?}", path.file_name().unwrap());

            let dryrun = dryrun.clone();
            let path = path.clone();

            let task = tokio::spawn(async move {
                    if let Err(e) = task::block_in_place(|| rename_by_exif(&path, dryrun)) {
                    eprintln!("Error renaming file {:?}: {}", path.file_name().unwrap(), e);
                    }
                    });

            tasks.push(task);
        }
    }

    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}

fn rename_by_exif(file_path: &Path, dryrun: bool) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = Metadata::new_from_path(file_path)?;
    let date_time = metadata.get_tag_string("Exif.Photo.DateTimeOriginal").ok();
    let subsec = metadata.get_tag_string("Exif.Photo.SubSecTimeOriginal").ok();

    if let (Some(date_time), Some(subsec)) = (date_time, subsec) {
        let date_time_str: &str = date_time.as_str();
        let naive_dt = NaiveDateTime::parse_from_str(date_time_str, "%Y:%m:%d %H:%M:%S").expect("Failed to parse date");
        let timezone_offset = FixedOffset::east_opt(8 * 3600).expect("Failed to parse date");
        let date_time_with_timezone: DateTime<FixedOffset> = timezone_offset.from_local_datetime(&naive_dt).unwrap();
        let unix_timestamp = date_time_with_timezone.timestamp();

        let date_part = date_time.split_whitespace().collect::<Vec<&str>>()[0];
        let time_part = date_time.split_whitespace().collect::<Vec<&str>>()[1];
        let date_part_cleaned = date_part.replace(":", "");
        let time_part_cleaned = time_part.replace(":", "")[0..4].to_string();
        let formatted_timestamp = format!("{}{}0", unix_timestamp, subsec);
        let new_file_name = format!("{}-{}-{}.CR3", formatted_timestamp, date_part_cleaned, time_part_cleaned);
        let new_file_path = file_path.with_file_name(new_file_name.clone());

        if file_path != new_file_path {
            if new_file_path.exists() {
                println!("For file {:?}, A file with the new name already exists: {}",file_path, new_file_name);
            } else {
                if let Some(file_name) = file_path.file_name() {
                    println!("Renaming {:?} to {}",file_name, new_file_name);
                }
                if !dryrun {
                    fs::rename(file_path, new_file_path)?;
                    println!("Renamed to {}", new_file_name);
                } else {
                    println!("Skip rename {} for --dry-run", new_file_name);
                }
            }
        } else {
            println!("The paths are the same.");
        }
    } else {
        return Err(Box::new(Error::new(ErrorKind::Other, "Required EXIF data not found")));
    }

    Ok(())
}

