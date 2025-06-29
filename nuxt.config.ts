// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@nuxt/eslint', '@nuxt/ui'],
  devtools: { enabled: false },
  css: ['~/assets/main.css'],
  colorMode: {
    preference: 'dark',
    fallback: 'dark',
  },
  ui: {
    fonts: false,
  },
  future: {
    compatibilityVersion: 4,
  },
  compatibilityDate: '2025-05-15',
  eslint: {
    config: {
      stylistic: true,
    },
  },
})
