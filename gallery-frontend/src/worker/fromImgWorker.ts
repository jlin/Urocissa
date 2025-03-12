import { useImgStore } from '@/store/imgStore'
import { createHandler } from 'typesafe-agent-events'
import { fromImgWorker } from '@/worker/workerApi'
import { IsolationId } from '@type/types'
import { useMessageStore } from '@/store/messageStore'
import { useRedirectionStore } from '@/store/redirectionStore'
const workerHandlerMap = new Map<Worker, (e: MessageEvent) => void>()

export function handleImgWorker(imgWorker: Worker, isolationId: IsolationId) {
  const imgStore = useImgStore(isolationId)
  const messageStore = useMessageStore('mainId')
  const redirectionStore = useRedirectionStore('mainId')

  const handler = createHandler<typeof fromImgWorker>({
    smallImageProcessed({ index, url }) {
      imgStore.imgUrl.set(index, url)
    },
    imageProcessed({ index, url }) {
      imgStore.imgOriginal.set(index, url)
    },
    unauthorized: async () => {
      redirectionStore.redirectionToLogin()
    },
    notification: function (payload: { message: string; messageType: 'info' | 'warn' }): void {
      messageStore.message = payload.message
      messageStore.warn = payload.messageType === 'warn'
      messageStore.showMessage = true
    }
  })

  const messageHandler = (e: MessageEvent) => {
    handler(e.data as ReturnType<(typeof fromImgWorker)[keyof typeof fromImgWorker]>)
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
