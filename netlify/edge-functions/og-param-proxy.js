export default async (request, context) => {
  const url = new URL(request.url);

  // Get the page content.
  const response = await context.next();
  let page = await response.text();

  try {
    const game = url.pathname.substring(6);

    page = page.replace(
      "https://myriad-game.com/icon/og_image.png",
      `https://myriad-game.com/.netlify/functions/image?game=${game}`
    );

    page = page.replace(
      '<meta property="og:url" content="https://myriad-game.com" />',
      '<meta property="og:url" content="https://myriad-game.com/game/${game}" />'
    );


    page = page.replace(
        '<meta property="og:image:alt" content="The Myriad Logo" />',
        '<meta property="og:image:alt" content="The Myriad Game: ${game}" />'
    )
    return new Response(page.replaceAll(search, replace), response);
  } catch {
    return response;
  }
};
