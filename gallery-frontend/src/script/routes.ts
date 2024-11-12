// src/router.ts

import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

// ======================================
// 1. Define Simple Static Routes
// ======================================

const simpleRoutes: RouteRecordRaw[] = [
  {
    path: '/tags',
    component: () => import('@/components/Page/TagsPage.vue'),
    name: 'TagsPage',
    meta: {
      navigation: true
    }
  },
  {
    path: '/login',
    component: () => import('@/components/LoginPage.vue'),
    name: 'LoginPage',
    meta: {
      navigation: true
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
function createRoute(path: string, component: () => Promise<any>, name: string): RouteRecordRaw[] {
  const mainRoute: RouteRecordRaw = {
    path: `/${path}`,
    component: component,
    name: name,
    meta: {
      navigation: true,
      sortable: true
    },
    children: [
      {
        path: 'view/:hash',
        component: () => import('@/components/Home/View/ViewPage.vue'),
        name: `${name}ViewPage`,
        meta: { navigation: false, isViewPage: true, sortable: false },
        children: [
          {
            path: 'read',
            component: () => import('@/components/Home/IsolatedHome.vue'),
            name: `${name}ReadPage`
          }
        ]
      }
    ],
    props: path.includes(':') // Enable props if the path has dynamic segments
  }

  return [mainRoute]
}

// ======================================
// 3. Create Routes Using the Helper Function
// ======================================

const homePageRoutes = createRoute('', () => import('@/components/Home/Home.vue'), 'HomePage')

const allPageRoutes = createRoute('all', () => import('@/components/Page/AllPage.vue'), 'AllPage')

const favoritePageRoutes = createRoute(
  'favorite',
  () => import('@/components/Page/FavoritePage.vue'),
  'FavoritePage'
)

const archivedPageRoutes = createRoute(
  'archived',
  () => import('@/components/Page/ArchivedPage.vue'),
  'ArchivedPage'
)

const trashedPageRoutes = createRoute(
  'trashed',
  () => import('@/components/Page/TrashedPage.vue'),
  'TrashedPage'
)

const albumsPageRoutes = createRoute(
  'albums',
  () => import('@/components/Page/AlbumsPage.vue'),
  'AlbumsPage'
)

// ======================================
// 4. Define Dynamic Route for InsideAlbum
// ======================================

/**
 * Creates routes for the InsideAlbum page with dynamic `id` parameter.
 *
 * @returns An array containing the RouteRecordRaw object.
 */
function createRouteForInsideAlbum(): RouteRecordRaw[] {
  const mainRoute: RouteRecordRaw = {
    path: '/album-:id',
    component: () => import('@/components/Page/InsideAlbumPage.vue'),
    name: 'InsideAlbumPage',
    meta: {
      navigation: true,
      sortable: true,
      isInsideAlbum: true
    },
    children: [
      {
        path: 'view/:hash',
        component: () => import('@/components/Home/View/ViewPage.vue'),
        name: 'InsideAlbumPageViewPage',
        meta: { navigation: false, isViewPage: true, sortable: false }
      }
    ],
    props: true // Enables passing route params as props to the component
  }

  return [mainRoute]
}

const insideAlbumPageRoutes = createRouteForInsideAlbum()

// ======================================
// 5. Define a Catch-All Route for 404 Errors
// ======================================

// ======================================
// 6. Combine All Routes
// ======================================

const routes: RouteRecordRaw[] = [
  ...simpleRoutes,
  ...homePageRoutes,
  ...allPageRoutes,
  ...favoritePageRoutes,
  ...archivedPageRoutes,
  ...trashedPageRoutes,
  ...albumsPageRoutes,
  ...insideAlbumPageRoutes
]

// ======================================
// 7. Create and Export the Router Instance
// ======================================

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
