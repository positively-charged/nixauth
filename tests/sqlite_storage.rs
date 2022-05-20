use nixauth::sqlite_storage::SqliteStorage;

#[ test ]
fn add_user() {
   let storage = SqliteStorage::new_in_memory();
   storage.add_user( "test", "abcde".as_bytes(), "12345".as_bytes() );

}
