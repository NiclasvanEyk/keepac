Source ../config.tape

Hide
# Create new file
Type "changelog init"
Sleep 500ms
Enter
Sleep 1500ms
Show

# Trigger section selection
Type "changelog insert"
Sleep 500ms
Enter
Sleep 1500ms

# Select "Fixed"
Down
Sleep 500ms
Down
Sleep 500ms
Down
Sleep 500ms
Down
Sleep 1500ms
Enter

Type "That one annoying bug"
Enter

Sleep 1500ms

# Cleanup
Hide
Type rm CHANGELOG.md
Enter
