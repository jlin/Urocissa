// This file initializes the Vue 3 application, sets up the router, state management (Pinia), and Vuetify UI framework.
// It is referenced in index.html to bootstrap the app, configures the application with a dark theme, and mounts it to the DOM.

// Importing core dependencies and main component
import { createApp } from 'vue'
import App from '@/components/App.vue'

// Importing router and state management
import router from '@/script/routes'
import { createPinia } from 'pinia'

// Importing global styles and icons
import '@/style/common.scss'
import '@mdi/font/css/materialdesignicons.css'

// Importing Vuetify UI framework and configuration
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import axios, { AxiosError } from 'axios'
import { useRedirectionStore } from '@/store/redirectionStore'

// Response interceptor to handle 401 Unauthorized
axios.interceptors.response.use(
  (response) => response, // Pass through valid responses
  async (error: AxiosError) => {
    if (error.response && error.response.status === 401) {
      const redirectionStore = useRedirectionStore('mainId')
      redirectionStore.redirectionToLogin()
    }
    return Promise.reject(error) // Always reject the error to maintain default behavior
  }
)

// Create Vue application instance
const app = createApp(App)

// Configure Vuetify with a dark theme
const vuetify = createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'dark'
  }
})

// Setup state management (Pinia)
const pinia = createPinia()

// Apply necessary plugins and mount the app
app.use(pinia)
app.use(router)
app.use(vuetify)
app.mount('#app')
