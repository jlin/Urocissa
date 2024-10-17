import { createRouter, createWebHistory } from 'vue-router'
import { defineAsyncComponent } from 'vue'

const HomePage = defineAsyncComponent(() => import('@/components/Home/Home.vue'))
const ViewPage = defineAsyncComponent(() => import('@/components/Home/View/ViewPage.vue'))
const TagsPage = defineAsyncComponent(() => import('@/components/Page/TagsPage.vue'))
const FavoritePage = defineAsyncComponent(() => import('@/components/Page/FavoritePage.vue'))
const ArchivedPage = defineAsyncComponent(() => import('@/components/Page/ArchivedPage.vue'))
const AllPage = defineAsyncComponent(() => import('@/components/Page/AllPage.vue'))
const LoginPage = defineAsyncComponent(() => import('@/components/LoginPage.vue'))

const simpleRoutes = [
  {
    path: '/tags',
    component: TagsPage,
    name: 'TagsPage',
    meta: {
      navigation: true
    }
  },
  {
    path: '/login',
    component: LoginPage,
    name: 'LoginPage',
    meta: {
      navigation: true
    }
  }
]

function createRouteWithAlias(path: string, component: any, name: string) {
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
        component: ViewPage,
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
        component: ViewPage,
        name: `${name}AliasViewPage`,
        meta: { navigation: false, isViewPage: true, sortable: false }
      }
    ]
  }

  // Return both route configurations
  return [mainRoute, aliasRoute]
}

const homePageRoutes = createRouteWithAlias('', HomePage, 'HomePage')
const allPageRoutes = createRouteWithAlias('all', AllPage, 'AllPage')
const favoritePageRoutes = createRouteWithAlias('favorite', FavoritePage, 'FavoritePage')
const archivedPageRoutes = createRouteWithAlias('archived', ArchivedPage, 'ArchivedPage')
const trashedPageRoutes = createRouteWithAlias('trashed', ArchivedPage, 'TrashedPage')

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
