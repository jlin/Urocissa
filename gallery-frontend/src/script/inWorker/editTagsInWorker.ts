import { usePrefetchStore } from '@/store/prefetchStore'
import { useTagStore } from '@/store/tagStore'
import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function editTagsInWorker(
  indexArray: number[],
  addTagsArray: string[],
  removeTagsArray: string[]
) {
  const workerStore = useWorkerStore()
  const prefetchStore = usePrefetchStore()
  const tagStore = useTagStore()

  tagStore.fetched = false

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }

  const dataWorker = workerStore.worker!
  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))
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
