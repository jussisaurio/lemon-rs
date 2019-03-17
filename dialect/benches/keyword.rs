#![feature(test)]
extern crate test;

use sqlite_dialect::MAX_KEYWORD_LEN;
use sqlite_dialect::keyword_token;
use test::Bencher;

static VALUES: [&[u8]; 136] = [
    b"ABORT",
    b"ACTION",
    b"ADD",
    b"AFTER",
    b"ALL",
    b"ALTER",
    b"ANALYZE",
    b"AND",
    b"AS",
    b"ASC",
    b"ATTACH",
    b"AUTOINCREMENT",
    b"BEFORE",
    b"BEGIN",
    b"BETWEEN",
    b"BY",
    b"CASCADE",
    b"CASE",
    b"CAST",
    b"CHECK",
    b"COLLATE",
    b"COLUMN",
    b"COMMIT",
    b"CONFLICT",
    b"CONSTRAINT",
    b"CREATE",
    b"CROSS",
    b"CURRENT",
    b"CURRENT_DATE",
    b"CURRENT_TIME",
    b"CURRENT_TIMESTAMP",
    b"DATABASE",
    b"DEFAULT",
    b"DEFERRABLE",
    b"DEFERRED",
    b"DELETE",
    b"DESC",
    b"DETACH",
    b"DISTINCT",
    b"DO",
    b"DROP",
    b"EACH",
    b"ELSE",
    b"END",
    b"ESCAPE",
    b"EXCEPT",
    b"EXCLUSIVE",
    b"EXISTS",
    b"EXPLAIN",
    b"FAIL",
    b"FILTER",
    b"FOLLOWING",
    b"FOR",
    b"FOREIGN",
    b"FROM",
    b"FULL",
    b"GLOB",
    b"GROUP",
    b"HAVING",
    b"IF",
    b"IGNORE",
    b"IMMEDIATE",
    b"IN",
    b"INDEX",
    b"INDEXED",
    b"INITIALLY",
    b"INNER",
    b"INSERT",
    b"INSTEAD",
    b"INTERSECT",
    b"INTO",
    b"IS",
    b"ISNULL",
    b"JOIN",
    b"KEY",
    b"LEFT",
    b"LIKE",
    b"LIMIT",
    b"MATCH",
    b"NATURAL",
    b"NO",
    b"NOT",
    b"NOTHING",
    b"NOTNULL",
    b"NULL",
    b"OF",
    b"OFFSET",
    b"ON",
    b"OR",
    b"ORDER",
    b"OUTER",
    b"OVER",
    b"PARTITION",
    b"PLAN",
    b"PRAGMA",
    b"PRECEDING",
    b"PRIMARY",
    b"QUERY",
    b"RAISE",
    b"RANGE",
    b"RECURSIVE",
    b"REFERENCES",
    b"REGEXP",
    b"REINDEX",
    b"RELEASE",
    b"RENAME",
    b"REPLACE",
    b"RESTRICT",
    b"RIGHT",
    b"ROLLBACK",
    b"ROW",
    b"ROWS",
    b"SAVEPOINT",
    b"SELECT",
    b"SET",
    b"TABLE",
    b"TEMP",
    b"TEMPORARY",
    b"THEN",
    b"TO",
    b"TRANSACTION",
    b"TRIGGER",
    b"UNBOUNDED",
    b"UNION",
    b"UNIQUE",
    b"UPDATE",
    b"USING",
    b"VACUUM",
    b"VALUES",
    b"VIEW",
    b"VIRTUAL",
    b"WHEN",
    b"WHERE",
    b"WINDOW",
    b"WITH",
    b"WITHOUT",
];

#[bench]
fn bench_keyword_token(b: &mut Bencher) {
    b.iter(|| {
        for value in VALUES.iter() {
            assert!(keyword_token(value).is_some())
        }
    });
}

#[bench]
fn bench_upcase_keyword_token(b: &mut Bencher) {
    let mut uppercase_buffer: [u8; MAX_KEYWORD_LEN] = [0; MAX_KEYWORD_LEN];
    b.iter(|| {
        for value in VALUES.iter() {
            let upcase = &mut uppercase_buffer[..value.len()];
            upcase.copy_from_slice(value);
            upcase.make_ascii_uppercase();
            assert!(keyword_token(upcase).is_some())
        }
    });
}
