import { addableStreamNames } from "@lib/stream_names"
import React from "react"
import useSWR from "swr"
import { StreamNameList } from "@components/stream_name_panel/stream_name_list"
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

const fetchJSON = (url: string) => fetch(url).then((res) => res.json())

export const ActiveStreams = ({
  selectedStreamNames,
}: {
  selectedStreamNames: string[]
}) => {
  let [activeStreamNames, setActiveStreamNames] = useState(["some-stream"])

  useEffect(() => {
    invoke<string>('active_stream_names', { })
      .then((stream_names) => setActiveStreamNames(stream_names))
      .catch(console.error)
  }, [])

  if (!activeStreamNames) return null

  activeStreamNames = addableStreamNames(activeStreamNames, selectedStreamNames)

  if (activeStreamNames.length === 0) return null

  return (
    <StreamNameList title="Active Streams" streamNames={activeStreamNames} />
  )
}
