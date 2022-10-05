
fn main() {
    println!("Hello, world!");
    dotenv().ok(); 
    let mut env = Environment::new();
    env.add_template("hello.txt", "{{drug_name}} has been approved for use in {{country}} for patients with {{drug_indication}} on {{drug_approval_date}}").unwrap();
    let template = env.get_template("hello.txt").unwrap();
    println!("{}", template.render(context! { drug_name=> "Crocin" ,country => "USA",drug_indication=> "headache",drug_approval_date=>"2110201"}).unwrap());

    let mut database_url = "".to_string();
    database_url = env::var("DATABASE_URL").unwrap();

    let mut username = "".to_string();
    username = env::var("USERNAME").unwrap();

    let mut password = "".to_string();
    password = env::var("PASSWORD").unwrap();

    let mut rawdata_database = "".to_string();
    rawdata_database = env::var("RAW_DATABASE_NAME").unwrap();

    let mut linksdata_database = "".to_string();
    linksdata_database = env::var("LINKS_DATABASE_NAME").unwrap();

    let mut client_options = ClientOptions::parse(database_url).unwrap();
    client_options.credential = Some(
        Credential::builder()
            .username(username)
            .password(password)
            .build(),
    );
    let uri = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri);
    
    let links_client = Client::with_options(client_options.clone());
    let links_database = links_client.database(&linksdata_database);
    
    
    
    let drugIndication_links_collection = links_database.collection::<DrugIndication>("drugIndication");
    let drugapproval_fda_links_collection = links_database.collection::<DrugApproval>("fdaDrugsApproval");
    let drugCompany_links_collection = links_database.collection::<DrugCompany>("drugCompany");

    
    let cursor = drugapproval_fda_links_collection.find(None, None).unwrap();

}
