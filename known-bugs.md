# Minor
## FIXED 
- changing pos doesn't change the priority.
- If task position is changed, the searchable state doesn't update
- Deletion doesn't touch the searchable state as well.

## OPEN
- boolean search should reset if another bool search was on
- New file location doesn't work if a completly empty file was dropped in OR .anankeconfig exists but is empty. -> Does work if new task was made.


# Major
## FIXED 
- text search is completly broken.
	-> ONLY for newly read files without restart.
- text search is completly broken. AGAIN.
	-> Now some of the correct items are displayed; there are wrong and missing ones though.
	-> Problem again only materializes when at least one new task has been created and usr then tries to sort.
		-> I am not updating the searchable appstate when creating a new task!

## OPEN
- text search is broken again. This time ananke crashes when a string is search that does not exist.
