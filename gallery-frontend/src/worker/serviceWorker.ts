import { getHashToken } from '@/db/db'

self.addEventListener('install', () => {
  console.log('[Service Worker] Installing...')
  const result = self as unknown as ServiceWorkerGlobalScope
  void result.skipWaiting().catch((err: unknown) => {
    console.error('[Service Worker] skipWaiting() failed:', err)
  })
})

self.addEventListener('activate', (event: unknown) => {
  if (!(event instanceof ExtendableEvent)) {
    return
  }

  const result = self as unknown as ServiceWorkerGlobalScope
  console.log('[Service Worker] Activating...')

  event.waitUntil(
    (async () => {
      try {
        await result.clients.claim()
        console.log('[Service Worker] Clients claimed.')
      } catch (err: unknown) {
        console.error('[Service Worker] Failed during activation:', err)
      }
    })()
  )
})

self.addEventListener('fetch', (event: unknown) => {
  if (!(event instanceof FetchEvent)) return

  const url = new URL(event.request.url)
  const shouldHandle = url.pathname.includes('/imported') || url.pathname.endsWith('.mp4')
  if (!shouldHandle) return

  event.respondWith(handleMediaRequest(event.request))
})

async function handleMediaRequest(request: Request): Promise<Response> {
  const url = new URL(request.url)
  const parts = url.pathname.split('/')
  const filename = parts.at(-1) ?? ''
  const hash = filename.replace(/\.[^.]+$/, '')
  console.log('intercepting: hash is', hash)

  let token: string | null
  try {
    token = await getHashToken(hash)
  } catch (err: unknown) {
    console.error('Failed to get token from IndexedDB:', err)
    return new Response('Internal error while accessing IndexedDB', { status: 500 })
  }

  if (typeof token !== 'string' || token.trim() === '') {
    console.error('Token is missing or invalid:', token)
    return new Response('Unauthorized', { status: 401 })
  }

  // 複製原始 headers（例如 Range、Accept-Encoding...），並注入 Authorization
  const originalHeaders = new Headers(request.headers)
  originalHeaders.set('Authorization', `Bearer ${token}`)

  // 建立一個全新的 Request，強制 mode: 'cors'
  const newRequest = new Request(url.toString(), {
    method: request.method,
    headers: originalHeaders,
    mode: 'cors',
    // 若有需要帶 cookie，就改成 'include'
    credentials: 'include',
    // 如果是 GET/HEAD 就不帶 body
    body: request.method === 'GET' || request.method === 'HEAD' ? undefined : request.body
  })

  try {
    return await fetch(newRequest)
  } catch (err: unknown) {
    console.error('Fetch failed:', err)
    // 回一個 502 Bad Gateway 或其他適合的錯誤
    return new Response('Bad Gateway', { status: 502 })
  }
}
