export default async (request, context) => {
    const url = new URL(request.url)

    // Get the page content.
    const response = await context.next()
    const page = await response.text()

    try{
        const game = url.pathname.substring(6);
        const search = 'https://myriad-game.com/icon/og_image.png'
        const replace = `https://myriad-game.com/.netlify/functions/image?game=${game}`

        return new Response(page .replaceAll(search, replace), response);
    }
    catch{
        return response;
    }


}