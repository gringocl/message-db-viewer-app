import { StreamMessage } from "@components/stream_list/stream_message"
import * as Data from "@data"
import { removeStream } from "@lib/router"
import { useRouter } from "next/router"

const EmptyStream = () => (
  <div className="italic border border-dashed px-8 py-8 inline-block">Empty</div>
)

export const Stream = ({
  stream,
  selectedMessage,
  selectMessage,
}: {
  stream: Data.Stream
  selectedMessage: Data.Message | null
  selectMessage: (message: Data.Message | null) => void
}) => {
  const { messages } = stream

  const router = useRouter()

  return (
    <div className="pl-2">
      <div className="z-10 absolute bg-gray-50 bg-opacity-75 flex -ml-8 items-baseline">
        <button
          className="text-xs mr-2 text-gray-400 hover:(bg-red-400 text-white) transition duration-150 ease-in-out rounded-full "
          style={{ width: 25, height: 25 }}
          onClick={() => removeStream(router, stream.name)}
        >
          ✕
        </button>
        <div className="truncate text-sm">{stream.name}</div>
      </div>
      <div className="z-0 mt-8">
        {messages.length == 0 ? (
          <EmptyStream />
        ) : (
          <div
            className="grid grid-flow-col auto-cols-max gap-2 px-2 py-1 rounded bg-gradient-to-b from-gray-200 via-gray-300 to-gray-300 shadow-md"
            style={{ width: "max-content" }}
          >
            {messages.map((message) => {
              const selected =
                message.global_position == selectedMessage?.global_position
              return (
                <StreamMessage
                  key={message.id}
                  message={message}
                  selected={selected}
                  selectMessage={selectMessage}
                />
              )
            })}
          </div>
        )}
      </div>
    </div>
  )
}
