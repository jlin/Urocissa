import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'
import { IsolationId } from '../common/types'

export function setAsCoverInWorker(albumId: string, coverHash: string, isolationId: IsolationId) {
  const workerStore = useWorkerStore(isolationId)

  if (workerStore.worker === null) {
    workerStore.initializeWorker(isolationId)
  }

  const dataWorker = workerStore.worker

  const postToWorker = bindActionDispatch(toDataWorker, (action) => {
    if (dataWorker) {
      dataWorker.postMessage(action)
    }
  })

  const payload = {
    albumId: albumId,
    coverHash: coverHash
  }
  postToWorker.setAsAlbum(payload)
}
