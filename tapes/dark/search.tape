Source ../partials/config.tape
Require wget

Set Width 1500
Set Height 800

Hide
# We use the tailwindcss changelog as an example
Type "wget https://github.com/tailwindlabs/tailwindcss/raw/master/CHANGELOG.md"
Sleep 250ms
Enter
Sleep 250ms
Type "clear"
Enter
Show

Type "# Imagine you are trying to search for changes related to a specific feature."
Sleep 500ms
Enter

Type "# Lets use TailwindCSS as an example. Let's try searching for changes related to flexbox"
Sleep 500ms
Enter

Type "cat CHANGELOG.md | grep flex"
Sleep 500ms
Enter

Type "# This works, but we can do better:"
Sleep 500ms
Enter

Type "changelog search flex"
Sleep 500ms
Enter
Sleep 2500ms

Type "# Much better! More context, nicer output and easier to remember."
Sleep 10000ms

Hide
Type "rm CHANGELOG.md"

