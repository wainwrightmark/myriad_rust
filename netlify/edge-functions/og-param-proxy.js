export default async (request, context) => {
  const url = new URL(request.url);

  // Get the page content.
  const response = await context.next();
  let page = await response.text();

  try {
    const game = url.pathname.substring(6);

    page = page.replace(
      `https://myriad-game.com/icon/og_image_square.png`,
      `https://myriad-game.com/.netlify/functions/image?level=${game}&width=1080&height=1080`
    );

    page = page.replace(
      `https://myriad-game.com/icon/og_image_landscape.png`,
      `https://myriad-game.com/.netlify/functions/image?level=${game}&width=1200&height=630`
    );

    page = page.replace(
      `<meta property="og:url" content="https://myriad-game.com"`,
      `<meta property="og:url" content="https://myriad-game.com/game/${game}"`
    );


    page = page.replace(
        `<meta property="og:image:alt" content="The Myriad Logo"`,
        `<meta property="og:image:alt" content="The Myriad Game: ${game}"`
    )
    return new Response(page, response);
  } catch {
    return response;
  }
};
