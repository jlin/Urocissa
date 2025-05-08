import axios from 'axios'
import { useMessageStore } from '@/store/messageStore'
import { useAlbumStore } from '@/store/albumStore'
import { Album, IsolationId } from '@type/types'
import { useDataStore } from '@/store/dataStore'
import { usePrefetchStore } from '@/store/prefetchStore'

export async function createNonEmptyAlbum(
  elementsIndex: number[],
  isolationId: IsolationId
): Promise<string | undefined> {
  const messageStore = useMessageStore('mainId')
  const albumStore = useAlbumStore('mainId')
  const prefetchStore = usePrefetchStore(isolationId)
  try {
    const createNonEmptyAlbumData = {
      title: null,
      elementsIndex: elementsIndex,
      timestamp: prefetchStore.timestamp
    }

    const response = await axios.post<string>(
      '/post/create_non_empty_album',
      createNonEmptyAlbumData,
      {
        headers: {
          'Content-Type': 'application/json'
        }
      }
    )

    messageStore.success('Album created successfully.')

    const newAlbumId = response.data
    await albumStore.fetchAlbums()
    return newAlbumId
  } catch (error) {
    console.error('Error creating album:', error)
    messageStore.error('Failed to create album.')
  }
}

export async function createEmptyAlbum(): Promise<string | undefined> {
  const messageStore = useMessageStore('mainId')
  const albumStore = useAlbumStore('mainId')
  try {
    const response = await axios.post<string>('/post/create_empty_album', {
      headers: {
        'Content-Type': 'application/json'
      }
    })

    messageStore.success('Album created successfully.')

    const newAlbumId = response.data
    await albumStore.fetchAlbums()
    return newAlbumId
  } catch (error) {
    console.error('Error creating album:', error)
    messageStore.error('Failed to create album.')
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
