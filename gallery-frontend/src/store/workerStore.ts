import { IsolationId } from '@type/types'
import { handleDataWorkerReturn, removeHandleDataWorkerReturn } from '@/worker/fromDataWorker'
import { handleImgWorker, removeHandleImgWorkerReturn } from '@/worker/fromImgWorker'
import {
  DeleteDataParams,
  EditAlbumsParams,
  EditTagsParams,
  FetchDataParams,
  FetchRowParams,
  ProcessAbortPayload,
  ProcessImagePayload,
  ProcessSmallImagePayload,
  toDataWorker,
  toImgWorker
} from '@/worker/workerApi'
import { defineStore } from 'pinia'
import { bindActionDispatch } from 'typesafe-agent-events'

interface PostToImgWorkerType {
  processSmallImage: (payload: ProcessSmallImagePayload) => void
  processImage: (payload: ProcessImagePayload) => void
  processAbort: (payload: ProcessAbortPayload) => void
}
interface PostToDataWorkerType {
  fetchData: (payload: FetchDataParams) => void
  fetchRow: (payload: FetchRowParams) => void
  editTags: (payload: EditTagsParams) => void
  editAlbums: (payload: EditAlbumsParams) => void
  deleteData: (payload: DeleteDataParams) => void
}

export const useWorkerStore = (isolationId: IsolationId) =>
  defineStore('workerStore' + isolationId, {
    state: (): {
      concurrencyNumber: number
      worker: null | Worker
      imgWorker: Worker[]
      postToDataWorker: PostToDataWorkerType | undefined
      postToImgWorkerList: PostToImgWorkerType[] | undefined
    } => ({
      concurrencyNumber: Math.max(Math.floor(navigator.hardwareConcurrency / 2), 1),
      worker: null,
      imgWorker: [],
      postToDataWorker: undefined,
      postToImgWorkerList: undefined
    }),
    actions: {
      initializeWorker(isolationId: IsolationId) {
        if (this.worker === null) {
          this.worker = new Worker(new URL('../worker/toDataWorker.ts', import.meta.url), {
            type: 'module'
          })
          handleDataWorkerReturn(this.worker, isolationId)
          this.postToDataWorker = bindActionDispatch(toDataWorker, (action) => {
            this.worker?.postMessage(action)
          })
        } else {
          console.error('There is already a worker')
        }

        if (this.imgWorker.length === 0) {
          this.postToImgWorkerList = []
          for (let i = 0; i <= this.concurrencyNumber; i++) {
            const worker = new Worker(new URL('../worker/toImgWorker.ts', import.meta.url), {
              type: 'module'
            })
            this.imgWorker.push(worker)
            const postToDataWorker = bindActionDispatch(toImgWorker, (action) => {
              worker.postMessage(action)
            })
            this.postToImgWorkerList.push(postToDataWorker)
          }
          this.imgWorker.forEach((worker) => {
            handleImgWorker(worker, isolationId)
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
  })()
