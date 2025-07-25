import './assets/main.css'

import { createApp } from 'vue'
import { createPinia} from 'pinia'
import App from './App.vue'
import router from './router'
import { PrimeVue } from '@primevue/core'
import Material from '@primeuix/themes/material';
import { ToastService } from 'primevue'

// Import Google Fonts
const link = document.createElement('link')
link.rel = 'preconnect'
link.href = 'https://fonts.googleapis.com'
document.head.appendChild(link)

const link2 = document.createElement('link')
link2.rel = 'preconnect'
link2.href = 'https://fonts.gstatic.com'
link2.crossOrigin = 'anonymous'
document.head.appendChild(link2)

const link3 = document.createElement('link')
link3.rel = 'stylesheet'
link3.href = 'https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap'
document.head.appendChild(link3)

const pinia = createPinia()

createApp(App)
    .use(router)
    .use(pinia)
    .use(PrimeVue, {
        theme: {
            preset: Material
        }
    })
    .use(ToastService)
    .mount('#app')
