# Tauri stale IPC response crash

1. Clone this repo
2. pnpm install
3. pnpm tauri dev
4. When the app starts, it kicks off a tauri command that will finish in 20 seconds.
5. Within that 20 seconds, modify main.ts and save (causing a reload)
6. When the 20 second command finishes, the app will crash.
