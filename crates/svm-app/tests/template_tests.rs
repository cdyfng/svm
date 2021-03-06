use svm_app::{
    memory::{JsonMemAppStore, JsonMemAppTemplateStore, JsonMemoryEnv},
    testing::AppTemplateBuilder,
    traits::Env,
    types::AppTemplate,
};
use svm_common::Address;

#[test]
fn store_template() {
    let app_store = JsonMemAppStore::new();
    let template_store = JsonMemAppTemplateStore::new();
    let mut env = JsonMemoryEnv::new(app_store, template_store);

    let code = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let name = "Template #1";
    let page_count = 10;
    let author = Address::from(0x10_20_30_40);

    let bytes = AppTemplateBuilder::new()
        .with_version(0)
        .with_name(name)
        .with_page_count(page_count)
        .with_code(&code)
        .build();

    let template = env.parse_template(&bytes, &author).unwrap();
    let expected_addr = env.derive_template_address(&template);

    let actual_addr = env.store_template(&template).unwrap();
    assert_eq!(expected_addr, actual_addr);

    let expected = AppTemplate {
        name: name.to_string(),
        author,
        page_count,
        code,
    };

    let addr = env.derive_template_address(&template);
    let actual = env.load_template(&addr).unwrap();

    assert_eq!(expected, actual);
}
