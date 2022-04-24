use sea_query::Iden;

#[derive(Iden, Clone)]
pub enum Customers {
    Table,
    Id,
    Name,
    Email,
    Phone,
    CreatedAt,
    UpdatedAt,
}
