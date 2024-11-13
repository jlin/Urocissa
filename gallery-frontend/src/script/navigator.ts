import { RouteLocationNormalizedLoadedGeneric } from 'vue-router'

export function leaveRead(route: RouteLocationNormalizedLoadedGeneric) {
  const suffix = 'ReadPage'
  const fullName = route.name! as string
  const name = fullName.replace(suffix, '')

  return {
    name: `${name}ViewPage`,
    params: {
      hash: route.params.hash
    },
    query: route.query
  }
}

export function intoViewPage(route: RouteLocationNormalizedLoadedGeneric, hashOrSubhash: string) {
  if (!route.meta.isReadPage) {
    const name = route.name! as string
    return { name: `${name}ViewPage`, params: { hash: hashOrSubhash }, query: route.query }
  } else {
    const suffix = 'ReadPage'
    const fullName = route.name! as string
    const name = fullName.replace(suffix, '')
    const hash = route.meta.hash as string

    return {
      name: `${name}ReadViewPage`,
      params: { hash: hash, subhash: hashOrSubhash },
      query: route.query
    }
  }
}

export function leaveViewPage(route: RouteLocationNormalizedLoadedGeneric) {
  if (!route.meta.isReadPage) {
    // this is in the main page, so leave to
    const suffix = 'ViewPage'
    const fullName = route.name! as string

    const name = fullName.replace(suffix, '')
    return { name: name, query: route.query }
  } else {
    const suffix = 'ReadViewPage'
    const fullName = route.name! as string

    const name = fullName.replace(suffix, '')
    return {
      name: `${name}ReadPage`,
      params: {
        hash: route.params.hash
      },
      query: route.query
    }
  }
}
