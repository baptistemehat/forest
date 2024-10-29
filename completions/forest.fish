# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_forest_global_optspecs
	string join \n h/help V/version
end

function __fish_forest_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_forest_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_forest_using_subcommand
	set -l cmd (__fish_forest_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c forest -n "__fish_forest_needs_command" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_needs_command" -s V -l version -d 'Print version'
complete -c forest -n "__fish_forest_needs_command" -f -a "task" -d 'Perform operations on tasks'
complete -c forest -n "__fish_forest_needs_command" -f -a "tree" -d 'Perform operations on trees'
complete -c forest -n "__fish_forest_needs_command" -f -a "note" -d 'Perform operations on notes'
complete -c forest -n "__fish_forest_needs_command" -f -a "switch" -d 'Switch to another tree'
complete -c forest -n "__fish_forest_needs_command" -f -a "start" -d 'Start recording time'
complete -c forest -n "__fish_forest_needs_command" -f -a "stop" -d 'Stop current time recording'
complete -c forest -n "__fish_forest_needs_command" -f -a "status" -d 'Show current time recording'
complete -c forest -n "__fish_forest_needs_command" -f -a "report" -d 'Reports time spent on each tree'
complete -c forest -n "__fish_forest_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "list" -d 'List tasks in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "add" -d 'Add a new task to the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "remove" -d 'Remove a task from the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "rename" -d 'Rename a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "show" -d 'Show description of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "edit" -d 'Edit description of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "priority" -d 'Set priority of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and not __fish_seen_subcommand_from list add remove rename show edit priority help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from add" -s p -l parent -d 'Uid of parent task. By default, adds the task to the tree root' -r
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from add" -s d -l description -d 'Description of the new task' -r
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from add" -s e -l edit -d 'Opens an editor to write the description of the new task'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from rename" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from edit" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from priority" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "list" -d 'List tasks in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "add" -d 'Add a new task to the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove a task from the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "rename" -d 'Rename a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show description of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "edit" -d 'Edit description of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "priority" -d 'Set priority of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand task; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "list" -d 'List trees in the current forest'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "add" -d 'Add a new tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "remove" -d 'Remove a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "rename" -d 'Rename a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "show" -d 'Show description of a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "edit" -d 'Edit description of a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and not __fish_seen_subcommand_from list add remove rename show edit help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from list" -s f -l format -d 'Formatting options' -r -f -a "{short\t'only display tree names',long\t''}"
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from add" -s d -l description -d 'Description of the new tree' -r
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from add" -s e -l edit -d 'Opens an editor to write the description of the new tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from rename" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from edit" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "list" -d 'List trees in the current forest'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "add" -d 'Add a new tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "rename" -d 'Rename a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show description of a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "edit" -d 'Edit description of a tree'
complete -c forest -n "__fish_forest_using_subcommand tree; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -f -a "list" -d 'List all notes'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -f -a "add" -d 'Create a new note associated to the current tree'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -f -a "remove" -d 'Remove a note'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -f -a "show" -d 'Show content of a note'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -f -a "edit" -d 'Edit a note'
complete -c forest -n "__fish_forest_using_subcommand note; and not __fish_seen_subcommand_from list add remove show edit help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from list" -s t -l show-tt -d 'Show time tracking notes (hidden by default)'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from show" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from edit" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all notes'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from help" -f -a "add" -d 'Create a new note associated to the current tree'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove a note'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from help" -f -a "show" -d 'Show content of a note'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from help" -f -a "edit" -d 'Edit a note'
complete -c forest -n "__fish_forest_using_subcommand note; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand switch" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand start" -l at -d 'Start date and time of recording' -r
complete -c forest -n "__fish_forest_using_subcommand start" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand stop" -l at -d 'Stop date and time of recording' -r
complete -c forest -n "__fish_forest_using_subcommand stop" -s n -l no-note -d 'Do not create a new note'
complete -c forest -n "__fish_forest_using_subcommand stop" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand status" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand report" -s h -l help -d 'Print help'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "task" -d 'Perform operations on tasks'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "tree" -d 'Perform operations on trees'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "note" -d 'Perform operations on notes'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "switch" -d 'Switch to another tree'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "start" -d 'Start recording time'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "stop" -d 'Stop current time recording'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "status" -d 'Show current time recording'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "report" -d 'Reports time spent on each tree'
complete -c forest -n "__fish_forest_using_subcommand help; and not __fish_seen_subcommand_from task tree note switch start stop status report help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "list" -d 'List tasks in the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "add" -d 'Add a new task to the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "remove" -d 'Remove a task from the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "rename" -d 'Rename a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "show" -d 'Show description of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "edit" -d 'Edit description of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from task" -f -a "priority" -d 'Set priority of a task in the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from tree" -f -a "list" -d 'List trees in the current forest'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from tree" -f -a "add" -d 'Add a new tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from tree" -f -a "remove" -d 'Remove a tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from tree" -f -a "rename" -d 'Rename a tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from tree" -f -a "show" -d 'Show description of a tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from tree" -f -a "edit" -d 'Edit description of a tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from note" -f -a "list" -d 'List all notes'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from note" -f -a "add" -d 'Create a new note associated to the current tree'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from note" -f -a "remove" -d 'Remove a note'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from note" -f -a "show" -d 'Show content of a note'
complete -c forest -n "__fish_forest_using_subcommand help; and __fish_seen_subcommand_from note" -f -a "edit" -d 'Edit a note'
