use torytis::functions::replace_skin_html_content::replace_skin_html_content;

#[test]
fn replace_skin_html_content_test() {
    let content = "<a tt-onlyattr=\"[##_prev_page_##]\" href=\"#\"";
    let content_string = content.to_string();
    replace_skin_html_content(&content_string, &false);
}