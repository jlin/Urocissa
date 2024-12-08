import { RouteLocationNormalizedLoadedGeneric } from 'vue-router'

export function leaveRead(route: RouteLocationNormalizedLoadedGeneric) {
  return {
    name: `${route.meta.baseName}ViewPage`,
    params: {
      hash: route.params.hash
    },
    query: route.query
  }
}

export function intoViewPage(route: RouteLocationNormalizedLoadedGeneric, hashOrSubhash: string) {
  if (!route.meta.isReadPage) {
    return {
      name: `${route.meta.baseName}ViewPage`,
      params: { hash: hashOrSubhash },
      query: route.query
    }
  } else {
    return {
      name: `${route.meta.baseName}ReadViewPage`,
      params: { hash: route.meta.hash as string, subhash: hashOrSubhash },
      query: route.query
    }
  }
}

export function leaveViewPage(route: RouteLocationNormalizedLoadedGeneric) {
  if (!route.meta.isReadPage) {
    return { name: route.meta.baseName, query: route.query }
  } else {
    return {
      name: `${route.meta.baseName}ReadPage`,
      params: {
        hash: route.params.hash
      },
      query: route.query
    }
  }
}
