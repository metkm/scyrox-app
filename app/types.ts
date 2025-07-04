import type { commands } from './constants'

export type CommandValues<T extends keyof typeof commands> = typeof commands[T]

export interface DpiValue {
  value: number
  color: string
}

export interface MouseConfig {
  report_rate: number
  dpi_values: DpiValue[]
  current_dpi_index: number
  max_dpi_index: number
  sleep_time: number
}

export interface Battery {
  charging: boolean
  level: number
}
