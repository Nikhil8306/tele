# Synopsis: 
 > **tele** [subcommand] [options] [args]

# Desciption: 
Save Directories under a name and open it in your favourite editor

`- r`: recursive directory search instead of looking for predefined names

`- e | --editor string`: open directory in particular editor

# Commmands :
- ## Init:
    `init` command is used to initialize all the functionalities
    
- ## Save:
    `save` command is used to save directory in the tele storage
    - Options :
        - `-e | --editor string`: to save under particular editor (default vscode)
        - `-n | --name string`: for exlusively providing name other than parent directory's name
        - `-d | --dir string`: for exlusively telling which directory to save other than current one

- ## Remove: 
    `rm | remove` command is used to delete the saved directory from tele database

- ## Update: 
    `update` command is used to update the directory which is stored under some name
    - Options :
        - `-d | --dir string(required)`: for changing the directory
        - `-n | --name string(required)`: for changing the name

- ## List: 
    `ls` command is used to get the list of all saved directories
    - Options :
        - `-n | --name string`: filter based on the name
        - `-d | --dir string`: filter based on the directory

