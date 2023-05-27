use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::http::{HeaderMap, HeaderValue};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use resvg::usvg::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let f = service_fn(image_request_handler);
    lambda_runtime::run(f).await?;
    Ok(())
}

async fn image_request_handler(
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
        .unwrap_or_else(|| "");

    let data = draw_image(game);

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Binary(data)),
        is_base64_encoded: true,
    };

    Ok(resp)
}

fn try_map_char(c: &char) -> Option<char> {
    if c.is_ascii_digit() {
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

fn try_map_chars(input1: &str) -> Option<[char; 9]> {
    let mut arr: [char; 9] = [' '; 9];
    let input2 = input1
        .replace(' ', "+")
        .replace("%C3%B7", "÷")
        .replace("%C3%97", "×");

    for (index, char) in input2.chars().enumerate() {
        let c = try_map_char(&char)?;
        arr[index] = c;
    }

    return Some(arr);
}

fn draw_image(game: &str) -> Vec<u8> {
    let opt: resvg::usvg::Options = Default::default();

    let mut tree = Tree::from_data(include_bytes!("template.svg"), &opt)
        .expect("Could not parse template.svg");
    let chars = try_map_chars(game).unwrap_or([' ', ' ', ' ', ' ', '?', ' ', ' ', ' ', ' ']);

    for (index, char) in chars.into_iter().enumerate() {
        let id = format!("text{}", index);
        let node = tree
            .node_by_id(id.as_str())
            .expect("Could not find node by id");
        if let NodeKind::Text(ref mut text) = *node.borrow_mut() {
            text.chunks[0].text = char.to_string();
        } else{
            panic!("Node was not a text node")
        };
    }

    let font_data: Vec<u8> = include_bytes!("Inconsolata-Regular.ttf").to_vec();

    let mut font_database: fontdb::Database = fontdb::Database::new();
    font_database.load_font_data(font_data);

    tree.convert_text(&font_database);
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 1080;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(WIDTH, HEIGHT).expect("Could not create Pixmap");

    let x_scale = WIDTH as f64 / tree.size.width();
    let y_scale = HEIGHT as f64 / tree.size.height();

    resvg::Tree::render(
        &resvg::Tree::from_usvg(&tree),
        resvg::tiny_skia::Transform::from_scale(x_scale as f32, y_scale as f32),
        &mut pixmap.as_mut(),
    );

    pixmap.encode_png().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

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
