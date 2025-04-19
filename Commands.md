# Synopsis: 
 > **tele** [Options] Command [args]

# Commmands :
- ## Init:
    `init` command is used to initialize all the functionalities
    
- ## Save:
    `save` command is used to save directory in the tele storage
    - Options :
        - `-e | --editor`: to save under particular editor
        - `-n | --name`: for exlusively providing name other than parent directory's name
        - `-d | --dir`: for exlusively telling which directory to save other than current one

- ## Remove: 
    `rm | remove` command is used to delete the saved directory in the tele database

- ## Update: 
    `update` command is used to update the directory which is stored under some name
    - Options :
        - `-d | --dir (required)`: for changing the directory
        - `-n | --name (required)`: for changing the name

- ## List: 
    `ls` command is used to get the list of all saved directories
    - Options :
        - `-n | --name`: filter based on the name
        - `-d | --dir`: filter based on the directory

