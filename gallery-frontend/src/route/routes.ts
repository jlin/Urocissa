// src/router.ts

import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import 'vue-router'

import HomeMain from '@/components/Page/HomePage.vue'
import AllPage from '@/components/Page/AllPage.vue'
import FavoritePage from '@/components/Page/FavoritePage.vue'
import ArchivedPage from '@/components/Page/ArchivedPage.vue'
import TrashedPage from '@/components/Page/TrashedPage.vue'
import AlbumsPage from '@/components/Page/AlbumsPage.vue'
import VideosPage from '@/components/Page/VideosPage.vue'
import { createRoute } from './createRoute'
import { tagsRoute } from './tagsRoute'
import { linksRoute } from './linksRoute'
import { loginRoute } from './loginRoute'
import { shareRoute } from './shareRoute'

// ======================================
// Define Simple Static Routes
// ======================================

const simpleRoutes: RouteRecordRaw[] = [
  { path: '/', redirect: '/home' },
  tagsRoute,
  linksRoute,
  loginRoute,
  shareRoute
]

// ======================================
// Create Routes Using the Helper Function
// ======================================

const homePageRoutes = createRoute('home', HomeMain)

const allPageRoutes = createRoute('all', AllPage)

const favoritePageRoutes = createRoute('favorite', FavoritePage)

const archivedPageRoutes = createRoute('archived', ArchivedPage)

const trashedPageRoutes = createRoute('trashed', TrashedPage)

const albumsPageRoutes = createRoute('albums', AlbumsPage)

const videosPageRoutes = createRoute('videos', VideosPage)

// ======================================
// Combine All Routes
// ======================================

const routes: RouteRecordRaw[] = [
  ...simpleRoutes,
  ...homePageRoutes,
  ...allPageRoutes,
  ...favoritePageRoutes,
  ...archivedPageRoutes,
  ...trashedPageRoutes,
  ...albumsPageRoutes,
  ...videosPageRoutes
]

// ======================================
// Create and Export the Router Instance
// ======================================

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
