<template>
  <v-app
    :style="{
      userSelect: scrollbarStore.isDragging ? 'none' : 'auto' // Prevent accidental selection while scrolling.
    }"
  >
    <component :is="NavBar" v-if="route.name !== 'LoginPage'" />
    <v-main class="h-screen">
      <router-view v-slot="{ Component }" :key="routeKey">
        <component :is="Component" />
      </router-view>
    </v-main>
    <NotificationWarn />
  </v-app>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { computed, defineAsyncComponent, onMounted } from 'vue'
import Cookies from 'js-cookie'
import { useScrollbarStore } from '@/store/scrollbarStore'

const NotificationWarn = defineAsyncComponent(() => import('@/components/NotificationWarn.vue'))
const NavBar = defineAsyncComponent(() => import('@/components/NavBar/NavBar.vue'))

const scrollbarStore = useScrollbarStore('')

// Function to check if cookie has no password field
async function checkCookieAndRedirect() {
  const jwt = Cookies.get('jwt')
  if (!jwt) {
    router.push('/login')
  }
}

const route = useRoute()
const router = useRouter()

const currentPage = computed(() => {
  if (route.path.startsWith('/favorite')) {
    return 'favorite'
  } else if (route.path.startsWith('/archived')) {
    return 'archived'
  } else if (route.path.startsWith('/all')) {
    return 'all'
  } else {
    return 'default'
  }
})

// The routeKey is used to ensure that the router-view reloads the Home.vue component properly.
// Without it, Vue may cache the component for optimization, potentially causing bugs.
const routeKey = computed(
  () =>
    `${currentPage.value}-${route.query.search}-${route.query.locate}-${route.query.priority_id}-${route.query.reverse}`
)

onMounted(() => {
  checkCookieAndRedirect()
})
</script>

<style scoped></style>
