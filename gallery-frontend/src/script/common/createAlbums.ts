import { useModalStore } from '@/store/modalStore'
import axios from 'axios'
import { useMessageStore } from '@/store/messageStore'
import { useAlbumStore } from '@/store/albumStore'
import { Album, IsolationId } from './types'
import { useDataStore } from '@/store/dataStore'
import { usePrefetchStore } from '@/store/prefetchStore'

export async function createAlbum(
  elementsIndex: number[],
  isolationId: IsolationId
): Promise<string | undefined> {
  const modalStore = useModalStore('mainId')
  const messageStore = useMessageStore('mainId')
  const albumStore = useAlbumStore('mainId')
  const prefetchStore = usePrefetchStore(isolationId)
  try {
    const createAlbumData = {
      title: null,
      elementsIndex: elementsIndex,
      timestamp: prefetchStore.timestamp
    }

    const response = await axios.post<string>('/post/create_album', createAlbumData, {
      headers: {
        'Content-Type': 'application/json'
      }
    })

    messageStore.showInfo('Album created successfully.')

    modalStore.showCreateAlbumsModal = false
    const newAlbumId = response.data
    await albumStore.fetchAlbums()
    return newAlbumId
  } catch (error) {
    console.error('Error creating album:', error)
    messageStore.showWarn('Failed to create album.')
  }
}

export async function editTitle(album: Album, titleModelValue: string) {
  const albumStore = useAlbumStore('mainId')
  const dataStore = useDataStore('mainId')

  if ((album.title ?? '') !== titleModelValue) {
    const id = album.id
    const title = titleModelValue === '' ? null : titleModelValue
    await axios.post('/post/set_album_title', {
      albumId: id,
      title: title
    })
    const albumInfo = albumStore.albums.get(id)

    const index = dataStore.hashMapData.get(album.id)
    if (index !== undefined) {
      const album = dataStore.data.get(index)?.album

      if (albumInfo && album) {
        albumInfo.albumName = title
        albumInfo.displayName = albumInfo.albumName ?? 'Untitled'
        album.title = title
      } else {
        console.error(`Cannot find album with id ${id}`)
      }
    }
  }
}
