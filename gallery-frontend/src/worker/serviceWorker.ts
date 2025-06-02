import { getHashToken } from '@/db/db'

self.addEventListener('fetch', (event: unknown) => {
  if (!(event instanceof FetchEvent)) return

  const url = new URL(event.request.url)

  // Only handle if path includes "/imported" or ends with ".mp4"
  const shouldHandle = url.pathname.includes('/imported') || url.pathname.endsWith('.mp4')
  if (!shouldHandle) return

  event.respondWith(handleMediaRequest(event.request))
})

async function handleMediaRequest(request: Request): Promise<Response> {
  const url = new URL(request.url)

  const parts = url.pathname.split('/')
  const filename = parts.at(-1) ?? ''
  const hash = filename.replace(/\.[^.]+$/, '') // remove file extension

  let token: string | null
  try {
    token = await getHashToken(hash)
  } catch {
    return new Response('Internal error while accessing IndexedDB', { status: 500 })
  }

  if (typeof token !== 'string' || token.trim() === '') {
    return new Response('Unauthorized', { status: 401 })
  }

  const realUrl = `https://your.origin.com/${hash}`
  return fetch(realUrl, {
    headers: { Authorization: `Bearer ${token}` },
    mode: 'cors',
    credentials: 'omit'
  })
}
