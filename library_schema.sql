/* This is the main table holding the important book data
 * id: A generated ID for the book
 * title: The title of the book
 * subtitle: A potential subtitle, potentially used to differentiate books in a series
 * isbn: The, supposed to be, unique identifier of the book
 */
CREATE TABLE book (
	id TEXT PRIMARY KEY, 
	title TEXT NOT NULL,
	subtitle TEXT,
	isbn TEXT UNIQUE NOT NULL
);

/* This table is a child to the `book` table storing reference to the potentially many authors to a book
 * book_id: The ID of the book being tracked, referencing the id in the `book` table
 * author: One of the potentially many authors for the book refered by the book_id
 */
CREATE TABLE authors (
	book_id TEXT NOT NULL,
	author TEXT NOT NULL,
	FOREIGN KEY(book_id) REFERENCES book(id)
) WITHOUT ROWID;

/* This is a child table to the `book` table, meant to track the books status
 * book_id: the ID of the Book being tracked, referencing the id in the `book` table
 * status: the current status of the book
 */
CREATE TABLE book_status (
	book_id TEXT UNIQUE NOT NULL,
	status TEXT CHECK('checked_in', 'on_loan', 'being_read') NOT NULL DEFAULT 'checked_in',
	FOREIGN KEY(book_id) REFERENCES book(id);
) WITHOUT ROWID;

/* Table stores and tracks currently connected and valid clients 
 * client_id: Generated UUID for a client
 * verification: Generated code to verify clients status
 * expiery: Date that the verification will expire supposed to be 24+(first contact)
 * user_agent: This is the User-agent of the client to verify that the client is from the same device.
 * 	This will mitigate some malicious/curious actors tyring to atack the resources of the server, 
 *	and also assist in tracking user activity
 */
CREATE TABLE verified_clients (
	client_id TEXT PRIMARY KEY,
	verification TEXT NOT NULL,
	expiery TEXT NOT NULL,
	user_agent TEXT
);
