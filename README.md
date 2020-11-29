# imgcopy
Copy and deduplicate images

The cli tool will read the date from the exif metadata of the images it finds in the source directory and copy them into the target directory in a specific directory structure

```bash
   <year>  
     |  
     \--- <month>  
             |  
             \--- <day>  
                    |  
                    \--- img-files  
```
  
USAGE:  
    imgcopy [FLAGS] [OPTIONS] <target>  
   
ARGS:  
    <target>    Target directory  
  
FLAGS:  
    -f, --force         Supress confirmation if target directory is not empty  
    -h, --help          Prints help information  
    -m, --move-files    Move image files to taget directory instead of copy  
    -V, --version       Prints version information  
  
OPTIONS:  
    -s, --source <source>    Source directory  
