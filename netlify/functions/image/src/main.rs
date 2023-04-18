use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use http::HeaderValue;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use resvg::usvg::{Tree, Options, TreeParsing, fontdb, TreeTextToPath};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(
    e: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/png"));

    let game = e.payload.query_string_parameters.iter().filter(|x|x.0.eq_ignore_ascii_case("game"))
    .map(|x|x.1).next().unwrap_or_else(||"myriad123");

    let data = draw_image(game);

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Binary(data)),
        is_base64_encoded: Some(true),
    };

    Ok(resp)
}

fn get_options() -> Options {
    let opt = resvg::usvg::Options::default();
    opt
}

 const WIDTH: u32 = 900;
 const HEIGHT: u32 = 900;

 fn make_text(chars: &str)-> String{

    let mut text: String = "".to_string();
    text.push_str(format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 238.1 238.1">"#).as_str());
    text.push('\n');

    text.push_str(r#"<path d="M0 0h238.1v238.1H0z" style="fill:#fff;" />"#);
    text.push('\n');

    for (i, c) in chars.chars().enumerate().take(9){
        let x = match i % 3{
            0=> 13.2,
            1=> 85.3,
            _=>157.4
        };
        let y = match i / 3{
            0=> 13.2,
            1=> 85.3,
            _=>157.4
        };

        text.push_str(format!(
            r#"
        <g transform="translate({x} {y})">
            <circle cx="33.7" cy="33.7" r="31.8"
                style="fill:none;stroke:#000;stroke-width:4;" />
            <text xml:space="preserve" x="21" y="50"
                style="font-size:50px;line-height:1.25;font-family:Inconsolata;font-weight:1000;stroke-width:.25">
                <tspan x="21" y="50" style="font-size:50px;stroke-width:.25">{c}</tspan>
            </text>
        </g>

        "#
        ).as_str());
        text.push('\n');
    }


    text.push_str("</svg>");

    return text;
 }

fn draw_image(game: &str) -> Vec<u8> {

    let opt: resvg::usvg::Options = get_options();
    let svg_data = make_text(game);

    //println!("{svg_data}");

    let mut tree = match Tree::from_data(&svg_data.as_bytes(), &opt){
        Ok(tree) => tree,
        Err(e) => panic!("{e}"),
    };

    let data: Vec<u8> = include_bytes!("Inconsolata-Regular.ttf").into_iter().cloned().collect();

    let mut font_database: fontdb::Database = fontdb::Database::new();
    //font_database.load_system_fonts();
    font_database.load_font_data(data);

    // println!("Fonts:");
    // for x in font_database.faces(){
    //     println!("{} {}: {}", x.id, x.post_script_name,x.families.iter().map(|x|x.0.clone()).next().unwrap())
    // }

    tree.convert_text(&font_database);


    let mut pixmap = resvg::tiny_skia::Pixmap::new(WIDTH, HEIGHT).unwrap();

    use resvg::FitTo;
    resvg::render(
        &tree,
        FitTo::Size(WIDTH, HEIGHT),
        resvg::tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();

    pixmap.encode_png().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::draw_image;

    #[test]
    fn it_works() {

        let data = draw_image("-184578+5");
        std::fs::write("test.png", data).unwrap();
    }
}