import { handleDataWorkerReturn, removeHandleDataWorkerReturn } from '@/worker/fromDataWorker'
import { handleImgWorker, removeHandleImgWorkerReturn } from '@/worker/fromImgWorker'
import {
  processAbortPayload,
  processImagePayload,
  processSmallImagePayload,
  toImgWorker
} from '@/worker/workerApi'
import { defineStore } from 'pinia'
import { bindActionDispatch } from 'typesafe-agent-events'

type postToWorkerType = {
  processSmallImage: (payload: processSmallImagePayload) => void
  processImage: (payload: processImagePayload) => void
  processAbort: (payload: processAbortPayload) => void
}
export const useWorkerStore = defineStore({
  id: 'workerStore',
  state: (): {
    concurrencyNumber: number
    worker: null | Worker
    imgWorker: Worker[]
    postToWorkerList: postToWorkerType[] | undefined
  } => ({
    concurrencyNumber: Math.max(0, Math.floor(navigator.hardwareConcurrency / 2)),
    worker: null,
    imgWorker: [],
    postToWorkerList: undefined
  }),
  actions: {
    initializeWorker() {
      if (this.worker === null) {
        this.worker = new Worker(new URL('../worker/toDataWorker.ts', import.meta.url), {
          type: 'module'
        })
        handleDataWorkerReturn(this.worker)
      } else {
        console.error('There is already a worker')
      }

      if (this.imgWorker.length === 0) {
        this.postToWorkerList = []
        for (let i = 0; i <= this.concurrencyNumber; i++) {
          const worker = new Worker(new URL('../worker/toImgWorker.ts', import.meta.url), {
            type: 'module'
          })
          this.imgWorker.push(worker)
          const postToWorker = bindActionDispatch(toImgWorker, (action) =>
            worker.postMessage(action)
          )
          this.postToWorkerList.push(postToWorker)
        }
        this.imgWorker.forEach((worker) => {
          handleImgWorker(worker)
        })
      } else {
        console.error('There is already an imgWorker')
      }
    },
    terminateWorker() {
      if (this.worker !== null) {
        this.worker.terminate()
        removeHandleDataWorkerReturn(this.worker)
        this.worker = null
      } else {
        console.error('No Worker is Working')
      }
      if (this.imgWorker.length > 0) {
        this.imgWorker.forEach((worker) => {
          worker.terminate()
          removeHandleImgWorkerReturn(worker)
        })
        this.imgWorker = []
      } else {
        console.error('No Worker is Working')
      }
    }
  }
})
