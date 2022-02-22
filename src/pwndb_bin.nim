import std/db_sqlite

# user, password, database name can be empty.
# These params are not used on db_sqlite module.
let db = open("mytest.db", "", "", "")
db.close()
