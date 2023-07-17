use diesel::query_dsl::methods::FilterDsl;

diesel::table! {
	tableA (rowid) {
		rowid -> Integer,
		columnA -> Integer,
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	use diesel::prelude::*;
	use crate::tableA::dsl::*;

	let mut conn =  SqliteConnection::establish("/tmp/test.db")?;

	let input = 10;
	let subquery = FilterDsl::filter(tableA.select(columnA), rowid.eq(input)).single_value();

	let res = FilterDsl::filter(tableA, columnA.nullable().eq(subquery))
		.count()
		.get_result(&mut conn)?;

	println!("res {:#?}", res);

	return Ok(());
}
