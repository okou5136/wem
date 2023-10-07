# wem
wirease iMproved

# About
This program was supposed to be the improved version of Wirease, a file generating program that I made and didn't make it to the actual release,
and this is where the name "Wirease iMproved" came from.
What this program basically does is to generate a filesystem based on a simple script called "wem script".
the intention of this program is:
- Automate the process of manually creating a bunch of files
- Reduce typos and increase the stability
- Increase the time effeciency

# How to use this program
1. To create a filesystem named *my_note* using a wem script *note_tmpl*
```
wem make note my_note
```
- Further more:
To specify the asset file, use -s/--source option
```
wem make note my_note -s your_asset_folder
```

# How to write wem script
commands used in wem script mainly consist of (with a few exceptions) two things:
- command
- name

these are separated with **:** like so:
command:name

in some cases, there could be extra things after this e.g. *{*, *=*,
additionally, some command line *file* can have possible subcommands e.g. *pre*

1. To create a directory, use dir command
```
dir:your_directorys_name
```
 - if you want to nest the file, use *{*:
 ```
 dir:your_directorys_name {
     // some process ...
 }
 ```
 you can also have the same result with this:
 ```
 dir:your_directorys_name{ // some process ... }
 ```
  - for readability's sake, 
  it is recommended that the number of the process(es) is as small as possible (preferably one)
  in this form

2. To create a file, use file command
```
file:your_files_name
```
 - this command can take *pre* as subcommand:
 ```
 file(pre:your_pretext):your_files_name
 ```

3.to define your own variables, use def command
```
def:your_vars_name = val_inside_of_var
```
 - this command uses *=* after the name of variable to insert value to the variable
 - you can define multiple variables at once:
 ```
 def:{
    var_one = value_one,
    var_two = value_two
 }
 ```
  - ATTENTION: value's name **cannot** be binded with double quotations

# Future plans
- add ref command to excerpt values from other file
