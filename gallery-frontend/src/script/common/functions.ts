// functions.ts

import { thumbHashToDataURL } from 'thumbhash'
import { z } from 'zod'
import { DataBaseParse } from './schemas'
import { DataBase, AbstractData, Album } from './types'
import { RouteLocationNormalizedLoaded } from 'vue-router'
import { computed, ComputedRef } from 'vue'

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
    filename: databaseParse.alias[0]?.file.split('/').pop() || ''
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
): ComputedRef<'default' | 'all' | 'favorite' | 'archived' | 'trashed' | 'album'> {
  const currentPage = computed(() => {
    if (route.path.startsWith('/favorite')) {
      return 'favorite'
    } else if (route.path.startsWith('/archived')) {
      return 'archived'
    } else if (route.path.startsWith('/trashed')) {
      return 'trashed'
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
