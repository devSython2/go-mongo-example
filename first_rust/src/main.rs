
fn main() {
    println!("Hello, world!");
    
    
    
    let drugIndication_links_collection = links_database.collection::<DrugIndication>("drugIndication");
    let drugapproval_fda_links_collection = links_database.collection::<DrugApproval>("fdaDrugsApproval");
    let drugCompany_links_collection = links_database.collection::<DrugCompany>("drugCompany");

    
    let cursor = drugapproval_fda_links_collection.find(None, None).unwrap();

}
