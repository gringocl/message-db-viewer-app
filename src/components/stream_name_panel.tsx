import { CausationStreams } from "@components/stream_name_panel/causation_streams"
import { ActiveStreams } from "@components/stream_name_panel/active_streams"
import { SpecificStream } from "@components/stream_name_panel/specific_stream"
import * as Data from "@data"

export const StreamNamePanel = ({
  selectedStreamNames,
  streams,
}: {
  selectedStreamNames: string[]
  streams: Data.Stream[]
}) => {
  return (
    <div
      className="bg-gray-800 px-3 py-4 flex flex-col gap-8 shadow-lg"
      style={{ width: 320 }}
    >
      <CausationStreams
        selectedStreamNames={selectedStreamNames}
        streams={streams}
      />

      <ActiveStreams selectedStreamNames={selectedStreamNames} />

      <SpecificStream />
    </div>
  )
}
