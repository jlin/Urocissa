import { usePrefetchStore } from '@/store/prefetchStore'
import { useTagStore } from '@/store/tagStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function editTagsInWorker(
  indexArray: number[],
  addTagsArray: string[],
  removeTagsArray: string[],
  isolationId: string
) {
  const workerStore = useWorkerStore(isolationId)
  const prefetchStore = usePrefetchStore(isolationId)
  const tagStore = useTagStore(isolationId)

  tagStore.fetched = false

  if (workerStore.worker === null) {
    workerStore.initializeWorker(isolationId)
  }

  const dataWorker = workerStore.worker
  const postToWorker = bindActionDispatch(toDataWorker, (action) => {
    if (dataWorker) {
      dataWorker.postMessage(action)
    }
  })
  const timestamp = prefetchStore.timestamp
  if (timestamp !== null) {
    const payload = {
      indexArray: [...indexArray],
      addTagsArray: [...addTagsArray],
      removeTagsArray: [...removeTagsArray],
      timestamp: timestamp
    }
    postToWorker.editTags(payload)
  }
}
