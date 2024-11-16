// functions.ts

import { thumbHashToDataURL } from 'thumbhash'
import { z } from 'zod'
import { DataBaseParse } from './schemas'
import { DataBase, AbstractData, Album } from './types'
import { RouteLocationNormalizedLoaded } from 'vue-router'
import { computed, ComputedRef } from 'vue'
import { useDataStore } from '@/store/dataStore'

/**
 * Creates a DataBase instance from parsed data and timestamp.
 */
export function createDataBase(
  databaseParse: z.infer<typeof DataBaseParse>,
  timestamp: number
): DataBase {
  const database: DataBase = {
    ...databaseParse,
    timestamp: timestamp,
    thumbhashUrl: thumbHashToDataURL(databaseParse.thumbhash),
    filename: databaseParse.alias[0]?.file.split('/').pop() ?? ''
  }
  return database
}

/**
 * Creates an AbstractData instance from DataBase or Album.
 */
export function createAbstractData(data: DataBase | Album): AbstractData {
  if ('hash' in data) {
    return { database: data }
  } else {
    return { album: data }
  }
}

export function useCurrentPage(
  route: RouteLocationNormalizedLoaded
): ComputedRef<'default' | 'all' | 'favorite' | 'archived' | 'trashed' | 'albums' | 'album'> {
  const currentPage = computed(() => {
    if (route.path.startsWith('/favorite')) {
      return 'favorite'
    } else if (route.path.startsWith('/archived')) {
      return 'archived'
    } else if (route.path.startsWith('/trashed')) {
      return 'trashed'
    } else if (route.path.startsWith('/albums')) {
      return 'albums'
    } else if (route.path.startsWith('/album')) {
      return 'album'
    } else if (route.path.startsWith('/all')) {
      return 'all'
    } else {
      return 'default'
    }
  })
  return currentPage
}

export function dater(timestamp: number): string {
  const locale = navigator.language
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }).format(timestamp)
}

export function getIsolationIdByRoute(route: RouteLocationNormalizedLoaded) {
  const isolationId = route.meta.isReadPage ? 'subId' : 'mainId'
  return isolationId
}

export function getHashIndexDataFromRoute(route: RouteLocationNormalizedLoaded) {
  const isolationId = route.meta.isReadPage ? 'subId' : 'mainId'
  const storeData = useDataStore(isolationId)

  let hash: string

  if (isolationId === 'mainId' && typeof route.params.hash === 'string') {
    hash = route.params.hash
  } else if (isolationId === 'subId' && typeof route.params.subhash === 'string') {
    hash = route.params.subhash
  } else {
    return undefined
  }

  const index = storeData.hashMapData.get(hash)

  if (index === undefined) {
    return undefined
  }

  const data = storeData.data.get(index)

  if (data === undefined) {
    return undefined
  }

  return { hash: hash, index: index, data: data }
}

export function getArrayValue<T>(array: T[], index: number): T {
  const result = array[index]
  if (result === undefined) {
    throw new RangeError(
      `Index ${index.toString()} is out of bounds for array of length ${array.length.toString()}`
    )
  } else {
    return result
  }
}
