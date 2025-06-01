self.addEventListener('fetch', (event: unknown) => {
  if (!(event instanceof FetchEvent)) return

  const url = new URL(event.request.url)
  if (!url.pathname.startsWith('/media-proxy/')) return

  event.respondWith(handleMediaRequest(event.request))
})

async function handleMediaRequest(request: Request): Promise<Response> {
  const cache = await caches.open('auth-cache')
  const tokenResponse = await cache.match('token')
  const token = await tokenResponse?.text()

  if (!token || token.trim() === '') {
    return new Response('Unauthorized', { status: 401 })
  }

  const match = request.url.match(/\/media-proxy\/(.+)$/)
  if (!match || !match[1]) {
    return new Response('Bad request', { status: 400 })
  }

  const realUrl = `https://your.origin.com/${match[1]}`
  return fetch(realUrl, {
    headers: { Authorization: `Bearer ${token}` },
    mode: 'cors',
    credentials: 'omit'
  })
}
