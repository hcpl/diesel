#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::connection::PgConnection;

table! {
    users {
        id -> Serial,
        name -> VarChar,
    }
}

fn main() {
    use self::users::dsl::*;

    let connection = PgConnection::establish("").unwrap();
    let select_id = users.select(id);
    let select_name = users.select(name);

    let ids: QueryResult<Vec<i32>> = select_name.load(&connection).map(Iterator::collect);
    //~^ ERROR the trait `diesel::query_source::Queryable<diesel::types::VarChar, diesel::backend::Pg>` is not implemented for the type `i32`
    let names: QueryResult<Vec<String>> = select_id.load(&connection).map(Iterator::collect);
    //~^ ERROR the trait `diesel::query_source::Queryable<diesel::types::Integer, diesel::backend::Pg>` is not implemented
}
