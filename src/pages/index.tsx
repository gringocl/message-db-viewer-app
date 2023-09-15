import * as React from "react"
import { useRouter } from "next/router"
import { SuspenseOnClient } from "@components/suspense"
import { StreamList } from "@components/stream_list"
import { parseStreamNames } from "@lib/router"

export default function Home() {
  const router = useRouter()

  let streamNames = parseStreamNames(router.query.streamNames)

  return (
    <StreamList names={streamNames} />
  )
}
