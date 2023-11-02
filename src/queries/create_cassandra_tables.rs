use cdrs::query::QueryExecutor;
use crate::log_writer;
use crate::queries::database_connection::CassandraStoreSession;

impl CassandraStoreSession{

    pub async fn create_tables(&self){
        self.create_seller_table().await;
        self.create_state_table().await;
    }

    pub async fn create_seller_table(&self) {
        let create_seller: &'static str = "CREATE TABLE IF NOT EXISTS rapidshyp.sellers (
        seller_id uuid,
        first_name text,
        last_name text,
        email text,
        primary_mobile bigint,
        company_name text,
        brand_name text,
        website text,
        company_logo_link text,
        company_address text,
        company_address_2 text,
        address_pincode int,
        city text,
        state text,
        country text,
        last_billing_update bigint,
        current_tier text,
        current_tier_id int,
        current_saas_plan text,
        current_saas_plan_id int,
        is_mobile_verified Boolean,
        is_email_verified Boolean,
        registered_since bigint,
        last_login bigint,
        lut bigint,
        pickup_locations text,
        PRIMARY KEY ((seller_id, email))
    );";

        self.session.query(create_seller).unwrap();
        log_writer!("Seller table created successfully...");

        let email_index: &'static str = "CREATE INDEX IF NOT EXISTS ON rapidshyp.sellers (email);";
        self.session.query(email_index).unwrap();
        log_writer!("Seller email indexes created successfully...");

        let contact_index: &'static str = "CREATE INDEX IF NOT EXISTS ON rapidshyp.sellers (primary_mobile);";
        self.session.query(contact_index).unwrap();
        log_writer!("Seller primary_mobile indexes created successfully...");
    }

    pub async fn  create_state_table(&self){
        let create_state  : &'static str = "CREATE TABLE IF NOT EXISTS rapidshyp.state (
        state_id uuid,
        state_name text);";

        self.session.query(create_state).unwrap();
        log_writer!("Seller table created successfully...");

        let state_name: &'static str = "CREATE INDEX IF NOT EXISTS ON rapidshyp.state (state_name);";
        self.session.query(state_name).unwrap();
        log_writer!("state_name indexes created successfully...");
    }


}

