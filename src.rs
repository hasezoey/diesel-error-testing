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
	let tablea_alias = diesel::alias!(crate::tableA as tablea_alias);
	let subquery = FilterDsl::filter(tablea_alias.select(tablea_alias.field(columnA)), tablea_alias.field(rowid).eq(input)).single_value();

	let res = FilterDsl::filter(tableA, columnA.nullable().eq(subquery))
		.count()
		.get_result::<i64>(&mut conn)?;

	println!("res {:#?}", res);

	return Ok(());
}
