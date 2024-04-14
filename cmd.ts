import { getClipboard } from "@nikiv/ts-utils"

async function main() {
  const clipboard = getClipboard()
  console.log(clipboard, "clip")
}

main()
