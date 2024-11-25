// https://github.com/magiclen/paginator
use core::fmt::Write;
use paginator::{PageItem, Paginator, YesNoDepends};

pub fn build_links(resource_path: &str, current_page: usize, total_pages: usize) -> String {
    let paginator = Paginator::builder(total_pages)
        .current_page(current_page)
        .has_prev(YesNoDepends::Yes)
        .has_next(YesNoDepends::Yes)
        .build_paginator()
        .unwrap();

    let mut html = String::new();

    for page_item in paginator.paginate() {
        match page_item {
            PageItem::ReservedPrev => {
                html.write_fmt(format_args!(
                    "<li class=\"page-item disabled\"><a class=\"page-link\">Previous</a></li>\n"
                ))
                .unwrap();
            }
            PageItem::Prev(page) => {
                html.write_fmt(format_args!("<li class=\"page-item\"><a class=\"page-link\" href=\"{resource_path}/{page}\">Previous</a></li>\n")).unwrap();
            }
            PageItem::Page(page) => {
                html.write_fmt(format_args!("<li class=\"page-item\"><a class=\"page-link\" href=\"{resource_path}/{page}\">{page}</a></li>\n")).unwrap();
            }
            PageItem::CurrentPage(page) => {
                html.write_fmt(format_args!(
                    "<li class=\"page-item active\"><a class=\"page-link\">{page}</a></li>\n"
                ))
                .unwrap();
            }
            PageItem::Ignore => {
                html.push_str("<li class=\"page-item\"><a class=\"page-link\">...</a></li>\n");
            }
            PageItem::Next(page) => {
                html.write_fmt(format_args!("<li class=\"page-item\"><a class=\"page-link\" href=\"{resource_path}/{page}\">Next</a></li>\n")).unwrap();
            }
            PageItem::ReservedNext => {
                html.write_fmt(format_args!(
                    "<li class=\"page-item disabled\"><a class=\"page-link\">Next</a></li>\n"
                ))
                .unwrap();
            }
        }
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginate() {
        let html = build_links("/posts/pages", 1, 1);
        println!("{}", html);

        let html = build_links("/posts/pages", 1, 2);
        println!("{}", html);

        let html = build_links("/posts/pages", 1, 20);
        println!("{}", html);

        let html = build_links("/posts/pages", 2, 20);
        println!("{}", html);

        let html = build_links("/posts/pages", 20, 20);
        println!("{}", html);
    }
}
