pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub pages: u32,
}

impl Pagination {
    const DEFAULT_PER_PAGE: u32 = 10;
    const DEFAULT_MAX_PER_PAGE: u32 = 100;

    pub fn new(page: u32, per_page: u32, total: i64) -> Self {
        let page = page.max(1);
        let per_page = if per_page == 0 {
            Self::DEFAULT_PER_PAGE
        } else {
            per_page.min(Self::DEFAULT_MAX_PER_PAGE)
        };

        let pages = (total as f32 / per_page as f32).ceil() as u32;
        Self {
            page,
            per_page,
            pages,
        }
    }

    /// Returns the offset for database queries
    pub fn offset(&self) -> u64 {
        ((self.page - 1) * self.per_page) as u64
    }
}
