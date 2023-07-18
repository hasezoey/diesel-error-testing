use diesel::query_dsl::methods::FilterDsl;
use diesel::{prelude::*, delete};


diesel::table! {
	tableA (rowid) {
		rowid -> Integer,
		columnA -> Integer,
	}
}

diesel::table! {
	tableB (rowid) {
		rowid -> Integer,
		ref_tablea -> Integer,
	}
}

diesel::table! {
	tableC (rowid) {
		rowid -> Integer,
		ref_tablea -> Integer,
	}
}

diesel::joinable!(tableB -> tableA (ref_tablea));
diesel::joinable!(tableC -> tableA (ref_tablea));

diesel::allow_tables_to_appear_in_same_query!(
	tableA,
	tableB,
	tableC,
);


fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut conn =  SqliteConnection::establish("/tmp/test.db")?;

	let subquery = tableA::columns::columnA.eq_any(vec![0, 1, 2]);

	let res = delete_many(&mut conn, subquery);

	println!("res {:#?}", res);

	return Ok(());
}

pub fn delete_many<P>(db: &mut SqliteConnection, predicate: P) -> QueryResult<()>
where
	P: diesel::BoxableExpression<
			tableA::table,
			diesel::sqlite::Sqlite,
			SqlType = diesel::sql_types::Bool,
		> + diesel::expression::NonAggregate
		+ diesel::query_builder::QueryId
		// + Copy, // disabled, because a "eq_any" query would not be possible
		+ Clone
{
	return db.transaction(|conn| {
		let subquery = FilterDsl::filter(tableA::table, predicate.clone()).select(tableA::columns::rowid);

		// let subquery_copy = subquery.clone();

		delete(tableB::table).filter(tableB::columns::ref_tablea.eq_any(subquery.into_boxed())).execute(conn)?;

		{
			// preferably combined with the first "subquery" variable
			let subquery = FilterDsl::filter(tableA::table, predicate.clone()).select(tableA::columns::rowid);
			let tablec_filter = tableC::columns::ref_tablea.eq_any(subquery/* .into_boxed() */);
			// let tablec_filter = tableC::columns::ref_tablea.eq_any(vec![0,1,2,3]); // works without problems
			delete_many_c(conn, tablec_filter)?;
//          -------------       ^^^^^^^^^^^^^ the trait `AppearsOnTable<query_source::joins::Join<tableA::table, tableC::table, Inner>>` is not implemented for `P`
//          |
//          required by a bound introduced by this call

		}

		delete(tableA::table).filter(predicate).execute(conn)?;

		return Ok(());
	});
}

pub fn delete_many_c<P>(db: &mut SqliteConnection, predicate: P) -> QueryResult<()>
where
	P: diesel::BoxableExpression<
			tableC::table,
			diesel::sqlite::Sqlite,
			SqlType = diesel::sql_types::Bool,
		> + diesel::expression::NonAggregate
		+ diesel::query_builder::QueryId
		// + Copy, // disabled, because a "eq_any" query would not be possible
		+ Clone
{
	return db.transaction(|conn| {
		delete(tableC::table).filter(predicate).execute(conn)?;

		// image similar additional deletes and clones of "predicate"

		return Ok(());
	});
}
