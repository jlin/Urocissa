declare module '*.vue' {
  import { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module '*/dataWorker?worker&inline' {
  const workerConstructor: new () => Worker
  export default workerConstructor
}
