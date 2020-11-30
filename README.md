# imgcopy
Copy and deduplicate images (WIP)

The cli tool will read the date from the exif metadata of the images it finds in the source directory and copy them into the target directory in a specific directory structure

```
   <year>  
     |  
     \--- <month>  
             |  
             \--- <day>  
                    |  
                    \--- img-files  
```
  
CLI interface:
```
USAGE:  
    imgcopy [FLAGS] [OPTIONS] <target>
   
ARGS:  
    <target>    Target directory  
  
FLAGS:  
    -f, --force         Suppress confirmation if target directory is not empty  
    -h, --help          Prints help information  
    -m, --move-files    Move image files to target directory instead of copy  
    -V, --version       Prints version information  
  
OPTIONS:  
    -s, --source <source>    Source directory  
```
