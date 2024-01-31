#[async_trait]
impl AccountPointRepository for AccountPointRepositoryImpl {
    async fn gain_gold(&self, account: Account, account_user_id: &str, gain_golds: i32) -> Result<usize, diesel::result::Error> {
        println!("AccountPointRepositoryImpl: gain_gold()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause = FilterDsl::filter(accounts, columns::user_id.eq(user_id));
        let found_accounts = where_clause
            .select((columns::id, columns::user_id, columns::password, columns::gold))
            .load::<Account>(&mut connection)?;

        let found_account = found_accounts
            .into_iter()
            .find(|account| account.user_id == account_user_id);

        let current_gold = found_account.unwrap().gold;
        let rusult_gold = current_gold + gain_golds;

        match diesel::update(FilterDsl::filter(accounts, columns::user_id.eq(account.user_id)))
            .set((
                columns::gold.eq(rusult_gold),
            ))
            .execute(&mut connection)
        {
            Ok(num) => {
                println!("Gold of account updated successfully.");
                Ok(num)
            }
            Err(e) => {
                eprintln!("Error updating gold of account: {:?}", e);
                Err(e)
            }
        }
    }

    async fn pay_gold(&self, account: Account, account_user_id: &str, pay_golds: i32) -> Result<usize, diesel::result::Error> {
        println!("AccountPointRepositoryImpl: pay_gold()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause = FilterDsl::filter(accounts, columns::user_id.eq(user_id));
        let found_accounts = where_clause
            .select((columns::id, columns::user_id, columns::password, columns::gold))
            .load::<Account>(&mut connection)?;

        let found_account = found_accounts
            .into_iter()
            .find(|account| account.user_id == account_user_id);

        let current_gold = found_account.unwrap().gold;

        if current_gold >= pay_golds {
            let result_gold = current_gold - pay_golds;

            match diesel::update(FilterDsl::filter(accounts, columns::user_id.eq(account.user_id)))
                .set((
                    columns::gold.eq(result_gold),
                ))
                .execute(&mut connection)
            {
                Ok(num) => {
                    println!("Gold of account updated successfully.");
                    Ok(num)
                }
                Err(e) => {
                    eprintln!("Error updating gold of account: {:?}", e);
                    Err(e)
                }
            }
        } else {
            println!("You don't have not enough gold");
            Err(diesel::result::Error::NotFound)
        }
    }
}