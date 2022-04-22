use sea_query::Iden;

#[derive(Iden)]
pub enum Customers {
    Table,
    Id,
    Name,
    Email,
    Phone,
    CreatedAt,
    UpdatedAt,
}
