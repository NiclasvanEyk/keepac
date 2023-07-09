Source ../partials/config.tape
Require tree

Hide
Type "mkdir init-demo && cd init-demo"
Sleep 100ms
Enter
Type "changelog init"
Sleep 100ms
Enter
Type "clear"
Sleep 100ms
Enter
Show

Type "ls"
Sleep 100ms
Enter
Sleep 2500ms

Type "changelog find"
Sleep 100ms
Enter
Sleep 2500ms

Type "mkdir -p some/deeply/nested/directory"
Sleep 100ms
Enter
Sleep 2500ms

Type "cd some/deeply/nested/directory"
Sleep 100ms
Enter
Sleep 2500ms

Type "changelog find"
Sleep 100ms
Enter
Sleep 2500ms

Type "# Still finds the 'nearest' CHANGELOG.md!"
Sleep 5000ms
Enter

Hide
Type "cd ../../../.."
Sleep 100ms
Enter
Type "rm -rf init-demo"
Sleep 100ms
Enter
