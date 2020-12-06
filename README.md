# imgcopy
Copy and deduplicate images (WIP)

This project supports 3 usage type:
1. As command line tool
2. As a GUI tool
3. As a library

Both the CLI and the GUI version of the tool make use of the imgcopy library. But was does the library do? It will walk over the files in the source directory and read the original date from the exif metadata of the images it finds. Afterwards it will copy the image files into the target directory in a specific directory structure.

```
   <year>
     |
     \--- <month>
             |
             \--- <day>
                    |
                    \--- img-files
```

Files that could not be identified as images or whose exif metadata could not be identified will be copied into the "ToDo" folder for you to manually process.


## CLI interface:

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

## GUI interface
The GUI is written the platform independent OrbTK toolkit.
