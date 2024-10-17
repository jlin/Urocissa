import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function prefetchInWorker(
  filterJsonString: string | null,
  priorityId: string | undefined = 'default',
  reverse: string | undefined,
  locate: string | null = null
) {
  const workerStore = useWorkerStore()

  if (workerStore.worker === null) {
    workerStore.initializeWorker()
  }

  const dataWorker = workerStore.worker!
  const postToWorker = bindActionDispatch(toDataWorker, (action) => dataWorker.postMessage(action))

  postToWorker.prefetch({
    filterJsonString: filterJsonString,
    priorityId: priorityId,
    reverse: reverse,
    locate: locate
  })
}
