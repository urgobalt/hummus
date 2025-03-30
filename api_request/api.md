# API explination
This is an explination of all routes for the api and is the api spec.
h2 are used for describing the service and h3 for the general topic, additionaly there is a comment after each url

## Store API
The api required for the store specification
### Notes API
- /api/note/<id> GET # get note by id
- /api/note PUT # put
- /api/note DELETE # Delete note
- /api/note PATCH # Update
- /api/notes GET # gets all notes ids
- /api/notes/search POST # search notes
- /api/authenticate POST # validates the session and adds it as trusted for the duration given by the IDS

### Sharing
Interactive notes are stored as a external url to the note
static means that the note is copied in the current store
- /api/share/interactive POST # gives sharing perms to a certain user given id
- /api/share/interactive/note POST # shares access to a note
- /api/share/static POST # shares access to a static copy by looking up the url of the store for the user to be shared Store
- /api/share/static/note POST # shares access to a static copy by transfering the copy
- /api/shared GET # gets all the notes that has been shared
- /api/shared/accept POST # accepts a share is sent to both servers if they are interavtive
- /api/shared/deny POST # decline one or many shared notes

## IDServer(IDS)
- /api/auth/login POST # login that returns session with id
- /api/auth/create GET # create account that retuns redirect url to create account given that it might require payment info
- /api/auth/validate POST # validates session id is correct for user
- /api/auth/delete POST # delete account
- /api/users POST # searches for users given atleast 4 chars of name

