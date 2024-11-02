import { createRouter, createWebHistory } from 'vue-router'

// Remove `defineAsyncComponent` and directly use dynamic imports in the route configuration
const simpleRoutes = [
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

function createRouteWithAlias(path: string, component: () => Promise<any>, name: string) {
  // Main route configuration
  const mainRoute = {
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
        meta: { navigation: false, isViewPage: true, sortable: false }
      }
    ]
  }

  // Alias route configuration
  const aliasRoute = {
    path: `/share/:album_id/${path}`,
    component: component,
    // Ensure this name is unique
    name: `${name}Alias`,
    meta: {
      navigation: true,
      sortable: true
    },
    children: [
      {
        path: 'view/:hash',
        component: () => import('@/components/Home/View/ViewPage.vue'),
        name: `${name}AliasViewPage`,
        meta: { navigation: false, isViewPage: true, sortable: false }
      }
    ]
  }

  // Return both route configurations
  return [mainRoute, aliasRoute]
}

// Use dynamic imports for all routes
const homePageRoutes = createRouteWithAlias('', () => import('@/components/Home/Home.vue'), 'HomePage')
const allPageRoutes = createRouteWithAlias('all', () => import('@/components/Page/AllPage.vue'), 'AllPage')
const favoritePageRoutes = createRouteWithAlias('favorite', () => import('@/components/Page/FavoritePage.vue'), 'FavoritePage')
const archivedPageRoutes = createRouteWithAlias('archived', () => import('@/components/Page/ArchivedPage.vue'), 'ArchivedPage')
const trashedPageRoutes = createRouteWithAlias('trashed', () => import('@/components/Page/ArchivedPage.vue'), 'TrashedPage')

const routes = simpleRoutes.concat(
  homePageRoutes,
  allPageRoutes,
  favoritePageRoutes,
  archivedPageRoutes,
  trashedPageRoutes
)

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
