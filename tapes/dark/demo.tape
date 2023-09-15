Source ../partials/config.tape

Hide
Type "mkdir keepac-demo"
Enter
Type "cd keepac-demo"
Enter
Type "clear"
Enter
Show

Type "# Imagine you start a new project and want to keep a proper changelog."
Enter
Sleep 2500ms

Type "# keepac's `changelog` CLI makes this simple:"
Enter
Sleep 2500ms

Type "changelog init"
Enter
Sleep 4000ms

Type "clear"
Enter
Sleep 1500ms

Type "# That looked kinda empty. Let's change that!"
Enter
Sleep 2500ms

Type "changelog add A beeeeauuuutiful, shiny new feature"
Enter
Sleep 1500ms

Type "# Good job on that feature btw!"
Enter
Sleep 2500ms

Type "# We can view our changelog anytime:"
Enter
Sleep 1500ms

Type "changelog show"
Enter
Sleep 2500ms

Type "# Let's add a few more changes!"
Enter
Sleep 1500ms

Type "changelog fix A gnarly, disastrous bug"
Enter
Sleep 1500ms

Type "changelog change Something that breaks backwards compatibility"
Enter
Sleep 2500ms

Type "clear"
Enter
Sleep 1500ms

Type "# I feel like we have enough changes to finally launch our project. Let's release v1!"
Enter
Sleep 2500ms

Type "changelog release --major"
Enter
Sleep 5000ms

Type "# Congrats! Now keep documenting your features."
Enter
Type "# Keepac has more functionality, explore it by reading this Readme or run"
Enter
Sleep 1500ms
Type "changelog --help"
Enter

Sleep 10000ms

Hide
Type "cd ../"
Enter
Type "rm -rf keepac-demo"
Enter
