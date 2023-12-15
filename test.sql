.load target/release/libsqlite_usaddress.dylib sqlite3_usaddress_init

.mode box
.header on
.timer on


select usaddress_parse_json('123 White Road');

select usaddress_parse_json('1600 Pennsylvania Avenue NW, Washington, DC 20500');
