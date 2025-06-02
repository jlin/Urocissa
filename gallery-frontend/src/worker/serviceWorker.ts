import { getHashToken } from '@/db/db'

self.addEventListener('install', () => {
  console.log('[Service Worker] Installing...')
  const result = self as unknown as ServiceWorkerGlobalScope
  result.skipWaiting().catch((err: unknown) => {
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
        // 讓新的 SW 立即接管所有頁面
        await result.clients.claim()
        console.log('[Service Worker] Clients claimed.')
      } catch (err) {
        console.error('[Service Worker] Failed during activation:', err)
      }
    })()
  )
})

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

  const parts = url.pathname.split('/') // '/media-proxy/imported/abc123.mp4' → ['', 'media-proxy', 'imported', 'abc123.mp4']
  const filename = parts.at(-1) ?? '' // 'abc123.mp4'
  const hash = filename.replace(/\.[^.]+$/, '') // 'abc123'

  let token: string | null
  try {
    token = await getHashToken(hash)
    console.log('token is', token)
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
