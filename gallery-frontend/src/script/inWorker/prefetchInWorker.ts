import { useWorkerStore } from '@/store/workerStore'
import { toDataWorker } from '@/worker/workerApi'
import { bindActionDispatch } from 'typesafe-agent-events'

export function prefetchInWorker(
  filterJsonString: string | null,
  priorityId: string | undefined = 'default',
  reverse: string | undefined,
  locate: string | null = null,
  isolationId: string
) {
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

  postToWorker.prefetch({
    filterJsonString: filterJsonString,
    priorityId: priorityId,
    reverse: reverse,
    locate: locate
  })
}
