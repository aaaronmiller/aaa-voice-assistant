import { createApp } from 'svelte'
import App from './App.svelte'

const app = createApp(App, {
  target: document.getElementById('app')
})

export default app
