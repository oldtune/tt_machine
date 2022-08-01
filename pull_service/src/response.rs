pub struct QueryResponse {
    data: Vec<DataItem>,
}

pub struct DataItem {
    item_type: u8,
    item: ItemDetail,
}

pub struct ItemDetail {
    id: String,
    desc: String,
    stats: ItemStat,
}

pub struct ItemStat {
    digg_count: u8,
    share_count: u8,
    comment_count: u8,
    play_count: u8,
}
