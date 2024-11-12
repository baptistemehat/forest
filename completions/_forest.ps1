
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'forest' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'forest'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'forest' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('task', 'task', [CompletionResultType]::ParameterValue, 'Perform operations on tasks')
            [CompletionResult]::new('tree', 'tree', [CompletionResultType]::ParameterValue, 'Perform operations on trees')
            [CompletionResult]::new('note', 'note', [CompletionResultType]::ParameterValue, 'Perform operations on notes')
            [CompletionResult]::new('switch', 'switch', [CompletionResultType]::ParameterValue, 'Switch to another tree')
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Start recording time')
            [CompletionResult]::new('stop', 'stop', [CompletionResultType]::ParameterValue, 'Stop current time recording')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show current time recording')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Reports time spent on each tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;task' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List tasks in the current tree')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new task to the current tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a task from the current tree')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename a task in the current tree')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show description of a task in the current tree')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit description of a task in the current tree')
            [CompletionResult]::new('priority', 'priority', [CompletionResultType]::ParameterValue, 'Set priority of a task in the current tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;task;list' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;add' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Uid of parent task. By default, adds the task to the tree root')
            [CompletionResult]::new('--parent', '--parent', [CompletionResultType]::ParameterName, 'Uid of parent task. By default, adds the task to the tree root')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Description of the new task')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'Description of the new task')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Opens an editor to write the description of the new task')
            [CompletionResult]::new('--edit', '--edit', [CompletionResultType]::ParameterName, 'Opens an editor to write the description of the new task')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;remove' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;rename' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;show' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;edit' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;priority' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;task;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List tasks in the current tree')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new task to the current tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a task from the current tree')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename a task in the current tree')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show description of a task in the current tree')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit description of a task in the current tree')
            [CompletionResult]::new('priority', 'priority', [CompletionResultType]::ParameterValue, 'Set priority of a task in the current tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;task;help;list' {
            break
        }
        'forest;task;help;add' {
            break
        }
        'forest;task;help;remove' {
            break
        }
        'forest;task;help;rename' {
            break
        }
        'forest;task;help;show' {
            break
        }
        'forest;task;help;edit' {
            break
        }
        'forest;task;help;priority' {
            break
        }
        'forest;task;help;help' {
            break
        }
        'forest;tree' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List trees in the current forest')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a tree')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename a tree')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show description of a tree')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit description of a tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;tree;list' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Formatting options')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Formatting options')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'forest;tree;add' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Description of the new tree')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'Description of the new tree')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Opens an editor to write the description of the new tree')
            [CompletionResult]::new('--edit', '--edit', [CompletionResultType]::ParameterName, 'Opens an editor to write the description of the new tree')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;tree;remove' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;tree;rename' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;tree;show' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;tree;edit' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;tree;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List trees in the current forest')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a tree')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename a tree')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show description of a tree')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit description of a tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;tree;help;list' {
            break
        }
        'forest;tree;help;add' {
            break
        }
        'forest;tree;help;remove' {
            break
        }
        'forest;tree;help;rename' {
            break
        }
        'forest;tree;help;show' {
            break
        }
        'forest;tree;help;edit' {
            break
        }
        'forest;tree;help;help' {
            break
        }
        'forest;note' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all notes')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Create a new note associated to the current tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a note')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show content of a note')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit a note')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;note;list' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Show time tracking notes (hidden by default)')
            [CompletionResult]::new('--show-tt', '--show-tt', [CompletionResultType]::ParameterName, 'Show time tracking notes (hidden by default)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;note;add' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;note;remove' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;note;show' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;note;edit' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;note;help' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all notes')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Create a new note associated to the current tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a note')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show content of a note')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit a note')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;note;help;list' {
            break
        }
        'forest;note;help;add' {
            break
        }
        'forest;note;help;remove' {
            break
        }
        'forest;note;help;show' {
            break
        }
        'forest;note;help;edit' {
            break
        }
        'forest;note;help;help' {
            break
        }
        'forest;switch' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;start' {
            [CompletionResult]::new('--at', '--at', [CompletionResultType]::ParameterName, 'Start date and time of recording')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;stop' {
            [CompletionResult]::new('--at', '--at', [CompletionResultType]::ParameterName, 'Stop date and time of recording')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Do not create a new note')
            [CompletionResult]::new('--no-note', '--no-note', [CompletionResultType]::ParameterName, 'Do not create a new note')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;status' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;report' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--from', '--from', [CompletionResultType]::ParameterName, 'from')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--to', '--to', [CompletionResultType]::ParameterName, 'to')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--day', '--day', [CompletionResultType]::ParameterName, 'day')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'forest;help' {
            [CompletionResult]::new('task', 'task', [CompletionResultType]::ParameterValue, 'Perform operations on tasks')
            [CompletionResult]::new('tree', 'tree', [CompletionResultType]::ParameterValue, 'Perform operations on trees')
            [CompletionResult]::new('note', 'note', [CompletionResultType]::ParameterValue, 'Perform operations on notes')
            [CompletionResult]::new('switch', 'switch', [CompletionResultType]::ParameterValue, 'Switch to another tree')
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Start recording time')
            [CompletionResult]::new('stop', 'stop', [CompletionResultType]::ParameterValue, 'Stop current time recording')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show current time recording')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Reports time spent on each tree')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'forest;help;task' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List tasks in the current tree')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new task to the current tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a task from the current tree')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename a task in the current tree')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show description of a task in the current tree')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit description of a task in the current tree')
            [CompletionResult]::new('priority', 'priority', [CompletionResultType]::ParameterValue, 'Set priority of a task in the current tree')
            break
        }
        'forest;help;task;list' {
            break
        }
        'forest;help;task;add' {
            break
        }
        'forest;help;task;remove' {
            break
        }
        'forest;help;task;rename' {
            break
        }
        'forest;help;task;show' {
            break
        }
        'forest;help;task;edit' {
            break
        }
        'forest;help;task;priority' {
            break
        }
        'forest;help;tree' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List trees in the current forest')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add a new tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a tree')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'Rename a tree')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show description of a tree')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit description of a tree')
            break
        }
        'forest;help;tree;list' {
            break
        }
        'forest;help;tree;add' {
            break
        }
        'forest;help;tree;remove' {
            break
        }
        'forest;help;tree;rename' {
            break
        }
        'forest;help;tree;show' {
            break
        }
        'forest;help;tree;edit' {
            break
        }
        'forest;help;note' {
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all notes')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Create a new note associated to the current tree')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove a note')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show content of a note')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit a note')
            break
        }
        'forest;help;note;list' {
            break
        }
        'forest;help;note;add' {
            break
        }
        'forest;help;note;remove' {
            break
        }
        'forest;help;note;show' {
            break
        }
        'forest;help;note;edit' {
            break
        }
        'forest;help;switch' {
            break
        }
        'forest;help;start' {
            break
        }
        'forest;help;stop' {
            break
        }
        'forest;help;status' {
            break
        }
        'forest;help;report' {
            break
        }
        'forest;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
