// This file initializes the Vue 3 application specifically for the login page, setting up the router.
// It is referenced in login.html to bootstrap the login page and mounts it to the DOM.

// Importing core dependencies and the login page component
import { createApp } from 'vue'
import router from '@/script/routes'
import LoginPage from '@/components/LoginPage.vue'

// Importing global styles
import '@/style/common.scss'

// Create Vue application instance with the LoginPage component
const app = createApp(LoginPage)

// Apply the router to the app
app.use(router)

// Mount the Vue application to the DOM
app.mount('#app')
