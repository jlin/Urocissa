// src/router.ts

import {
  createRouter,
  createWebHistory,
  RouteLocationNormalizedLoadedGeneric,
  RouteRecordRaw
} from 'vue-router'

// ======================================
// 1. Define Simple Static Routes
// ======================================

const simpleRoutes: RouteRecordRaw[] = [
  {
    path: '/tags',
    component: () => import('@/components/Page/TagsPage.vue'),
    name: 'TagsPage',
    meta: {
      isReadPage: false,
      isViewPage: false,
      basicString: null
    }
  },
  {
    path: '/login',
    component: () => import('@/components/LoginPage.vue'),
    name: 'LoginPage',
    meta: {
      isReadPage: false,
      isViewPage: false,
      basicString: null
    }
  }
]

// ======================================
// 2. Define a Helper Function to Create Routes
// ======================================

/**
 * Creates a main route with an optional child route.
 *
 * @param path - The base path for the route.
 * @param component - The component to be rendered.
 * @param name - The unique name for the route.
 * @returns An array containing the RouteRecordRaw object.
 */
function createRoute(
  path: string,
  component: () => Promise<any>,
  name: string,
  basicString: string | null
): RouteRecordRaw[] {
  const mainRoute: RouteRecordRaw = {
    path: `/${path}`,
    component: component,
    name: name,
    meta: {
      isReadPage: false,
      isViewPage: false,
      basicString: basicString
    },
    children: [
      {
        path: 'view/:hash',
        component: () => import('@/components/Home/View/mainViewPage.vue'),
        name: `${name}ViewPage`,
        meta: { isReadPage: false, isViewPage: true, basicString: basicString },
        children: [
          {
            path: 'read',
            component: () => import('@/components/Home/IsolatedHome.vue'),
            name: `${name}ReadPage`,
            meta: { isReadPage: true, isViewPage: false, basicString: basicString },
            children: [
              {
                path: 'view/:subhash',
                component: () => import('@/components/Home/View/isolatedViewPage.vue'),
                meta: { isReadPage: true, isViewPage: true, basicString: basicString }
              }
            ]
          }
        ]
      }
    ],
    props: true
  }

  return [mainRoute]
}

// ======================================
// 3. Create Routes Using the Helper Function
// ======================================

const homePageRoutes = createRoute(
  '',
  () => import('@/components/Home/mainHome.vue'),
  'HomePage',
  'and(not(tag: _archived), not(tag:_trashed))'
)

const allPageRoutes = createRoute(
  'all',
  () => import('@/components/Page/AllPage.vue'),
  'AllPage',
  null
)

const favoritePageRoutes = createRoute(
  'favorite',
  () => import('@/components/Page/FavoritePage.vue'),
  'FavoritePage',
  'and(tag:_favorite, not(tag:_trashed))'
)

const archivedPageRoutes = createRoute(
  'archived',
  () => import('@/components/Page/ArchivedPage.vue'),
  'ArchivedPage',
  'and(tag:_archived, not(tag:_trashed))'
)

const trashedPageRoutes = createRoute(
  'trashed',
  () => import('@/components/Page/TrashedPage.vue'),
  'TrashedPage',
  'and(tag:_trashed)'
)

const albumsPageRoutes = createRoute(
  'albums',
  () => import('@/components/Page/AlbumsPage.vue'),
  'AlbumsPage',
  'type:album'
)

// ======================================
// 4. Combine All Routes
// ======================================

const routes: RouteRecordRaw[] = [
  ...simpleRoutes,
  ...homePageRoutes,
  ...allPageRoutes,
  ...favoritePageRoutes,
  ...archivedPageRoutes,
  ...trashedPageRoutes,
  ...albumsPageRoutes
]

// ======================================
// 5. Create and Export the Router Instance
// ======================================

const router = createRouter({
  history: createWebHistory(),
  routes
})

export function pathLeave(route: RouteLocationNormalizedLoadedGeneric) {
  // Get the current full path, removing query parameters and hash (if any)
  let fullPath = route.fullPath.split('?')[0].split('#')[0]

  // Ensure the path starts with a slash
  if (!fullPath.startsWith('/')) {
    fullPath = '/' + fullPath
  }

  // Split the path into an array by slashes
  const pathSegments = fullPath.split('/')

  // Remove the last empty element if the path ends with a slash
  if (pathSegments[pathSegments.length - 1] === '') {
    pathSegments.pop()
  }

  // Remove the last path segment
  if (pathSegments.length > 1) {
    pathSegments.pop()
    const parentPath = pathSegments.join('/') || '/'
    return parentPath
  } else {
    // Already the root path, return '/'
    return '/'
  }
}

export function pathLeaveDouble(route: RouteLocationNormalizedLoadedGeneric) {
  // Get the current full path, removing query parameters and hash (if any)
  let fullPath = route.fullPath.split('?')[0].split('#')[0]

  // Ensure the path starts with a slash
  if (!fullPath.startsWith('/')) {
    fullPath = '/' + fullPath
  }

  // Split the path into an array by slashes
  let pathSegments = fullPath.split('/')

  // Remove empty strings caused by leading or trailing slashes
  pathSegments = pathSegments.filter((segment) => segment.length > 0)

  // Delete the last two path segments
  if (pathSegments.length >= 2) {
    // Remove the last two elements
    pathSegments.splice(-2, 2)
  } else {
    // If there are fewer than two path segments, return the root path
    return '/'
  }

  // Reconstruct the parent path
  const parentPath = '/' + pathSegments.join('/')

  // Ensure at least '/' is returned
  return parentPath || '/'
}

export function appendViewPath(route: RouteLocationNormalizedLoadedGeneric, hashOrId: string) {
  // Get the current path (excluding query parameters and hash)
  const currentPath = route.path

  // Ensure the path does not end with a slash to avoid double slashes
  const normalizedPath = currentPath.endsWith('/') ? currentPath.slice(0, -1) : currentPath

  // Build the new path
  const newPath = `${normalizedPath}/view/${encodeURIComponent(hashOrId)}`

  // Return the route object, including query parameters and hash if needed to preserve
  return {
    path: newPath,
    query: route.query // Preserve query parameters (optional)
  }
}

declare module 'vue-router' {
  interface RouteMeta {
    isReadPage: boolean
    isViewPage: boolean
    basicString: string | null
  }
}

export default router
