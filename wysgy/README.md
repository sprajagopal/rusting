# Notes for the power user
## Higher dimensions to simple notes
Wysgy aims to increase the clarity of note taking. It does this by allowing:

* relationships between notes
    * Related notes need not link to each other. Instead `wysgy rel note1 note2` and this creates a link file (add your notes about the relationship here
		```mermaid
		graph LR
		notesA -- here you can describe the relationship --> notesB

* arbitrary existing files to be attached to notes
	* Want to add a note about a syslog file or a test output? `wysgy file <your_existing_file>` and this will open a blank file in your `$EDITOR`. You can add description about the file here.
		* Ex: `wysgy file <your_existing_file>`
		* In the editor, 

			    test_on: 6th Jan
			    success: yes
			    email: pending

* add text notes as key-value pairs
	* Jotting down websites/numbers? `wysgy json <note_name> <your_notes_go_here>`
		* Ex: `wysgy json ccare_online "number: 94384859, expected_call: 2 days from now"


## Viewing notes
Wysgy notes are fundamentally key-value pairs (currently the value can only be a String). 
To view notes:
    
    wysgy show <note-name>

To view related notes

    wysgy cnts <note-name>


## Portability
Wysgy notes, relationships are simply files. A wysgy project is a collection of your notes and relationship files describing the interconnects. It also contains the files you may have added with `wysgy file`. It becomes a self-contained, portable folder that can be backed up with git, dropbox, google drive, whatever you prefer.
