import { LanguageToolResponse, LanguageToolMatch } from '../shared/types'

interface CacheEntry {
  response: LanguageToolResponse
  timestamp: number
}

const cache = new Map<string, CacheEntry>()
const CACHE_TTL = 5 * 60 * 1000 // 5 minutes
const REQUEST_TIMEOUT = 15000 // Increased to 15s

// Request queue to prevent server overload
const MAX_CONCURRENT_REQUESTS = 2
let activeRequests = 0
const requestQueue: Array<() => void> = []

function processQueue(): void {
  while (requestQueue.length > 0 && activeRequests < MAX_CONCURRENT_REQUESTS) {
    const next = requestQueue.shift()
    if (next) next()
  }
}

async function throttledRequest<T>(fn: () => Promise<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    const execute = async () => {
      activeRequests++
      try {
        const result = await fn()
        resolve(result)
      } catch (error) {
        reject(error)
      } finally {
        activeRequests--
        processQueue()
      }
    }

    if (activeRequests < MAX_CONCURRENT_REQUESTS) {
      execute()
    } else {
      requestQueue.push(execute)
    }
  })
}

function getCacheKey(text: string, language: string): string {
  return `${language}:${text}`
}

function cleanCache(): void {
  const now = Date.now()
  for (const [key, entry] of cache.entries()) {
    if (now - entry.timestamp > CACHE_TTL) {
      cache.delete(key)
    }
  }
  // Limit cache size
  if (cache.size > 100) {
    const keysToDelete = Array.from(cache.keys()).slice(0, 50)
    keysToDelete.forEach(k => cache.delete(k))
  }
}

export async function checkText(
  text: string,
  language: 'auto' | 'fr' | 'en',
  apiUrl: string
): Promise<LanguageToolMatch[]> {
  if (text.trim().length < 3) {
    return []
  }

  const langParam = language === 'auto' ? 'auto' : language === 'fr' ? 'fr-FR' : 'en-US'
  const cacheKey = getCacheKey(text, langParam)

  // Check cache first
  const cached = cache.get(cacheKey)
  if (cached && Date.now() - cached.timestamp < CACHE_TTL) {
    console.log('[AutoCorrect] Cache hit for:', text.substring(0, 30))
    return cached.response.matches
  }

  // Clean cache periodically
  if (Math.random() < 0.1) {
    cleanCache()
  }

  // Use throttled request to prevent server overload
  return throttledRequest(async () => {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), REQUEST_TIMEOUT)

    try {
      console.log('[AutoCorrect] Fetching:', text.substring(0, 30), '... (queue:', requestQueue.length, ')')
      const response = await fetch(`${apiUrl}/v2/check`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({
          text,
          language: langParam,
          preferredVariants: 'fr-FR,en-US',
        }),
        signal: controller.signal,
      })

      clearTimeout(timeoutId)

      if (!response.ok) {
        throw new Error(`API error: ${response.status}`)
      }

      const data: LanguageToolResponse = await response.json()

      // Cache the result
      cache.set(cacheKey, {
        response: data,
        timestamp: Date.now(),
      })

      console.log('[AutoCorrect] Got', data.matches.length, 'matches for:', text.substring(0, 30))
      return data.matches
    } catch (error) {
      clearTimeout(timeoutId)
      if (error instanceof Error && error.name === 'AbortError') {
        console.warn('[AutoCorrect] Request timeout for:', text.substring(0, 30))
      } else {
        console.error('[AutoCorrect] API error:', error)
      }
      return []
    }
  })
}

export async function checkConnection(apiUrl: string): Promise<boolean> {
  try {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), 3000)

    const response = await fetch(`${apiUrl}/v2/languages`, {
      signal: controller.signal,
    })

    clearTimeout(timeoutId)
    return response.ok
  } catch {
    return false
  }
}
