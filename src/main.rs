#[tokio::main]
async fn main() {
    let url = "https://www.itinisere.fr/fr/donnees-open-data/169/OpenData/Download?fileName=CG38.GTFS.zip";
    let gtfs = gtfs_structures::RawGtfs::from_url_async(url).await;
    println!("{:?}", gtfs);
}
