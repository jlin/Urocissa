import { useImgStore } from '@/store/imgStore'
import { createHandler } from 'typesafe-agent-events'
import { fromImgWorker } from '@/worker/workerApi'

const workerHandlerMap = new Map<Worker, (e: MessageEvent) => void>()

export function handleImgWorker(imgWorker: Worker) {
  const imgStore = useImgStore()

  const handler = createHandler<typeof fromImgWorker>({
    smallImageProcessed({ index, url }) {
      imgStore.imgUrl.set(index, url)
    },
    imageProcessed({ index, url }) {
      imgStore.imgOriginal.set(index, url)
    }
  })

  const messageHandler = (e: MessageEvent) => {
    handler(e.data)
  }

  imgWorker.addEventListener('message', messageHandler)

  workerHandlerMap.set(imgWorker, messageHandler)
}

export function removeHandleImgWorkerReturn(dataWorker: Worker) {
  const messageHandler = workerHandlerMap.get(dataWorker)
  if (messageHandler) {
    dataWorker.removeEventListener('message', messageHandler)
    workerHandlerMap.delete(dataWorker)
  }
}
