# imgcopy
Copy and deduplicate images (WIP)

This project supports 3 usage type:
1. As a command line tool
2. As a GUI tool
3. As a library

Both the CLI and the GUI version of the tool make use of the imgcopy library. But what does the library do? It will walk over the files in the source directory and read the original date from the exif metadata of the image, mp4 and mov files. Afterwards it will copy the files into the target directory in a specific directory structure.

```
   <year>
     |
     \--- <month>
             |
             \--- <day>
                    |
                    \--- img-files
```

Files that could not be identified as images or whose metadata could not be identified will be copied into the "ToDo" folder for you to manually process.


## CLI interface:

```bash
USAGE:
    imgcp [OPTIONS] <TARGET>

ARGS:
    <TARGET>    Target directory

OPTIONS:
    -f, --force              Suppress confirmation if target directory is not empty
    -h, --help               Print help information
    -l, --log                Write a log file
    -m, --move-files         Move image files to target directory instead of copy
    -s, --source <SOURCE>    Source directory
    -v, --verbose            Print info messages
    -V, --version            Print version information
```

## GUI interface
The GUI is more or less a guided dialog written with native-dialog.
