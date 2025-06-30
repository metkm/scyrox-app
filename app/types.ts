import type { commands } from './constants'

export type CommandValues<T extends keyof typeof commands> = typeof commands[T]
