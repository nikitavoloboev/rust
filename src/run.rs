use mdka::from_html;

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
