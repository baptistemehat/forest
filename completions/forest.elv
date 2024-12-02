
use builtin;
use str;

set edit:completion:arg-completer[forest] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'forest'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'forest'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand task 'Perform operations on tasks'
            cand tree 'Perform operations on trees'
            cand note 'Perform operations on notes'
            cand switch 'Switch to another tree'
            cand start 'Start recording time'
            cand stop 'Stop current time recording'
            cand status 'Show current time recording'
            cand report 'Reports time spent on each tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;task'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand list 'List tasks in the current tree'
            cand add 'Add a new task to the current tree'
            cand remove 'Remove a task from the current tree'
            cand rename 'Rename a task in the current tree'
            cand show 'Show description of a task in the current tree'
            cand edit 'Edit description of a task in the current tree'
            cand priority 'Set priority of a task in the current tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;task;list'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;add'= {
            cand -p 'Uid of parent task. By default, adds the task to the tree root'
            cand --parent 'Uid of parent task. By default, adds the task to the tree root'
            cand -d 'Description of the new task'
            cand --description 'Description of the new task'
            cand -e 'Opens an editor to write the description of the new task'
            cand --edit 'Opens an editor to write the description of the new task'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;remove'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;rename'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;show'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;edit'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;priority'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;task;help'= {
            cand list 'List tasks in the current tree'
            cand add 'Add a new task to the current tree'
            cand remove 'Remove a task from the current tree'
            cand rename 'Rename a task in the current tree'
            cand show 'Show description of a task in the current tree'
            cand edit 'Edit description of a task in the current tree'
            cand priority 'Set priority of a task in the current tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;task;help;list'= {
        }
        &'forest;task;help;add'= {
        }
        &'forest;task;help;remove'= {
        }
        &'forest;task;help;rename'= {
        }
        &'forest;task;help;show'= {
        }
        &'forest;task;help;edit'= {
        }
        &'forest;task;help;priority'= {
        }
        &'forest;task;help;help'= {
        }
        &'forest;tree'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand list 'List trees in the current forest'
            cand add 'Add a new tree'
            cand remove 'Remove a tree'
            cand rename 'Rename a tree'
            cand show 'Show description of a tree'
            cand edit 'Edit description of a tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;tree;list'= {
            cand -f 'Formatting options'
            cand --format 'Formatting options'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'forest;tree;add'= {
            cand -d 'Description of the new tree'
            cand --description 'Description of the new tree'
            cand -e 'Opens an editor to write the description of the new tree'
            cand --edit 'Opens an editor to write the description of the new tree'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;tree;remove'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;tree;rename'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;tree;show'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;tree;edit'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;tree;help'= {
            cand list 'List trees in the current forest'
            cand add 'Add a new tree'
            cand remove 'Remove a tree'
            cand rename 'Rename a tree'
            cand show 'Show description of a tree'
            cand edit 'Edit description of a tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;tree;help;list'= {
        }
        &'forest;tree;help;add'= {
        }
        &'forest;tree;help;remove'= {
        }
        &'forest;tree;help;rename'= {
        }
        &'forest;tree;help;show'= {
        }
        &'forest;tree;help;edit'= {
        }
        &'forest;tree;help;help'= {
        }
        &'forest;note'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand list 'List all notes'
            cand add 'Create a new note associated to the current tree'
            cand remove 'Remove a note'
            cand show 'Show content of a note'
            cand edit 'Edit a note'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;note;list'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;note;add'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;note;remove'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;note;show'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;note;edit'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;note;help'= {
            cand list 'List all notes'
            cand add 'Create a new note associated to the current tree'
            cand remove 'Remove a note'
            cand show 'Show content of a note'
            cand edit 'Edit a note'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;note;help;list'= {
        }
        &'forest;note;help;add'= {
        }
        &'forest;note;help;remove'= {
        }
        &'forest;note;help;show'= {
        }
        &'forest;note;help;edit'= {
        }
        &'forest;note;help;help'= {
        }
        &'forest;switch'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;start'= {
            cand --at 'Start date and time of recording'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;stop'= {
            cand --at 'Stop date and time of recording'
            cand -n 'Do not create a new note'
            cand --no-note 'Do not create a new note'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;status'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;report'= {
            cand -f 'Report recorded time from a specific date time Defaults to ...'
            cand --from 'Report recorded time from a specific date time Defaults to ...'
            cand -t 'Report recorded time to a specific date time Defaults to now'
            cand --to 'Report recorded time to a specific date time Defaults to now'
            cand -d 'Report recorded time for the current day'
            cand --day 'Report recorded time for the current day'
            cand -w 'Report recorded time for the current week'
            cand --week 'Report recorded time for the current week'
            cand -m 'Report recorded time for the current month'
            cand --month 'Report recorded time for the current month'
            cand -y 'Report recorded time for the current year'
            cand --year 'Report recorded time for the current year'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'forest;help'= {
            cand task 'Perform operations on tasks'
            cand tree 'Perform operations on trees'
            cand note 'Perform operations on notes'
            cand switch 'Switch to another tree'
            cand start 'Start recording time'
            cand stop 'Stop current time recording'
            cand status 'Show current time recording'
            cand report 'Reports time spent on each tree'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'forest;help;task'= {
            cand list 'List tasks in the current tree'
            cand add 'Add a new task to the current tree'
            cand remove 'Remove a task from the current tree'
            cand rename 'Rename a task in the current tree'
            cand show 'Show description of a task in the current tree'
            cand edit 'Edit description of a task in the current tree'
            cand priority 'Set priority of a task in the current tree'
        }
        &'forest;help;task;list'= {
        }
        &'forest;help;task;add'= {
        }
        &'forest;help;task;remove'= {
        }
        &'forest;help;task;rename'= {
        }
        &'forest;help;task;show'= {
        }
        &'forest;help;task;edit'= {
        }
        &'forest;help;task;priority'= {
        }
        &'forest;help;tree'= {
            cand list 'List trees in the current forest'
            cand add 'Add a new tree'
            cand remove 'Remove a tree'
            cand rename 'Rename a tree'
            cand show 'Show description of a tree'
            cand edit 'Edit description of a tree'
        }
        &'forest;help;tree;list'= {
        }
        &'forest;help;tree;add'= {
        }
        &'forest;help;tree;remove'= {
        }
        &'forest;help;tree;rename'= {
        }
        &'forest;help;tree;show'= {
        }
        &'forest;help;tree;edit'= {
        }
        &'forest;help;note'= {
            cand list 'List all notes'
            cand add 'Create a new note associated to the current tree'
            cand remove 'Remove a note'
            cand show 'Show content of a note'
            cand edit 'Edit a note'
        }
        &'forest;help;note;list'= {
        }
        &'forest;help;note;add'= {
        }
        &'forest;help;note;remove'= {
        }
        &'forest;help;note;show'= {
        }
        &'forest;help;note;edit'= {
        }
        &'forest;help;switch'= {
        }
        &'forest;help;start'= {
        }
        &'forest;help;stop'= {
        }
        &'forest;help;status'= {
        }
        &'forest;help;report'= {
        }
        &'forest;help;help'= {
        }
    ]
    $completions[$command]
}
