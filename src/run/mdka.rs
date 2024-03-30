use mdka::from_html;

// TODO: breaks for some reason, can't call from main.rs, fix
pub fn mdka() {
    let input = r#"
<h1>heading 1</h1>
<p>Hello, world.</p>"#;
    let ret = from_html(input);
    println!("{}", ret);
    // # heading 1
    //
    // Hello, world.
    //
}
