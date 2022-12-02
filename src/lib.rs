use std::collections::{HashMap, HashSet};
use std::error::Error;

use chrono::TimeZone;
use log::*;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug)]
pub struct FinancistoCurrency {
    pub id: i32,
    pub decimal_separator: Option<String>,
    pub updated_on: i64,
    pub title: String,
    pub entity: String,
    pub decimals: i32,
    pub symbol: String,
    pub is_active: i32,
    pub name: String,
    pub is_default: i32,
    pub symbol_format: String,
    pub group_separator: Option<String>,
}

impl From<&HashMap<String, String>> for FinancistoCurrency {
    fn from(map: &HashMap<String, String>) -> Self {
        Self {
            id: map.get("_id").unwrap().parse().unwrap(),
            decimal_separator: map.get("decimal_separator").map(|v| v.to_owned()),
            updated_on: map.get("updated_on").unwrap().parse().unwrap(),
            title: map.get("title").unwrap().to_owned(),
            entity: map.get("entity").unwrap().to_owned(),
            decimals: map.get("decimals").unwrap().parse().unwrap(),
            symbol: map.get("symbol").unwrap().to_owned(),
            is_active: map.get("is_active").unwrap().parse().unwrap(),
            name: map.get("name").unwrap().to_owned(),
            is_default: map.get("is_default").unwrap().parse().unwrap(),
            symbol_format: map.get("symbol_format").unwrap().to_owned(),
            group_separator: map.get("group_separator").map(|v| v.to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct FinancistoAccount {
    pub id: i32,
    pub note: Option<String>,
    pub total_limit: i32,
    pub entity: String,
    pub total_amount: i32,
    pub last_transaction_date: i64,
    pub payment_day: i32,
    pub sort_order: i32,
    pub r#type: String,
    pub updated_on: i64,
    pub is_active: i32,
    pub issuer: Option<String>,
    pub title: String,
    pub creation_date: i64,
    pub last_account_id: i32,
    pub last_category_id: i32,
    pub currency_id: i32,
    pub closing_day: i64,
    pub card_issuer: Option<String>,
    pub is_include_into_totals: i32,
}

impl From<&HashMap<String, String>> for FinancistoAccount {
    fn from(map: &HashMap<String, String>) -> Self {
        Self {
            id: map.get("_id").unwrap().parse().unwrap(),
            note: map.get("note").map(|v| v.to_owned()),
            total_limit: map.get("total_limit").unwrap().parse().unwrap(),
            entity: map.get("entity").unwrap().to_owned(),
            total_amount: map.get("total_amount").unwrap().parse().unwrap(),
            last_transaction_date: map.get("last_transaction_date").unwrap().parse().unwrap(),
            payment_day: map.get("payment_day").unwrap().parse().unwrap(),
            sort_order: map.get("sort_order").unwrap().parse().unwrap(),
            r#type: map.get("type").unwrap().to_owned(),
            updated_on: map.get("updated_on").unwrap().parse().unwrap(),
            is_active: map.get("is_active").unwrap().parse().unwrap(),
            issuer: map.get("issuer").map(|v| v.to_owned()),
            title: map.get("title").unwrap().to_owned(),
            creation_date: map.get("creation_date").unwrap().parse().unwrap(),
            last_account_id: map.get("last_account_id").unwrap().parse().unwrap(),
            last_category_id: map.get("last_category_id").unwrap().parse().unwrap(),
            currency_id: map.get("currency_id").unwrap().parse().unwrap(),
            closing_day: map.get("closing_day").unwrap().parse().unwrap(),
            card_issuer: map.get("card_issuer").map(|v| v.to_owned()),
            is_include_into_totals: map.get("is_include_into_totals").unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct FinancistoCategory {
    pub r#type: i32,
    pub id: i32,
    pub updated_on: i64,
    pub right: i32,
    pub left: i32,
    pub last_location_id: i32,
    pub is_active: i32,
    pub title: String,
    pub last_project_id: i32,
    pub entity: String,
}

impl From<&HashMap<String, String>> for FinancistoCategory {
    fn from(map: &HashMap<String, String>) -> Self {
        Self {
            r#type: map.get("type").unwrap().parse().unwrap(),
            id: map.get("_id").unwrap().parse().unwrap(),
            updated_on: map.get("updated_on").unwrap().parse().unwrap(),
            right: map.get("right").unwrap().parse().unwrap(),
            left: map.get("left").unwrap().parse().unwrap(),
            last_location_id: map.get("last_location_id").unwrap().parse().unwrap(),
            is_active: map.get("is_active").unwrap().parse().unwrap(),
            title: map.get("title").unwrap().to_owned(),
            last_project_id: map.get("last_project_id").unwrap().parse().unwrap(),
            entity: map.get("entity").unwrap().to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct FinancistoTransaction {
    pub longitude: f32,
    pub from_account_id: i32,
    pub location_id: i32,
    pub note: Option<String>,
    pub to_account_id: i32,
    pub entity: String,
    pub datetime: i64,
    pub project_id: i32,
    pub provider: Option<String>,
    pub template_name: Option<String>,
    pub latitude: f32,
    pub accuracy: f32,
    pub status: Option<String>,
    pub is_ccard_payment: i32,
    pub payee_id: i32,
    pub category_id: i32,
    pub parent_id: i32,
    pub to_amount: i64,
    pub from_amount: i64,
    pub original_currency_id: i32,
    pub original_from_amount: i32,
    pub last_recurrence: i64,
    pub is_template: i32,
    pub updated_on: i64,
    pub id: i32,
}

impl From<&HashMap<String, String>> for FinancistoTransaction {
    fn from(map: &HashMap<String, String>) -> Self {
        Self {
            longitude: map.get("longitude").unwrap().parse().unwrap(),
            from_account_id: map.get("from_account_id").unwrap().parse().unwrap(),
            location_id: map.get("location_id").unwrap().parse().unwrap(),
            note: map.get("note").map(|v| v.to_owned()),
            to_account_id: map.get("to_account_id").unwrap().parse().unwrap(),
            entity: map.get("entity").unwrap().to_owned(),
            datetime: map.get("datetime").unwrap().parse().unwrap(),
            project_id: map.get("project_id").unwrap().parse().unwrap(),
            provider: map.get("provider").map(|v| v.to_owned()),
            template_name: map.get("template_name").map(|v| v.to_owned()),
            latitude: map.get("latitude").unwrap().parse().unwrap(),
            accuracy: map.get("accuracy").unwrap().parse().unwrap(),
            status: map.get("v").map(|v| v.to_owned()),
            is_ccard_payment: map.get("is_ccard_payment").unwrap().parse().unwrap(),
            payee_id: map.get("payee_id").unwrap().parse().unwrap(),
            category_id: map.get("category_id").unwrap().parse().unwrap(),
            parent_id: map.get("parent_id").unwrap().parse().unwrap(),
            to_amount: map.get("to_amount").unwrap().parse().unwrap(),
            from_amount: map.get("from_amount").unwrap().parse().unwrap(),
            original_currency_id: map.get("original_currency_id").unwrap().parse().unwrap(),
            original_from_amount: map.get("original_from_amount").unwrap().parse().unwrap(),
            last_recurrence: map.get("last_recurrence").unwrap().parse().unwrap(),
            is_template: map.get("is_template").unwrap().parse().unwrap(),
            updated_on: map.get("updated_on").unwrap().parse().unwrap(),
            id: map.get("_id").unwrap().parse().unwrap(),
        }
    }
}

pub struct Financisto {
    pub currencies: Vec<FinancistoCurrency>,
    pub accounts: Vec<FinancistoAccount>,
    pub categories: Vec<FinancistoCategory>,
    pub transactions: Vec<FinancistoTransaction>,
}

impl Financisto {
    pub fn new() -> Financisto {
        Self {
            currencies: Vec::new(),
            accounts: Vec::new(),
            categories: Vec::new(),
            transactions: Vec::new(),
        }
    }
}

pub fn convert_maps(lines: &Vec<String>) -> Result<Financisto> {
    let mut data = Financisto::new();
    let mut index = 4;

    while index < lines.len() {
        let line = lines.get(index).unwrap();

        if "#START" == line || "#END" == line {
            index = index + 1;
        } else {
            if line.starts_with("$ENTITY:") {
                let map = convert_entity(lines, &mut index);

                if let Some(entity) = map.get("entity") {
                    match entity.as_ref() {
                        "currency" => {
                            trace!("{} = {:?}", entity, &map);
                            let item: FinancistoCurrency = (&map).into();
                            debug!("{:?}", &item);
                            data.currencies.push(item);
                        }
                        "account" => {
                            trace!("{} = {:?}", entity, &map);
                            let item: FinancistoAccount = (&map).into();
                            debug!("{:?}", &item);
                            data.accounts.push(item);
                        }
                        "category" => {
                            trace!("{} = {:?}", entity, &map);
                            let item: FinancistoCategory = (&map).into();
                            debug!("{:?}", &item);
                            data.categories.push(item);
                        }
                        "transactions" => {
                            trace!("{} = {:?}", entity, &map);
                            let item: FinancistoTransaction = (&map).into();
                            debug!("{:?}", &item);
                            data.transactions.push(item);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(data)
}

fn convert_entity(lines: &Vec<String>, index: &mut usize) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    loop {
        let line = lines.get(*index).unwrap();

        *index = *index + 1;

        if "$$" == line {
            break;
        } else {
            if let Some(split_position) = line.find(':') {
                let name = line[..split_position].to_owned();
                let value = line[split_position + 1..].to_owned();
                match name.as_str() {
                    "$ENTITY" => map.insert("entity".to_owned(), value),
                    _ => map.insert(name, value),
                };
            }
        }
    }

    map
}

pub fn print_entity(list: &Vec<HashMap<String, String>>) {
    let entities = list
        .iter()
        .map(|m| m.get("entity").unwrap())
        .fold(&mut HashSet::new(), |set, x| {
            set.insert(x);
            set
        })
        .clone();

    for entity in entities.iter() {
        info!("entity = {}", entity);
    }
}

fn escape_quote(text: &str) -> String {
    text.replace("'", "''")
}

pub fn migrate_accounts(
    accounts: &Vec<FinancistoAccount>,
    currencies: &Vec<FinancistoCurrency>,
) -> Result<Vec<String>> {
    let mut item_id = 5;
    let mut statements = Vec::new();

    for account in accounts.iter() {
        debug!("{}: {:?}\n", account.entity, account);

        // map Financisto account type to bluecoins account type (ACCOUNTTYPETABLE)
        let account_type = match account.r#type.as_str() {
            "ASSET" => "15",
            "BANK" => "3",
            "CREDIT_CARD" => "8",
            "DEBIT_CARD" => "8",
            "CASH" => "4",
            "ELECTRONIC" => "15",
            "OTHER" => "15",
            _ => "",
        };

        let currency = currencies
            .iter()
            .find(|c| c.id == account.currency_id)
            .unwrap()
            .name
            .to_string();
        let create_time = chrono::Local
            .timestamp_millis_opt(account.creation_date)
            .unwrap();

        // account
        statements.push(format!(
            "INSERT INTO \"ACCOUNTSTABLE\" (\"accountsTableID\", \"accountName\", \"accountTypeID\", \"accountHidden\", \"accountCurrency\", \"accountConversionRateNew\", \"currencyChanged\", \"creditLimit\", \"cutOffDa\", \"creditCardDueDate\", \"cashBasedAccounts\", \"accountSelectorVisibility\", \"accountsExtraColumnInt1\", \"accountsExtraColumnInt2\", \"accountsExtraColumnString1\", \"accountsExtraColumnString2\") VALUES ('{}', '{}', '{}', '0', '{}', '1.0', NULL, '0', '0', '0', '0', '0', NULL, NULL, NULL, NULL);",
            account.id,
            escape_quote(&account.title),
            account_type,
            currency,
        ));

        // item
        statements.push(format!(
            "INSERT INTO \"ITEMTABLE\" (\"itemTableID\", \"itemName\", \"itemAutoFillVisibility\") VALUES ('{}', '{}', '0');",
            item_id,
            escape_quote(&account.title),
        ));

        // init transaction
        statements.push(format!(
            "INSERT INTO \"TRANSACTIONSTABLE\" (\"transactionsTableID\", \"itemID\", \"amount\", \"transactionCurrency\", \"conversionRateNew\", \"date\", \"transactionTypeID\", \"categoryID\", \"accountID\", \"notes\", \"status\", \"accountReference\", \"accountPairID\", \"uidPairID\", \"deletedTransaction\", \"newSplitTransactionID\", \"transferGroupID\", \"reminderTransaction\", \"reminderGroupID\", \"reminderFrequency\", \"reminderRepeatEvery\", \"reminderEndingType\", \"reminderStartDate\", \"reminderEndDate\", \"reminderAfterNoOfOccurences\", \"reminderAutomaticLogTransaction\", \"reminderRepeatByDayOfMonth\", \"reminderExcludeWeekend\", \"reminderWeekDayMoveSetting\", \"reminderUnbilled\", \"creditCardInstallment\", \"reminderVersion\", \"dataExtraColumnString1\") VALUES ('{transactionsTableID}', '{itemID}', '{amount}', '{transactionCurrency}', '{conversionRateNew}', '{date}', '{transactionTypeID}', '{categoryID}', '{accountID}', '{notes}', '{status}', '{accountReference}', '{accountPairID}', '{uidPairID}', '{deletedTransaction}', '{newSplitTransactionID}', '{transferGroupID}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);",
            transactionsTableID = account.id,
            itemID = item_id,
            amount = 0,
            transactionCurrency = currency,
            conversionRateNew = 1.0,
            date = create_time.format("%Y-%m-%d %H:%M:%S"),
            transactionTypeID  = 2,
            categoryID = 2,
            accountID = account.id,
            notes = "",
            status = 2,
            accountReference = 3,
            accountPairID = account.id,
            uidPairID = account.creation_date,
            deletedTransaction = 6,
            newSplitTransactionID = 0,
            transferGroupID = 0,
        ));

        item_id = item_id + 1;
    }

    Ok(statements)
}

pub fn migrate_categories(categories: &Vec<FinancistoCategory>) -> Result<Vec<String>> {
    let mut statements = Vec::new();

    // category parent
    for item in categories.iter().filter(|m| {
        m.right - m.left > 1
            || !categories
                .iter()
                .any(|n| n.left < m.left && n.right > m.right)
    }) {
        debug!("{}: {:?}\n", item.entity, item);

        let category_type = match item.title.as_str() {
            "收入" => "2",
            _ => "3",
        };

        statements.push(format!(
            "INSERT INTO \"PARENTCATEGORYTABLE\" (\"parentCategoryTableID\", \"parentCategoryName\", \"categoryGroupID\", \"budgetAmountCategoryParent\", \"budgetCustomSetupParent\", \"budgetPeriodCategoryParent\", \"budgetEnabledCategoryParent\", \"categoryParentExtraColumnInt1\", \"categoryParentExtraColumnInt2\", \"categoryParentExtraColumnString1\", \"categoryParentExtraColumnString2\") VALUES ('{parentCategoryTableID}', '{parentCategoryName}', '{categoryGroupID}', NULL, NULL, NULL, '1', NULL, NULL, NULL, NULL);",
            parentCategoryTableID = item.id,
            parentCategoryName = item.title,
            categoryGroupID = category_type,
        ));

        statements.push(format!(
            "INSERT INTO \"CHILDCATEGORYTABLE\" (\"categoryTableID\", \"childCategoryName\", \"parentCategoryID\", \"budgetAmount\", \"budgetCustomSetup\", \"budgetPeriod\", \"budgetEnabledCategoryChild\", \"childCategoryIcon\", \"categorySelectorVisibility\", \"categoryExtraColumnInt1\", \"categoryExtraColumnInt2\", \"categoryExtraColumnString1\", \"categoryExtraColumnString2\") VALUES ('{categoryTableID}', '{childCategoryName}', '{parentCategoryID}', '0', NULL, '3', '1', NULL, '0', NULL, NULL, NULL, NULL);",
            categoryTableID = item.id,
            childCategoryName = item.title,
            parentCategoryID = item.id,
        ));
    }

    // category child
    for item in categories.iter().filter(|m| {
        m.right - m.left == 1
            && categories
                .iter()
                .any(|n| n.left < m.left && n.right > m.right)
    }) {
        debug!("{}: {:?}\n", item.entity, item);

        // find parent
        let parent = categories
            .iter()
            .filter(|m| {
                m.right - m.left > 1
                    || !categories
                        .iter()
                        .any(|n| n.left < m.left && n.right > m.right)
            })
            .filter(|m| m.left < item.left && m.right > item.right)
            .reduce(|a, _| a)
            .unwrap();

        debug!("parent = {:?}", parent);

        statements.push(format!(
            "INSERT INTO \"CHILDCATEGORYTABLE\" (\"categoryTableID\", \"childCategoryName\", \"parentCategoryID\", \"budgetAmount\", \"budgetCustomSetup\", \"budgetPeriod\", \"budgetEnabledCategoryChild\", \"childCategoryIcon\", \"categorySelectorVisibility\", \"categoryExtraColumnInt1\", \"categoryExtraColumnInt2\", \"categoryExtraColumnString1\", \"categoryExtraColumnString2\") VALUES ('{categoryTableID}', '{childCategoryName}', '{parentCategoryID}', '0', NULL, '3', '1', NULL, '0', NULL, NULL, NULL, NULL);",
            categoryTableID = item.id,
            childCategoryName = item.title,
            parentCategoryID = parent.id,
        ));
    }

    Ok(statements)
}

pub fn migrate_transactions(
    transactions: &Vec<FinancistoTransaction>,
    currencies: &Vec<FinancistoCurrency>,
) -> Result<Vec<String>> {
    let mut statements = Vec::new();
    let mut last_item_id: i64 = 40;
    let mut items: HashMap<String, i64> = HashMap::new();
    let mut seq = 1;
    let mut id_set: HashSet<i64> = HashSet::new();

    for tx in transactions {
        debug!("{:?}", tx);

        let currency =
            if let Some(currency) = currencies.iter().find(|c| c.id == tx.original_currency_id) {
                &currency.name
            } else {
                "TWD"
            };
        let mut notes = "";
        let mut tx_time_in_milli: i64 = tx.datetime;
        let tx_time = chrono::Local
            .timestamp_millis_opt(tx_time_in_milli)
            .unwrap();

        if tx.to_account_id == 0 {
            // income or payment
            let item_id: i64 = if let Some(value) = &tx.note {
                if items.contains_key(value) {
                    *items.get(value).unwrap() as i64
                } else if tx.from_account_id == 33 && value.parse::<i32>().is_ok() {
                    // lottery ticket number
                    notes = value;

                    let item_name = "運動彩券";
                    if items.contains_key(item_name) {
                        *items.get(item_name).unwrap() as i64
                    } else {
                        last_item_id = last_item_id + 1;
                        items.insert(item_name.to_owned(), last_item_id);

                        statements.push(format!(
                            "INSERT INTO \"ITEMTABLE\" (\"itemTableID\", \"itemName\", \"itemAutoFillVisibility\") VALUES ('{}', '{}', '0');",
                            last_item_id,
                            escape_quote(item_name),
                        ));

                        last_item_id
                    }
                } else {
                    last_item_id = last_item_id + 1;
                    items.insert(value.to_owned(), last_item_id);

                    statements.push(format!(
                        "INSERT INTO \"ITEMTABLE\" (\"itemTableID\", \"itemName\", \"itemAutoFillVisibility\") VALUES ('{}', '{}', '0');",
                        last_item_id,
                        escape_quote(value),
                    ));

                    last_item_id
                }
            } else {
                if tx.from_amount >= 0 {
                    3 // Unnamed Income
                } else {
                    2 // Unnamed Expense
                }
            };

            if tx.parent_id == 0 {
                // single or split head
                id_set.insert(tx_time_in_milli);

                let mut split_children = transactions
                    .iter()
                    .filter(|t| t.parent_id == tx.id)
                    .collect::<Vec<_>>();

                if split_children.is_empty() {
                    // single

                    if id_set.contains(&tx_time_in_milli) {
                        // warn!("conflict id: {:?}", tx);
                        tx_time_in_milli = tx_time_in_milli + seq;
                        seq = seq + 1;
                    }

                    statements.push(format!(
                        "INSERT INTO \"TRANSACTIONSTABLE\" (\"transactionsTableID\", \"itemID\", \"amount\", \"transactionCurrency\", \"conversionRateNew\", \"date\", \"transactionTypeID\", \"categoryID\", \"accountID\", \"notes\", \"status\", \"accountReference\", \"accountPairID\", \"uidPairID\", \"deletedTransaction\", \"newSplitTransactionID\", \"transferGroupID\", \"reminderTransaction\", \"reminderGroupID\", \"reminderFrequency\", \"reminderRepeatEvery\", \"reminderEndingType\", \"reminderStartDate\", \"reminderEndDate\", \"reminderAfterNoOfOccurences\", \"reminderAutomaticLogTransaction\", \"reminderRepeatByDayOfMonth\", \"reminderExcludeWeekend\", \"reminderWeekDayMoveSetting\", \"reminderUnbilled\", \"creditCardInstallment\", \"reminderVersion\", \"dataExtraColumnString1\") VALUES ('{transactionsTableID}', '{itemID}', '{amount}', '{transactionCurrency}', '{conversionRateNew}', '{date}', '{transactionTypeID}', '{categoryID}', '{accountID}', '{notes}', '{status}', '{accountReference}', '{accountPairID}', '{uidPairID}', '{deletedTransaction}', '{newSplitTransactionID}', '{transferGroupID}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);",
                        transactionsTableID = tx_time_in_milli,
                        itemID = item_id,
                        amount = tx.from_amount * 1000000 / 100, // 2 digit fromn financisto, 6 digit from bluecoins
                        transactionCurrency = currency,
                        conversionRateNew = 1.0,
                        date = tx_time.format("%Y-%m-%d %H:%M:%S"),
                        transactionTypeID = if tx.from_amount >= 0 { 4 } else { 3 }, // 3 = expense, 4 = income
                        categoryID = tx.category_id,
                        accountID = tx.from_account_id,
                        notes = notes,
                        status = 0,
                        accountReference = 1, // UNKNOW meanings
                        accountPairID = tx.from_account_id,
                        uidPairID = tx_time_in_milli,
                        deletedTransaction = 6,
                        newSplitTransactionID = 0,
                        transferGroupID = 0
                    ));
                } else {
                    if id_set.contains(&tx_time_in_milli) {
                        // warn!("conflict id: {:?}", tx);
                        tx_time_in_milli = tx_time_in_milli + seq;
                        seq = seq + split_children.len() as i64;
                    }

                    // split header
                    split_children.sort_by(|a, b| a.datetime.partial_cmp(&b.datetime).unwrap());

                    for (index, child) in split_children.iter().enumerate() {
                        let currency = if let Some(currency) = currencies
                            .iter()
                            .find(|c| c.id == child.original_currency_id)
                        {
                            &currency.name
                        } else {
                            "TWD"
                        };

                        statements.push(format!(
                            "INSERT INTO \"TRANSACTIONSTABLE\" (\"transactionsTableID\", \"itemID\", \"amount\", \"transactionCurrency\", \"conversionRateNew\", \"date\", \"transactionTypeID\", \"categoryID\", \"accountID\", \"notes\", \"status\", \"accountReference\", \"accountPairID\", \"uidPairID\", \"deletedTransaction\", \"newSplitTransactionID\", \"transferGroupID\", \"reminderTransaction\", \"reminderGroupID\", \"reminderFrequency\", \"reminderRepeatEvery\", \"reminderEndingType\", \"reminderStartDate\", \"reminderEndDate\", \"reminderAfterNoOfOccurences\", \"reminderAutomaticLogTransaction\", \"reminderRepeatByDayOfMonth\", \"reminderExcludeWeekend\", \"reminderWeekDayMoveSetting\", \"reminderUnbilled\", \"creditCardInstallment\", \"reminderVersion\", \"dataExtraColumnString1\") VALUES ('{transactionsTableID}', '{itemID}', '{amount}', '{transactionCurrency}', '{conversionRateNew}', '{date}', '{transactionTypeID}', '{categoryID}', '{accountID}', '{notes}', '{status}', '{accountReference}', '{accountPairID}', '{uidPairID}', '{deletedTransaction}', '{newSplitTransactionID}', '{transferGroupID}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);",
                            transactionsTableID = tx_time_in_milli + index as i64 + 1,
                            itemID = item_id,
                            amount = child.from_amount * 1000000 / 100, // 2 digit fromn financisto, 6 digit from bluecoins
                            transactionCurrency = currency,
                            conversionRateNew = 1.0,
                            date = tx_time.format("%Y-%m-%d %H:%M:%S"),
                            transactionTypeID = if tx.from_amount >= 0 { 4 } else { 3 }, // 3 = expense, 4 = income
                            categoryID = child.category_id,
                            accountID = child.from_account_id,
                            notes = child.note.as_ref().unwrap_or(&String::new()),
                            status = 0,
                            accountReference = 1, // UNKNOW meanings
                            accountPairID = child.from_account_id,
                            uidPairID = tx_time_in_milli + index as i64 + 1,
                            deletedTransaction = 6,
                            newSplitTransactionID = tx_time_in_milli,
                            transferGroupID = 0
                        ));
                    }
                }
            } else {
                // split items, handled by header
            }
        } else {
            // transfer
            let name = "轉帳";
            let item_id = if items.contains_key(name) {
                *items.get(name).unwrap()
            } else {
                last_item_id = last_item_id + 1;
                items.insert(name.to_owned(), last_item_id);

                statements.push(format!(
                    "INSERT INTO \"ITEMTABLE\" (\"itemTableID\", \"itemName\", \"itemAutoFillVisibility\") VALUES ('{}', '{}', '0');",
                    last_item_id,
                    escape_quote(name),
                ));

                last_item_id
            };

            if id_set.contains(&tx_time_in_milli) {
                // warn!("conflict id: {:?}", tx);
                tx_time_in_milli = tx_time_in_milli + seq;
                seq = seq + 2;
            }
            id_set.insert(tx_time_in_milli);

            // from account -> to account
            statements.push(format!(
                "INSERT INTO \"TRANSACTIONSTABLE\" (\"transactionsTableID\", \"itemID\", \"amount\", \"transactionCurrency\", \"conversionRateNew\", \"date\", \"transactionTypeID\", \"categoryID\", \"accountID\", \"notes\", \"status\", \"accountReference\", \"accountPairID\", \"uidPairID\", \"deletedTransaction\", \"newSplitTransactionID\", \"transferGroupID\", \"reminderTransaction\", \"reminderGroupID\", \"reminderFrequency\", \"reminderRepeatEvery\", \"reminderEndingType\", \"reminderStartDate\", \"reminderEndDate\", \"reminderAfterNoOfOccurences\", \"reminderAutomaticLogTransaction\", \"reminderRepeatByDayOfMonth\", \"reminderExcludeWeekend\", \"reminderWeekDayMoveSetting\", \"reminderUnbilled\", \"creditCardInstallment\", \"reminderVersion\", \"dataExtraColumnString1\") VALUES ('{transactionsTableID}', '{itemID}', '{amount}', '{transactionCurrency}', '{conversionRateNew}', '{date}', '{transactionTypeID}', '{categoryID}', '{accountID}', '{notes}', '{status}', '{accountReference}', '{accountPairID}', '{uidPairID}', '{deletedTransaction}', '{newSplitTransactionID}', '{transferGroupID}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);",
                transactionsTableID = tx_time_in_milli,
                itemID = item_id,
                amount = tx.from_amount * 1000000 / 100, // 2 digit fromn financisto, 6 digit from bluecoins
                transactionCurrency = currency,
                conversionRateNew = 1.0,
                date = tx_time.format("%Y-%m-%d %H:%M:%S"),
                transactionTypeID = 5, // transfer
                categoryID = 3, // transfer
                accountID = tx.from_account_id,
                notes = notes,
                status = 0,
                accountReference = 1, // UNKNOW meanings
                accountPairID = tx.to_account_id,
                uidPairID = tx_time_in_milli + 1, // pair to transactionsTableID
                deletedTransaction = 6,
                newSplitTransactionID = 0,
                transferGroupID = tx_time_in_milli
            ));

            // to account -> from account
            statements.push(format!(
                "INSERT INTO \"TRANSACTIONSTABLE\" (\"transactionsTableID\", \"itemID\", \"amount\", \"transactionCurrency\", \"conversionRateNew\", \"date\", \"transactionTypeID\", \"categoryID\", \"accountID\", \"notes\", \"status\", \"accountReference\", \"accountPairID\", \"uidPairID\", \"deletedTransaction\", \"newSplitTransactionID\", \"transferGroupID\", \"reminderTransaction\", \"reminderGroupID\", \"reminderFrequency\", \"reminderRepeatEvery\", \"reminderEndingType\", \"reminderStartDate\", \"reminderEndDate\", \"reminderAfterNoOfOccurences\", \"reminderAutomaticLogTransaction\", \"reminderRepeatByDayOfMonth\", \"reminderExcludeWeekend\", \"reminderWeekDayMoveSetting\", \"reminderUnbilled\", \"creditCardInstallment\", \"reminderVersion\", \"dataExtraColumnString1\") VALUES ('{transactionsTableID}', '{itemID}', '{amount}', '{transactionCurrency}', '{conversionRateNew}', '{date}', '{transactionTypeID}', '{categoryID}', '{accountID}', '{notes}', '{status}', '{accountReference}', '{accountPairID}', '{uidPairID}', '{deletedTransaction}', '{newSplitTransactionID}', '{transferGroupID}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);",
                transactionsTableID = tx_time_in_milli + 1,
                itemID = item_id,
                amount = tx.to_amount * 1000000 / 100, // 2 digit fromn financisto, 6 digit from bluecoins
                transactionCurrency = currency,
                conversionRateNew = 1.0,
                date = tx_time.format("%Y-%m-%d %H:%M:%S"),
                transactionTypeID = 5, // transfer
                categoryID = 3, // transfer
                accountID = tx.to_account_id,
                notes = notes,
                status = 0,
                accountReference = 2, // UNKNOW meanings
                accountPairID = tx.from_account_id,
                uidPairID = tx_time_in_milli, // pair to transactionsTableID
                deletedTransaction = 6,
                newSplitTransactionID = 0,
                transferGroupID = tx_time_in_milli
            ));
        }
    }

    Ok(statements)
}
