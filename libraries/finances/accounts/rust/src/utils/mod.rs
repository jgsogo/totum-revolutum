pub fn reorder_breadcrumbs(mut results: Vec<(i64, String)>, order: &[i64]) -> Vec<String> {
    results.sort_by(|lhs, rhs| {
        let lhs_index = order.iter().position(|&x| x == lhs.0);
        let rhs_index = order.iter().position(|&x| x == rhs.0);
        lhs_index.cmp(&rhs_index)
    });
    results.into_iter().map(|v| v.1).collect()
}
