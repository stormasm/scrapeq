use scraper::{Html, Selector};

fn main() {
    let html = String::from(
        r#"
      <html>
        <head>
          <title>Test</title>
        </head>
        <body>
          <div id="foo"><div></div><div><div></div><div class="inner"><span>x<div>yo</div></span></div></div></div>
        </body>
      </html>
    "#,
    );

    let parsed_html = Html::parse_document(&html);
    let fragment = parsed_html
        .select(&Selector::parse("body").unwrap())
        .next()
        .unwrap();
    let foo = fragment
        .select(&Selector::parse("div#foo").unwrap())
        .next()
        .unwrap();

    let text = foo
        .children()
        .nth(1)
        .unwrap()
        .children()
        .nth(1)
        .unwrap()
        .children()
        .map(|child| child.value())
        .collect::<Vec<_>>();

    println!("{:?}", text);
}
