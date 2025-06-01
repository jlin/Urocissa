import { getHashToken } from '@/db/db'

self.addEventListener('fetch', (event: unknown) => {
  if (!(event instanceof FetchEvent)) return

  const url = new URL(event.request.url)
  if (!url.pathname.startsWith('/media-proxy/')) return

  event.respondWith(handleMediaRequest(event.request))
})

async function handleMediaRequest(request: Request): Promise<Response> {
  const match = request.url.match(/\/media-proxy\/(.+)$/)
  if (!match || !match[1]) {
    return new Response('Bad request', { status: 400 })
  }

  // 從 IndexedDB 取得 token
  let token: string | null = null
  try {
    token = await getHashToken('token') // 假設 key 是 'token'
  } catch (err) {
    return new Response('Internal error while accessing IndexedDB', { status: 500 })
  }

  if (!token || token.trim() === '') {
    return new Response('Unauthorized', { status: 401 })
  }

  const realUrl = `https://your.origin.com/${match[1]}`
  return fetch(realUrl, {
    headers: { Authorization: `Bearer ${token}` },
    mode: 'cors',
    credentials: 'omit'
  })
}
