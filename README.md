# klone
klone is a simple backup tool that saves the most recent copy of your files. If don't want to use a complex backup system and you only want a simple system for you to save the most recent copy of your files, just do:
```bash
(sudo) klone origin_dir target_dir
```
The objectives of this application:
As a user: make simple backups with the most recent copy of the files and access them in a simple way through the OS fs, without the need to use an external app:
1. Iterate through the directories.  (DONE)
2. Get the times of modification of each file in the directory of origin directory.  (DONE)
3. Get the times of modification of each file in the directory of target directory.  (DONE)
4. Compare both. If the origin directory time is greater than the target directory time that means that there were some changes, so it's neccessary to make a backup.  (DONE)
5. If a directory or a file doesn't exist in the target directory it will be created.  (DONE)

The directory structure is intended to be something like this (is up to you to maintain a correct file structure):
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
- An option to store the configuration in a configuration file, so you don't have to indicate the paths or the exclusions all the time. (Maybe with a [tui](https://docs.rs/tui/latest/tui)). It will also be useful to show progress. As the project progress I see this option more viable to not bloat the program with lots of arguments
- Multithreading support
## TODOS
- Exclusions and configurations stored in files (also initial copy) -> DONE, only exclusions
- Cache the exclusions file for it to not be asking for the file again and again
- Add behaviour for Windows when file or dir doesn't exist -> cacache and tokio
- Change transfer of ownership to reference passing to avoid clones and improve performance time and memory wise
- Some tests, especially a integration one
- Look for unwraps and change them for proper error handling. If can't be done, use expect.
