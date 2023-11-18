# Minor
## FIXED 
-changing pos doesn't change the priority.

## OPEN
- If task position is changed, the searchable state doesn't update
- Deletion doesn't touch the searchable state as well.

# Major
## FIXED 
-text search is completly broken.
	-> ONLY for newly read files without restart.
-text search is completly broken. AGAIN.
	-> Now some of the correct items are displayed; there are wrong and missing ones though.
	-> Problem again only materializes when at least one new task has been created and usr then tries to sort.
		-> I am not updating the searchable appstate when creating a new task!

## OPEN

