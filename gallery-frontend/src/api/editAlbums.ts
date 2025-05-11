import { useMessageStore } from '@/store/messageStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import { IsolationId } from '@/type/types'
import axios from 'axios'
export async function editAlbums(
  indexArray: number[],
  addAlbumsArray: string[],
  removeAlbumsArray: string[],
  isolationId: IsolationId
) {
  const prefetchStore = usePrefetchStore(isolationId)
  const timestamp = prefetchStore.timestamp
  const messageStore = useMessageStore('mainId')

  if (timestamp === null) {
    messageStore.error('Cannot edit albums because timestamp is missing.')
    return
  }

  try {
    const response = await axios.put('/put/edit_album', {
      indexArray,
      addAlbumsArray,
      removeAlbumsArray,
      timestamp
    })

    if (response.status === 200) {
      messageStore.success('Successfully edited albums.')
    } else {
      messageStore.error(`Failed to edit albums. Server responded with status ${response.status}.`)
    }
  } catch (error) {
    messageStore.error('Failed to edit albums due to a network or server error.')
    console.error(error)
  }
}
