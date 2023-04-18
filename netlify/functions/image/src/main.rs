use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use http::HeaderValue;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use resvg::usvg::{fontdb, Tree, TreeParsing, TreeTextToPath};
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

    let game = e
        .payload
        .query_string_parameters
        .iter()
        .filter(|x| x.0.eq_ignore_ascii_case("game"))
        .map(|x| x.1)
        .next()
        .unwrap_or_else(|| "myriad123");

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

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const WHITE: &str = "#f7f5f0";
const BLACK: &str = "#1f1b20";
const GRAY: &str = "#a1a9b0";

fn try_map_char(c: &char) -> Option<char> {
    if c.is_ascii_digit(){
        return Some(*c);
    }

    Some(match c.to_ascii_lowercase() {
        'i' => 'Ⅰ',
        'v' => 'Ⅴ',
        'x' => 'Ⅹ',
        'l' => 'Ⅼ',
        'c' => 'Ⅽ',

        '-' => '-',
        '⨉' => '×',
        '×' => '×',
        '+' => '+',
        '*' => '×',
        '/' => '÷',
        '÷' => '÷',
        '_' => ' ',
        ' ' => '+', //this is an artifact of how urls work
        _ => return None,
    })
}

fn try_map_chars(input: &str)-> Option<[char; 9]>{
    let mut arr : [char; 9] = [' '; 9];

    for (index,char) in input.chars().enumerate(){
        let c= try_map_char(&char)?;
        arr[index] = c;
    }

    return Some(arr);
}

fn make_svg_text(chars: &[char; 9]) -> String {
    let mut svg_text: String = "".to_string();
    svg_text.push_str(format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 238.1 238.1">"#).as_str());
    svg_text.push('\n');

    svg_text.push_str(format!(r#"<path d="M0 0h238.1v238.1H0z" style="fill:{WHITE};stroke:{GRAY};stroke-width:4;" />"#).as_str());
    svg_text.push('\n');

    for (i, c) in chars.iter().enumerate().take(9) {
        let x = match i % 3 {
            0 => 13.2,
            1 => 85.3,
            _ => 157.4,
        };
        let y = match i / 3 {
            0 => 13.2,
            1 => 85.3,
            _ => 157.4,
        };



        svg_text.push_str(format!(
            r#"
        <g transform="translate({x} {y})">
            <circle cx="33.7" cy="33.7" r="31.8"
                style="fill:none;stroke:{GRAY};stroke-width:4;" />
            <text  x="18.5" y="50"
                style="stroke:{BLACK};font-size:50px;line-height:1.25;font-family:Inconsolata;font-weight:1000;stroke-width:.25">
                <tspan x="18.5" y="50" style="font-size:50px;stroke-width:.25">{c}</tspan>
            </text>
        </g>

        "#
        ).as_str());
        svg_text.push('\n');
    }

    svg_text.push_str("</svg>");

    return svg_text;
}

fn make_svg(game: &str)-> String{

    let chars = try_map_chars(game).unwrap_or([' ',' ',' ',' ','?',' ',' ',' ',' ',]);
    let svg_data = make_svg_text(&chars);
    svg_data
}

fn draw_image(game: &str) -> Vec<u8> {
    let opt: resvg::usvg::Options = Default::default();
    let svg_data = make_svg(game);

    //println!("{svg_data}");

    let mut tree = match Tree::from_data(&svg_data.as_bytes(), &opt) {
        Ok(tree) => tree,
        Err(e) => panic!("{e}"),
    };

    let data: Vec<u8> = include_bytes!("Inconsolata-Regular.ttf")
        .into_iter()
        .cloned()
        .collect();

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
    use crate::{draw_image, make_svg};

    #[test]
    fn test_svg(){
        let svg: String = make_svg("-1+4/78 5");
        std::fs::write("og_example.svg", svg).unwrap();
    }

    #[test]
    fn parse_test() {
        let data = draw_image("-1+4/78 5");
        std::fs::write("parse_test.png", data).unwrap();
    }

    #[test]
    fn unknown_test() {
        let data = draw_image("null");
        std::fs::write("unknown.png", data).unwrap();
    }
}
