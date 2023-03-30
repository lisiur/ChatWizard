// use diesel::{
//     expression::NonAggregate,
//     prelude::*,
//     query_builder::QueryFragment,
//     query_dsl::methods::{BoxedDsl, OrderDsl},
//     sqlite::Sqlite,
// };

// fn sort_by_column<'a, U, T>(
//     mut query: diesel::dsl::IntoBoxed<'static, T, Sqlite>,
//     column: U,
//     sort_dir: Option<String>,
// ) -> diesel::dsl::IntoBoxed<'static, T, Sqlite>
// where
//     T: BoxedDsl<'static, Sqlite>,
//     diesel::dsl::IntoBoxed<'static, T, Sqlite>: OrderDsl<
//             Box<dyn BoxableExpression<T, Sqlite, SqlType = ()>>,
//             Output = diesel::dsl::IntoBoxed<'static, T, Sqlite>,
//         > + QueryDsl,
//     U: ExpressionMethods
//         + QueryFragment<Sqlite>
//         + AppearsOnTable<T>
//         + SelectableExpression<T>
//         + NonAggregate
//         + 'static,
// {
//     match sort_dir.as_ref().map(String::as_str) {
//         Some("ASC") => query.order_by(Box::new(column.asc())),
//         Some("DESC") => query.order_by(Box::new(column.desc())),
//         _ => query,
//     }
// }
