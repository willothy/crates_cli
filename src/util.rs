use crates_io_api::Sort;

pub fn get_sort_type(sort: &str) -> Sort {
    match sort {
        "relevance" => Sort::Relevance,
        "downloads" => Sort::Downloads,
        "newly-added" => Sort::NewlyAdded,
        "recent-downloads" => Sort::RecentDownloads,
        "recently-updated" => Sort::RecentUpdates,
        "alphabetical" => Sort::Alphabetical,
        _ => Sort::Relevance,
    }
}
