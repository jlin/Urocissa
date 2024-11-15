// src/router.ts

import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import 'vue-router'

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
      basicString: null,
      baseName: 'tags'
    }
  },
  {
    path: '/login',
    component: () => import('@/components/LoginPage.vue'),
    name: 'LoginPage',
    meta: {
      isReadPage: false,
      isViewPage: false,
      basicString: null,
      baseName: 'login'
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
      basicString: basicString,
      baseName: name
    },
    children: [
      {
        path: 'view/:hash',
        component: () => import('@/components/Home/View/mainViewPage.vue'),
        name: `${name}ViewPage`,
        meta: { isReadPage: false, isViewPage: true, basicString: basicString, baseName: name },
        children: [
          {
            path: 'read',
            component: () => import('@/components/Home/IsolatedHome.vue'),
            name: `${name}ReadPage`,
            meta: { isReadPage: true, isViewPage: false, basicString: basicString, baseName: name },
            children: [
              {
                path: 'view/:subhash',
                name: `${name}ReadViewPage`,
                component: () => import('@/components/Home/View/isolatedViewPage.vue'),
                meta: {
                  isReadPage: true,
                  isViewPage: true,
                  basicString: basicString,
                  baseName: name
                }
              }
            ]
          }
        ]
      }
    ]
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

declare module 'vue-router' {
  interface RouteMeta {
    isReadPage: boolean
    isViewPage: boolean
    baseName: string
    basicString: string | null
  }
}

export default router
