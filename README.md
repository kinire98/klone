# klone
klone is a simple backup tool that saves the most recent copy of your files. If don't want to use a complex backup system and you only want a simple system for you to save the most recent copy of your files, just do:
```bash
(sudo) klone origin_dir target_dir
```
The objectives of this application:
As a user: make simple backups with the most recent copy of the files and access them in a simple way through the OS fs, without the need to use an external app:
1. Iterate through the directories.  
2. Get the times of modification of each file in the directory of origin directory.  
3. Get the times of modification of each file in the directory of target directory.  
4. Compare both. If the origin directory time is greater than the target directory time that means that there were some changes, so it's neccessary to make a backup.  
5. If a directory or a file doesn't exist in the target directory it will be created.  
6. If a directory or a file no longer exists in the origin directory two options: if nothing is addressed for this, it will just leave it there or you can delete it.  
7. All of this will be created in the most recent directory of the backup directory.

The directory structure will be like this 
```
|
 \_ backup_dir
              |
              |_ backup1
              |         |_origin_dir(week1)
              |
              |_ backup2
                        |_origin_dir(week2)
```
## Summary
A backup app that will store the most recent version of you files.  
You indicate a directory and inside that directory a copy will be created. If you don't say the opposite it will be in the same directory.  
You can tell the application to create a new directory so you can store the history of your file.  
## Maybes
- Make an option to exclude directories
- An option to store the configuration in a configuration file, so you don't have to indicate the paths or the exclusions all the time. (Maybe with a [tui](https://docs.rs/tui/latest/tui)). It will also be useful to show progress
- A way to make differential and incremental backups. This is improbable because it defeats the purpose of the application.