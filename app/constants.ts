/*  */
export const commands = {
  EncryptionData: 1,//下传加密沟通数据
  PCDriverStatus: 2,//下传驱动状态的命令（驱动是否处于窗口激活状态）
  DeviceOnLine: 3,//获取无线鼠标是否在线
  BatteryLevel: 4, //获取电池电量
  DongleEnterPair: 5,//设置无线Dongle进入配对状态
  GetPairState: 6,//获取无线Dongle配对结果
  WriteFlashData: 7,//设置eeprom内容
  ReadFlashData: 8,//获取eeprom内容
  ClearSetting: 9,//恢复出厂设置
  StatusChanged: 0x0A,//上报鼠标某些状态改变，如DPI等
  SetDeviceVidPid: 0x0B,//设置Dongle的USB VID/PID
  SetDeviceDescriptorString: 0x0C,//设置Dongle的USB设备描述字符串
  EnterUsbUpdateMode: 0x0D,//进入USB升级模式
  GetCurrentConfig: 0x0E,//获取当前配置
  SetCurrentConfig: 0x0F,//设置当前配置
  ReadCIDMID: 0x10,//获取鼠标CID/MID
  EnterMTKMode: 0x11,//设置无线Dongle进入EMI/MTK测试模式
  ReadVersionID: 0x12,//获取鼠标版本号
  Set4KDongleRGB: 0x14,//设置4K dongle RGB灯模式,dongle上有个rgb灯（不是在鼠标上）
  Get4KDongleRGBValue: 0x15,
  SetLongRangeMode: 0x16,
  GetLongRangeMode: 0x17,
  GetDongleVersion: 0x1D,//获取dongle版本
  MusicColorful: 0xB0,//音乐律动全彩
  MusicSingleColor: 0xB1,//音乐律动全键单色
  WriteKBCIdMID: 0xF0,//读取cid mid,cx53710专用
  ReadKBCIdMID: 0xF1,//读取cid mid,cx53710专用
} as const

export const mouseEepromAddr = {
  ReportRate: 0x00,//报告率
  MaxDPI: 0x02,//最大DPI档位
  CurrentDPI: 0x04,//当前DPI档位
  LOD: 0x0A,//LOD高度
  DPIValue: 0x0C,//第一档DPI值
  DPIColor: 0x20,//第一档DPI颜色
  DPIEffectMode: 0x4C,//DPI灯效
  DPIEffectBrightness: 0x4E,//DPI灯效亮度
  DPIEffectSpeed: 0x50,//DPI灯效亮度
  DPIEffectState: 0x52,//DPI灯效亮度
  LightEffectMode: 0xA0,//装饰灯
  LightEffectColor: 0xA1,//装饰灯
  LightEffectSpeed: 0xA4,//装饰灯
  LightEffectBrightness: 0xA5,//装饰灯
  LightEffectState: 0xA7,//装饰灯
  LightEffectTime: 0xAD,//装饰灯
  DebounceTime: 0xA9,//按钮消抖
  MotionSync: 0xAB,
  SleepTime: 0xAD,//休眠时间
  Angle: 0xAF,
  Ripple: 0xB1,
  MovingOffLight: 0xB3,
  PerformanceState: 0xB5,
  Performance: 0xB7,
  SensorMode: 0xB9,
  KeyFunction: 0x60,
  ShortcutKey: 0x0100,
  Macro: 0x0300,
} as const

export const reportId = 0x08

export type CommandValues<T extends keyof typeof commands> = typeof commands[T]
