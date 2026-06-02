<template>
  <div id="app" :class="{ light: !isDarkMode }" @contextmenu.prevent>
    <header @mousedown="handleHeaderMouseDown">
      <div class="header-left">
        <svg class="app-logo" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
          <rect x="0" y="0" width="512" height="512" rx="113" fill="#2C3E50"/>
          <path d="M 144 112 L 416 112 L 416 144 L 128 144 L 128 368 L 416 368 L 416 400 L 96 400 Z" fill="white"/>
          <path d="M 192 160 L 416 160 L 416 192 L 224 192 L 224 256 L 288 256 L 352 320 L 288 384 L 224 384 L 224 320 L 192 320 Z" fill="white"/>
          <circle cx="464" cy="464" r="12" fill="#F7B731"/>
        </svg>
        <h1>{{ t('app.title') }}</h1>
      </div>
      <div class="header-right">
        <button @click="showMenu = !showMenu" class="menu-btn" :title="t('menu.title')">
          <svg class="hamburger-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            有权  <line x1="3" y1="5" x2="21" y2="5"/>
            <line x1="3" y1="12" x2="21" y2="12"/>
            <line x1="3" y1="19" x2="21" y2="19"/>
          </svg>
        </button>
        <div v-if="showMenu" class="menu-dropdown" @click.stop>
          <div class="menu-content">
            <button @click="showSettings = true; showMenu = false" class="menu-item">
              <span>{{ t('settings.title') }}</span>
            </button>
            <button @click="showAbout = true; showMenu = false" class="menu-item">
              <span>{{ t('about.title') }}</span>
            </button>
            <div class="menu-divider"></div>
            <div class="menu-help-section">
              <div class="menu-help-title">{{ t('help.shortcuts') }}</div>
              <div class="menu-help-item">
                <span class="menu-help-key">{{ t('help.drag') }}</span>
                <span>{{ t('help.move') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">{{ toggleShortcut }}</span>
                <span>{{ t('help.toggle') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">J/K/L</span>
                <span>{{ t('help.paste') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">↑/↓</span>
                <span>{{ t('help.nav') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">Enter</span>
                <span>{{ t('help.enter') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">ESC</span>
                <span>{{ t('help.esc') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">Delete</span>
                <span>{{ t('help.delete') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">F</span>
                <span>{{ t('help.favorite') }}</span>
              </div>
              <div class="menu-help-item">
                <span class="menu-help-key">Tab</span>
                <span>{{ t('help.tab') }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- 设置对话框 -->
    <div v-if="showSettings" class="modal-overlay" @click.self="showSettings = false">
      <div class="modal">
        <div class="modal-header">
          <h2>{{ t('settings.title') }}</h2>
          <button @click="showSettings = false" class="modal-close">✕</button>
        </div>
        <div class="modal-body">
          <div class="setting-group">
            <h3>{{ t('settings.appearance') }}</h3>
            <div class="setting-item">
              <label>{{ t('settings.darkMode') }}</label>
              <button @click="isDarkMode = !isDarkMode" class="toggle-btn" :class="{ active: isDarkMode }">
                <span class="toggle-slider"></span>
              </button>
            </div>
            <div class="setting-item">
              <label>{{ t('settings.language') }}</label>
              <select v-model="language" class="setting-input">
                <option value="zh">{{ t('language.zh') }}</option>
                <option value="en">{{ t('language.en') }}</option>
              </select>
            </div>
          </div>
          <div class="setting-group">
            <h3>{{ t('settings.shortcuts') }}</h3>
            <div class="setting-item">
              <label>{{ t('settings.toggleWindow') }}</label>
              <input
                type="text"
                v-model="toggleShortcut"
                @keydown.prevent="captureShortcut"
                class="setting-input shortcut-input"
                :placeholder="t('settings.pressKey')"
                readonly
              />
            </div>
            <div class="setting-item">
              <label>{{ t('settings.paste123') }}</label>
              <span class="shortcut-hint">J / K / L</span>
            </div>
          </div>
          <div class="setting-group">
            <h3>{{ t('settings.dataManagement') }}</h3>
            <div class="setting-item">
              <label>{{ t('settings.maxHistory') }}</label>
              <input type="number" v-model.number="maxHistory" min="10" max="1000" class="setting-input">
            </div>
            <div class="setting-item">
              <label>{{ t('settings.enableImages') }}</label>
              <button @click="enableImages = !enableImages" class="toggle-btn" :class="{ active: enableImages }">
                <span class="toggle-slider"></span>
              </button>
            </div>
            <div class="setting-item">
              <label>{{ t('settings.autostart') }}</label>
              <button @click="enableAutostart = !enableAutostart" class="toggle-btn" :class="{ active: enableAutostart }">
                <span class="toggle-slider"></span>
              </button>
            </div>
            <div class="setting-item">
              <label>{{ t('settings.hideAfterPaste') }}</label>
              <button @click="hideAfterPaste = !hideAfterPaste" class="toggle-btn" :class="{ active: hideAfterPaste }">
                <span class="toggle-slider"></span>
              </button>
            </div>
          </div>
          <div class="setting-group">
            <h3>{{ t('settings.actions') }}</h3>
            <button @click="clearAll" class="btn-danger">{{ t('settings.clearAll') }}</button>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="saveSettings" class="btn-primary">{{ t('settings.save') }}</button>
        </div>
      </div>
    </div>

    <!-- 关于对话框 -->
    <div v-if="showAbout" class="modal-overlay" @click.self="showAbout = false">
      <div class="modal">
        <div class="modal-header">
          <h2>{{ t('about.title') }}</h2>
          <button @click="showAbout = false" class="modal-close">✕</button>
        </div>
        <div class="modal-body about-content">
          <div class="about-icon">📋</div>
          <h3 class="app-name">{{ t('about.appName') }}</h3>
          <p class="app-version">{{ t('about.version') }}</p>
          <p class="app-author">{{ t('about.author') }}</p>
          <div class="app-description">
            <p>{{ t('about.description1') }}</p>
            <p>{{ t('about.description2') }}</p>
          </div>
          <div class="app-features">
            <h4>{{ t('about.features') }}</h4>
            <ul>
              <li>{{ t('about.feature1') }}</li>
              <li>{{ t('about.feature2') }}</li>
              <li>{{ t('about.feature3') }}</li>
              <li>{{ t('about.feature4') }}</li>
              <li>{{ t('about.feature5') }}</li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <main>
      <div class="tabs">
        <button
          :class="{ active: activeTab === 'history' }"
          @click="activeTab = 'history'"
        >
          {{ t('tabs.history') }} ({{ historyItems.length }})
        </button>
        <button
          :class="{ active: activeTab === 'favorite' }"
          @click="activeTab = 'favorite'"
        >
          {{ t('tabs.favorite') }} ({{ favoriteItems.length }})
        </button>
      </div>

      <div class="content">
        <div v-if="currentList.length > 0" class="list">
          <div
            v-for="(item, index) in currentList"
            :key="item.id"
            class="item"
            :class="{ selected: selectedIndex === index, pinned: item.is_favorite }"
            @click="pasteItem(item.id)"
            @mouseenter="selectedIndex = index"
          >
            <div class="item-number">
              <span v-if="index < 3" class="hotkey-badge">{{ ['J', 'K', 'L'][index] }}</span>
              <span v-else>{{ index + 1 }}</span>
            </div>
            <div class="item-content-wrapper">
              <div class="item-header">
                <span v-if="item.data_type === 'text'" class="item-content">{{ truncate(item.content, 100) }}</span>
                <span v-else-if="item.data_type === 'image'" class="item-content">
                  [图片] {{ getImageData(item.content)?.width }}x{{ getImageData(item.content)?.height }}
                </span>
                <div class="item-actions">
                  <button @click.stop="toggleFavorite(item.id)" class="btn-icon" :title="item.is_favorite ? '取消收藏' : '收藏'">
                    {{ item.is_favorite ? '⭐' : '☆' }}
                  </button>
                  <button @click.stop="deleteItem(item.id)" class="btn-icon delete" :title="t('help.delete')">✕</button>
                </div>
              </div>
              <img v-if="item.data_type === 'image' && getImagePreviewData(item.content_preview)" :src="getImagePreviewData(item.content_preview)!" class="item-preview-image" alt="预览" />
              <div class="item-meta">
                <span class="item-type">{{ item.data_type }}</span>
                <span class="time">{{ formatTime(item.create_time) }}</span>
              </div>
            </div>
          </div>
        </div>

        <div v-if="currentList.length === 0" class="empty">
          <div class="empty-icon">{{ activeTab === 'history' ? '📋' : '⭐' }}</div>
          <div class="empty-text">{{ activeTab === 'history' ? t('empty.history') : t('empty.favorite') }}</div>
          <div class="empty-hint">{{ activeTab === 'history' ? t('empty.hint.history') : t('empty.hint.favorite') }}</div>
        </div>
      </div>

      <div class="status" :class="{ visible: statusMessage }">
        {{ statusMessage }}
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-shell'
import zhTranslations from './translations/zh.json'
import enTranslations from './translations/en.json'

interface ClipboardItem {
  id: number
  content: string
  content_preview?: string
  data_type: string
  is_favorite: boolean
  create_time: number
}

// 多语言翻译
const language = ref<'zh' | 'en'>('zh')

// 深度获取嵌套对象的值
function getNestedValue(obj: any, path: string): string {
  return path.split('.').reduce((current, key) => current?.[key], obj) || path
}

const translations = computed(() => {
  return language.value === 'zh' ? zhTranslations : enTranslations
})

const t = (key: string) => {
  return getNestedValue(translations.value, key)
}

const allItems = ref<ClipboardItem[]>([])
const activeTab = ref<'history' | 'favorite'>('history')
const statusMessage = ref('')
const selectedIndex = ref(-1)
const isDarkMode = ref(true)
const showHelp = ref(false)
const showSettings = ref(false)
const showAbout = ref(false)
const showMenu = ref(false)
const maxHistory = ref(100)
const enableImages = ref(true)
const enableAutostart = ref(false)
const hideAfterPaste = ref(false)
const toggleShortcut = ref('Alt+C')
const hasPermission = ref(true)

const appWindow = getCurrentWindow()
let statusTimeout: ReturnType<typeof setTimeout> | null = null
let blurTimeout: ReturnType<typeof setTimeout> | null = null

// 捕获快捷键
function captureShortcut(event: KeyboardEvent) {
  const keys: string[] = []

  if (event.ctrlKey || event.metaKey) keys.push('Ctrl')
  if (event.altKey) keys.push('Alt')
  if (event.shiftKey) keys.push('Shift')

  const code = event.code
  const key = event.key

  // 使用 code 来获取物理键码，避免 Option 键导致的问题
  if (code !== 'ControlLeft' && code !== 'ControlRight' &&
      code !== 'AltLeft' && code !== 'AltRight' &&
      code !== 'ShiftLeft' && code !== 'ShiftRight' &&
      code !== 'MetaLeft' && code !== 'MetaRight') {
    // 对于字母键，使用 code 提取键名 (如 'KeyV' -> 'V')
    let keyChar = ''
    if (code.startsWith('Key')) {
      keyChar = code.substring(3)
    } else if (code.startsWith('Digit')) {
      keyChar = code.substring(5)
    } else {
      // 其他键使用 key（但要排除特殊字符）
      if (key.length === 1 && /[a-zA-Z0-9]/.test(key)) {
        keyChar = key.toUpperCase()
      } else {
        // F1-F12, Space, Enter 等功能键
        const functionalKeys = {
          'Space': 'Space',
          'Enter': 'Enter',
          'Backspace': 'Backspace',
          'Tab': 'Tab',
          'Escape': 'Escape',
          'Insert': 'Insert',
          'Delete': 'Delete',
          'Home': 'Home',
          'End': 'End',
          'PageUp': 'PageUp',
          'PageDown': 'PageDown',
          'ArrowUp': 'ArrowUp',
          'ArrowDown': 'ArrowDown',
          'ArrowLeft': 'ArrowLeft',
          'ArrowRight': 'ArrowRight'
        }
        keyChar = functionalKeys[code] || code
      }
    }
    if (keyChar) {
      keys.push(keyChar.toUpperCase())
      toggleShortcut.value = keys.join('+')
    }
  }
}

function showStatus(message: string, duration = 1500) {
  statusMessage.value = message
  if (statusTimeout) clearTimeout(statusTimeout)
  statusTimeout = setTimeout(() => {
    statusMessage.value = ''
  }, duration)
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()

  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)

  if (seconds < 60) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  return `${Math.floor(hours / 24)}天前`
}

function truncate(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
}

async function pasteItem(id: number) {
  // 每次粘贴前检查权限（macOS）
  if (navigator.platform.includes('Mac')) {
    try {
      const permitted = await invoke('check_accessibility_permission') as boolean
      if (!permitted) {
        hasPermission.value = false
        showStatus(t('status.permissionNeeded'), 5000)
        setTimeout(() => {
          showStatus(t('status.permissionSteps'), 5000)
        }, 5000)
        return
      }
      hasPermission.value = permitted
    } catch (error) {
      console.log('检查权限失败:', error)
    }
  }

  try {
    console.log('粘贴项目 ID:', id)
    const result = await invoke('paste_content', { id, hideWindow: hideAfterPaste.value })
    console.log('粘贴成功:', result)
  } catch (error) {
    console.error('粘贴失败:', error)
    const errorMsg = String(error)

    // 检查是否是权限错误
    if (errorMsg.includes('1002') || errorMsg.includes('不允许发送按键') || errorMsg.includes('accessibility') || errorMsg.includes('automation')) {
      showStatus(t('status.permissionNeeded'), 5000)
      setTimeout(() => {
        showStatus(t('status.permissionSteps'), 5000)
      }, 5000)
    } else {
      showStatus(t('status.pasteFailed') + ': ' + errorMsg)
    }
  }
}

async function toggleFavorite(id: number) {
  const item = allItems.value.find(i => i.id === id)
  if (item) {
    item.is_favorite = !item.is_favorite
    showStatus(item.is_favorite ? t('status.favorited') : t('status.unfavorited'))
  }
}

async function deleteItem(id: number) {
  try {
    await invoke('delete_by_id', { id })
    allItems.value = allItems.value.filter(i => i.id !== id)
    selectedIndex.value = -1
    showStatus(t('status.deleted'))
  } catch (error) {
    showStatus(t('status.deleteFailed'))
  }
}

async function clearAll() {
  try {
    await invoke('clear_all')
    allItems.value = []
    showStatus(t('status.cleared'))
  } catch (error) {
    showStatus(t('status.clearFailed'))
  }
}

function toggleTheme() {
  isDarkMode.value = !isDarkMode.value
  showStatus(isDarkMode.value ? t('status.darkMode') : t('status.lightMode'))
}

async function saveSettings() {
  // 保存到 localStorage
  localStorage.setItem('clipboard-settings', JSON.stringify({
    isDarkMode: isDarkMode.value,
    maxHistory: maxHistory.value,
    enableImages: enableImages.value,
    enableAutostart: enableAutostart.value,
    hideAfterPaste: hideAfterPaste.value,
    language: language.value,
    toggleShortcut: toggleShortcut.value
  }))

  // 保存自启动设置
  try {
    await invoke('set_autostart', { enable: enableAutostart.value })
    console.log('✅ 自启动设置已保存:', enableAutostart.value)
  } catch (error) {
    console.error('❌ 保存自启动设置失败:', error)
  }

  // 注册新的快捷键
  try {
    await invoke('register_toggle_shortcut', { shortcut: toggleShortcut.value })
  } catch (error) {
    console.error('注册快捷键失败:', error)
  }

  // 保存最大记录数到后端
  try {
    await invoke('set_max_history', { limit: maxHistory.value })
    console.log('✅ 最大记录数已保存:', maxHistory.value)
  } catch (error) {
    console.error('❌ 保存最大记录数失败:', error)
  }

  showSettings.value = false
  showStatus(t('status.settingsSaved'))
}

async function loadSettings() {
  const saved = localStorage.getItem('clipboard-settings')
  if (saved) {
    try {
      const settings = JSON.parse(saved)
      isDarkMode.value = settings.isDarkMode ?? true
      maxHistory.value = settings.maxHistory ?? 100
      enableImages.value = settings.enableImages ?? true
      enableAutostart.value = settings.enableAutostart ?? false
      hideAfterPaste.value = settings.hideAfterPaste ?? false
      language.value = settings.language ?? 'zh'
      toggleShortcut.value = settings.toggleShortcut ?? 'Alt+C'
    } catch (e) {
      console.error('加载设置失败:', e)
    }
  }

  // 获取当前自启动状态
  try {
    const isEnabled = await invoke('is_autostart_enabled') as boolean
    enableAutostart.value = isEnabled
    console.log('✅ 自启动状态:', isEnabled)
  } catch (error) {
    console.error('❌ 获取自启动状态失败:', error)
  }
}

// 保存窗口状态
async function saveWindowState() {
  try {
    const position = await appWindow.outerPosition()
    const size = await appWindow.innerSize()
    const state = {
      x: position.x,
      y: position.y,
      width: size.width,
      height: size.height
    }
    localStorage.setItem('window-state', JSON.stringify(state))
  } catch (e) {
    console.error('保存窗口状态失败:', e)
  }
}

// 恢复窗口状态
async function restoreWindowState() {
  try {
    const saved = localStorage.getItem('window-state')
    if (saved) {
      const state = JSON.parse(saved)
      await appWindow.setPosition(new PhysicalPosition(state.x, state.y))
      await appWindow.setSize(new PhysicalSize(state.width, state.height))
    }
  } catch (e) {
    console.error('恢复窗口状态失败:', e)
  }
}

// 隐藏窗口
async function hideWindow() {
  console.log('🚪 开始隐藏窗口...')
  await saveWindowState()
  await appWindow.hide()
  console.log('✅ 窗口已隐藏')
}

// 自动隐藏窗口
async function hideWindowDelayed() {
  if (blurTimeout) clearTimeout(blurTimeout)

  // macOS 上无延迟，其他平台延迟 200ms
  const delay = navigator.userAgent.includes('Mac') ? 0 : 200

  blurTimeout = setTimeout(async () => {
    const isFocused = await appWindow.isFocused()
    const isVisible = await appWindow.isVisible()

    if (!isFocused && isVisible) {
      // 保存窗口状态
      await saveWindowState()
      // 隐藏窗口
      await appWindow.hide()
    }
  }, delay)
}

// 设置焦点事件监听
function setupFocusListeners() {
  // 监听获得焦点事件
  listen('tauri://focus', () => {
    if (blurTimeout) clearTimeout(blurTimeout)
  })

  // 监听失去焦点事件 - 参考 EcoPaste，只保存状态，不自动隐藏窗口
  listen('tauri://blur', async () => {
    // 只保存窗口状态，不隐藏窗口（参考 EcoPaste）
    await saveWindowState()
  })
}

// 计算属性：历史记录
const historyItems = computed(() => {
  return allItems.value.filter(item => !item.is_favorite)
})

// 计算属性：收藏项
const favoriteItems = computed(() => {
  return allItems.value.filter(item => item.is_favorite)
})

// 当前显示的列表
const currentList = computed(() => {
  return activeTab.value === 'history' ? historyItems.value : favoriteItems.value
})

function getImageData(content: string) {
  try {
    const data = JSON.parse(content)
    return {
      src: `data:image/jpeg;base64,${data.base64}`,
      width: data.width,
      height: data.height
    }
  } catch {
    return null
  }
}

function getImagePreviewData(preview: string | undefined) {
  if (!preview) return null
  try {
    const data = JSON.parse(preview)
    return `data:image/jpeg;base64,${data.base64}`
  } catch {
    return null
  }
}

// 手动启动窗口拖拽
async function handleHeaderMouseDown(event: MouseEvent) {
  // 只响应左键且不在按钮上
  if (event.button !== 0 || (event.target as HTMLElement).closest('button')) {
    return
  }

  console.log('🖱️ 尝试启动窗口拖拽')

  try {
    await invoke('start_dragging')
    console.log('✅ 拖拽启动成功')
  } catch (error) {
    console.error('❌ 拖拽启动失败:', error)
  }
}

// 键盘导航（仅在窗口内有效）
async function handleKeyDown(event: KeyboardEvent) {
  console.log('⌨️ handleKeyDown 被调用:', event.key, event.code, event.metaKey, event.altKey, event.ctrlKey, event.shiftKey)

  // macOS 开发工具快捷键: Cmd+Option+I
  if (event.metaKey && event.altKey && event.key.toLowerCase() === 'i') {
    event.preventDefault()
    const window = getCurrentWindow()
    window.setDecorations(true).then(() => {
      // 尝试打开开发者工具
      window.setFocus()
      console.log('🔧 开发工具快捷键已触发')
    })
    return
  }

  const current = currentList.value

  // 处理 J/K/L 快速粘贴
  switch (event.key.toLowerCase()) {
    case 'j':
      console.log('✅ J 键被按下')
      if (current.length > 0) {
        event.preventDefault()
        pasteItem(current[0].id)
      }
      return
    case 'k':
      if (current.length > 1) {
        event.preventDefault()
        pasteItem(current[1].id)
      }
      return
    case 'l':
      if (current.length > 2) {
        event.preventDefault()
        pasteItem(current[2].id)
      }
      return
  }

  switch (event.key) {
    case 'Escape':
      console.log('🎯 检测到 ESC 键')
      // 不阻止默认行为，让 ESC 可以正常工作
      hideWindow()
      break

    case 'ArrowDown':
      event.preventDefault()
      if (current.length > 0) {
        selectedIndex.value = Math.min(selectedIndex.value + 1, current.length - 1)
      }
      break

    case 'ArrowUp':
      event.preventDefault()
      selectedIndex.value = Math.max(selectedIndex.value - 1, 0)
      break

    case 'Enter':
      event.preventDefault()
      if (selectedIndex.value >= 0 && current[selectedIndex.value]) {
        pasteItem(current[selectedIndex.value].id)
      }
      break

    case 'Delete':
    case 'Backspace':
      event.preventDefault()
      if (selectedIndex.value >= 0 && current[selectedIndex.value]) {
        deleteItem(current[selectedIndex.value].id)
      }
      break

    case 'f':
    case 'F':
      event.preventDefault()
      if (selectedIndex.value >= 0 && current[selectedIndex.value]) {
        toggleFavorite(current[selectedIndex.value].id)
      }
      break

    case 'Tab':
      event.preventDefault()
      activeTab.value = activeTab.value === 'history' ? 'favorite' : 'history'
      selectedIndex.value = -1
      break
  }
}

onMounted(async () => {
  console.log('🚀 应用已挂载')
  console.log('🖥️ 平台:', navigator.platform)

  // macOS 上请求辅助功能权限（用于粘贴功能）
  if (navigator.platform.includes('Mac')) {
    try {
      // 使用 Tauri 命令请求权限（在 Rust 端实现）
      console.log('🔐 检查 macOS 辅助功能权限...')
      try {
        const permitted = await invoke('check_accessibility_permission') as boolean
        hasPermission.value = permitted
        console.log(`🔐 辅助功能权限: ${permitted ? '✅ 已授权' : '❌ 未授权'}`)

        if (!permitted) {
          console.log('🔐 正在请求辅助功能权限...')
          await invoke('request_accessibility_permission')
          // 请求后再次检查
          const permittedAfterRequest = await invoke('check_accessibility_permission') as boolean
          hasPermission.value = permittedAfterRequest
          
          if (!permittedAfterRequest) {
            console.log('🔐 用户未授予权限，将在首次粘贴时提示')
            // 显示权限提示
            showStatus(t('status.permissionDenied'), 4000)
            setTimeout(() => {
              showStatus(t('status.permissionHelp'), 4000)
            }, 4000)
          } else {
            console.log('✅ 辅助功能权限已授予')
          }
        } else {
          console.log('✅ 辅助功能权限已授予')
        }
      } catch (permError) {
        console.log('🔐 权限命令不可用，跳过权限检查')
      }
    } catch (error) {
      console.log('🔐 权限检查失败:', error)
    }
  }

  // 测试拖拽区域
  const dragRegions = document.querySelectorAll('[data-tauri-drag-region]')
  console.log('🎯 找到拖拽区域:', dragRegions.length, '个')
  dragRegions.forEach((region, index) => {
    console.log(`  拖拽区域 ${index + 1}:`, region)
  })

  // 加载设置
  loadSettings()

  // 注册快捷键
  try {
    await invoke('register_toggle_shortcut', { shortcut: toggleShortcut.value })
    console.log('✅ 快捷键已注册:', toggleShortcut.value)
  } catch (error) {
    console.error('❌ 注册快捷键失败:', error)
  }

  // 恢复窗口状态
  await restoreWindowState()

  // 设置焦点事件监听
  setupFocusListeners()

  // 点击外部关闭菜单
  document.addEventListener('click', (e) => {
    const menuBtn = document.querySelector('.menu-btn')
    const menuDropdown = document.querySelector('.menu-dropdown')
    if (menuDropdown && menuBtn && !menuBtn.contains(e.target as Node) && !menuDropdown.contains(e.target as Node)) {
      showMenu.value = false
    }
  })

  // 监听键盘事件（仅窗口内）
  window.addEventListener('keydown', handleKeyDown)

  // 加载初始数据
  try {
    const records = await invoke('get_all_records')
    allItems.value = records as ClipboardItem[]
  } catch (error) {
    console.error('加载数据失败:', error)
  }

  // 监听剪贴板变化事件
  const unlistenChange = await listen<ClipboardItem>('clipboard-change', (event) => {
    const newItem = event.payload
    allItems.value.unshift(newItem)
  })

  // 监听刷新事件
  const unlistenRefresh = await listen('clipboard-refresh', () => {
    invoke('get_all_records').then((records) => {
      allItems.value = records as ClipboardItem[]
    })
  })

  // 开始监听剪贴板
  try {
    await invoke('start_clipboard_monitor')
  } catch (error) {
    console.error('启动监听失败:', error)
  }

  return () => {
    window.removeEventListener('keydown', handleKeyDown)
    if (blurTimeout) clearTimeout(blurTimeout)
    unlistenChange()
    unlistenRefresh()
  }
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

// 切换标签时重置选择
watch(activeTab, () => {
  selectedIndex.value = -1
})
</script>

<style scoped>
#app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--app-bg-color-1);
  color: var(--app-font-color-base);
}

header {
  padding: 0.4rem 0.8rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--border-color-1);
  background: var(--app-bg-color-2);
  -webkit-app-region: drag;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  -webkit-app-region: drag;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 0.2rem;
  position: relative;
}

.app-logo {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

h1 {
  font-size: 0.9rem;
  margin: 0;
  font-weight: 500;
  color: var(--app-font-color-base);
  letter-spacing: 0.02em;
}

/* 汉堡菜单按钮 */
.menu-btn {
  width: 24px;
  height: 24px;
  border: 1px solid var(--border-color-1);
  border-radius: 3px;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-base);
  cursor: pointer;
  transition: all 0.15s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 3px;
  -webkit-app-region: no-drag;
}

.menu-btn:hover {
  background: var(--table-hover-color);
  border-color: var(--border-color-2);
}

.hamburger-icon {
  width: 18px;
  height: 18px;
}

/* 下拉菜单 */
.menu-dropdown {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  min-width: 240px;
  max-height: 70vh;
  background: var(--picker-bg-color);
  border: 1px solid var(--border-color-1);
  border-radius: 6px;
  box-shadow: var(--notification-shadow);
  z-index: 200;
  overflow: hidden;
  animation: fadeIn 0.15s ease;
}

.menu-content {
  max-height: 70vh;
  overflow-y: auto;
  overflow-x: hidden;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.menu-item {
  width: 100%;
  padding: 0.45rem 0.8rem;
  border: none;
  background: transparent;
  color: var(--app-font-color-base);
  font-size: 0.8rem;
  text-align: left;
  cursor: pointer;
  transition: background 0.15s ease;
  letter-spacing: 0.01em;
  display: flex;
  align-items: center;
}

.menu-item:hover {
  background: var(--table-hover-color);
}

.menu-divider {
  height: 1px;
  background: var(--border-color-1);
  margin: 0.25rem 0;
}

.menu-help-section {
  padding: 0.5rem 0.6rem;
}

.menu-help-title {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--app-font-color-2);
  margin-bottom: 0.4rem;
  letter-spacing: 0.01em;
  padding-bottom: 0.3rem;
  border-bottom: 1px solid var(--border-color-1);
}

.menu-help-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.3rem 0;
  font-size: 0.75rem;
  gap: 0.5rem;
  color: var(--app-font-color-2);
  letter-spacing: 0.01em;
}

.menu-help-key {
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 0.68rem;
  padding: 0.15rem 0.3rem;
  border-radius: 2px;
  font-weight: 500;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-2);
  white-space: nowrap;
}

.theme-toggle {
  width: 22px;
  height: 22px;
  border: 1px solid var(--border-color-1);
  border-radius: 3px;
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.15s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-base);
  -webkit-app-region: no-drag;
}

.theme-toggle:hover {
  background: var(--table-hover-color);
}

.help-tooltip {
  position: relative;
  cursor: pointer;
  -webkit-app-region: no-drag;
}

.help-icon {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 0.75rem;
  font-weight: 600;
  transition: all 0.15s ease;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-base);
}

.help-icon:hover {
  background: var(--table-hover-color);
}

.help-content {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 6px;
  min-width: 180px;
  padding: 0.6rem;
  border-radius: 6px;
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s ease;
  z-index: 100;
  background: var(--picker-bg-color);
  box-shadow: var(--notification-shadow);
}

.help-content.visible {
  opacity: 1;
  visibility: visible;
}

.help-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.25rem 0;
  font-size: 0.75rem;
  gap: 0.65rem;
  color: var(--app-font-color-base);
  letter-spacing: 0.01em;
}

.help-item:not(:last-child) {
  border-bottom: var(--app-boder-color-1);
}

.help-key {
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 0.65rem;
  padding: 0.12rem 0.3rem;
  border-radius: 3px;
  font-weight: 500;
  background: var(--app-bg-color-2);
  color: var(--app-font-color-base);
}

main {
  padding: 0.5rem 0.8rem;
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--app-bg-color-2);
}

.tabs {
  display: flex;
  gap: 0.3rem;
  margin-bottom: 0.5rem;
  flex-shrink: 0;
}

.tabs button {
  flex: 1;
  padding: 0.3rem 0.4rem;
  border: 1px solid transparent;
  border-radius: 3px;
  font-size: 0.78rem;
  font-weight: 400;
  cursor: pointer;
  transition: all 0.15s ease;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-2);
  letter-spacing: 0.01em;
}

.tabs button:hover {
  background: var(--table-hover-color);
  color: var(--app-font-color-base);
}

.tabs button.active {
  background: var(--current-row-bg-color);
  color: var(--app-font-color-base);
  border-color: var(--border-color-2);
}

.content {
  border-radius: 4px;
  //padding: 0.3rem;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  //background: var(--app-bg-color-3);
  //border: 1px solid var(--border-color-1);
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.item {
  padding: 0.45rem;
  border: 1px solid var(--border-color-1);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  gap: 0.5rem;
  align-items: flex-start;
  background: var(--app-bg-color-4);
}

.item:hover,
.item.selected {
  background: var(--table-hover-color);
  border-color: var(--border-color-2);
}

.item.selected {
  box-shadow: var(--notification-shadow);
}

.item.pinned {
  border-left: 3px solid #ffd700;
}

.item-number {
  min-width: 1.5rem;
  font-size: 0.7rem;
  font-weight: 500;
  padding: 0.18rem 0.3rem;
  text-align: center;
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  border-radius: 3px;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-2);
}

.hotkey-badge {
  background: #ffd700 !important;
  color: #1a1a1a !important;
  font-weight: 700;
  box-shadow: 0 2px 6px rgba(255, 215, 0, 0.3);
}

.item-content-wrapper {
  flex: 1;
  min-width: 0;
}

.item-header {
  display: flex;
  align-items: flex-start;
  gap: 0.4rem;
  margin-bottom: 0.25rem;
}

.item-content {
  flex: 1;
  font-size: 0.78rem;
  line-height: 1.35;
  word-break: break-word;
  color: var(--app-font-color-base);
  letter-spacing: 0.01em;
}

.item-preview-image {
  max-width: 100%;
  max-height: 80px;
  border-radius: 4px;
  margin-top: 0.25rem;
  object-fit: contain;
}

.item-actions {
  display: flex;
  gap: 0.2rem;
  flex-shrink: 0;
}

.btn-icon {
  width: 20px;
  height: 20px;
  border: none;
  border-radius: 3px;
  font-size: 0.72rem;
  cursor: pointer;
  transition: all 0.15s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-2);
}

.btn-icon:hover {
  background: var(--table-hover-color);
  color: var(--app-font-color-base);
}

.btn-icon.delete:hover {
  background: rgba(239, 68, 68, 0.2) !important;
}

.item-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 0.4rem;
}

.item-type {
  font-size: 0.62rem;
  padding: 0.08rem 0.3rem;
  border-radius: 3px;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-2);
  font-weight: 400;
}

.time {
  font-size: 0.62rem;
  color: var(--app-font-color-2);
  letter-spacing: 0.01em;
}

.empty {
  text-align: center;
  padding: 2rem 1rem;
}

.empty-icon {
  font-size: 2rem;
  margin-bottom: 0.6rem;
  opacity: 0.6;
}

.empty-text {
  font-size: 0.82rem;
  font-weight: 400;
  margin-bottom: 0.3rem;
  color: var(--app-font-color-2);
  letter-spacing: 0.01em;
}

.empty-hint {
  font-size: 0.72rem;
  color: var(--app-font-color-3);
  letter-spacing: 0.01em;
}

.status {
  position: fixed;
  bottom: 1rem;
  left: 50%;
  transform: translateX(-50%) translateY(10px);
  padding: 0.5rem 1rem;
  border-radius: 2rem;
  font-size: 0.8rem;
  font-weight: 500;
  opacity: 0;
  transition: all 0.25s ease;
  z-index: 100;
  background: var(--notification-bg);
  color: var(--notification-message-color);
  box-shadow: var(--notification-shadow);
}

.status.visible {
  opacity: 1;
  transform: translateX(-50%) translateY(0);
}

/* 滚动条样式 */
.content::-webkit-scrollbar {
  width: 5px;
}

.content::-webkit-scrollbar-track {
  background: var(--scrollbar-track-bg);
  border-radius: 2px;
}

.content::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb-bg);
  border-radius: 2px;
  transition: background 0.15s ease;
}

.content::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover-bg);
}

.content::-webkit-scrollbar-thumb:active {
  background: var(--scrollbar-thumb-hover-bg);
}

/* 菜单滚动条样式 */
.menu-content::-webkit-scrollbar {
  width: 4px;
}

.menu-content::-webkit-scrollbar-track {
  background: transparent;
}

.menu-content::-webkit-scrollbar-thumb {
  background: var(--border-color-1);
  border-radius: 2px;
}

.menu-content::-webkit-scrollbar-thumb:hover {
  background: var(--border-color-2);
}

/* 快捷键输入框 */
.shortcut-input {
  cursor: pointer;
  text-align: center;
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 0.75rem;
  letter-spacing: 0.01em;
}

.shortcut-hint {
  font-size: 0.72rem;
  color: var(--app-font-color-3);
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  letter-spacing: 0.01em;
}

/* 头部按钮 */
.header-btn {
  width: 22px;
  height: 22px;
  border: 1px solid var(--border-color-1);
  border-radius: 3px;
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.15s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-base);
  -webkit-app-region: no-drag;
}

.header-btn:hover {
  background: var(--table-hover-color);
}

/* 模态框 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.modal {
  background: var(--app-bg-color-2);
  border: 1px solid var(--border-color-1);
  border-radius: 8px;
  box-shadow: var(--notification-shadow);
  max-width: 500px;
  width: 90%;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--border-color-1);
  background: var(--app-bg-color-3);
}

.modal-header h2 {
  margin: 0;
  font-size: 1rem;
  color: var(--app-font-color-base);
  font-weight: 500;
  letter-spacing: 0.01em;
}

.modal-close {
  width: 22px;
  height: 22px;
  border: 1px solid var(--border-color-1);
  border-radius: 3px;
  background: var(--app-bg-color-4);
  color: var(--app-font-color-base);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.85rem;
  transition: all 0.15s ease;
}

.modal-close:hover {
  background: var(--table-hover-color);
}

.modal-body {
  padding: 1rem;
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: 1rem;
  border-top: 1px solid var(--border-color-1);
  background: var(--app-bg-color-3);
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}

/* 设置组 */
.setting-group {
  margin-bottom: 1.5rem;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-group h3 {
  margin: 0 0 0.7rem 0;
  font-size: 0.85rem;
  color: var(--app-font-color-base);
  font-weight: 500;
  letter-spacing: 0.01em;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.45rem 0;
}

.setting-item label {
  font-size: 0.8rem;
  color: var(--app-font-color-2);
  letter-spacing: 0.01em;
}

.setting-input {
  width: 110px;
  padding: 0.3rem 0.45rem;
  border: 1px solid var(--border-color-1);
  border-radius: 3px;
  background: var(--app-bg-color-3);
  color: var(--app-font-color-base);
  font-size: 0.8rem;
  letter-spacing: 0.01em;
}

.setting-input:focus {
  outline: none;
  border-color: var(--border-color-2);
}

/* 开关按钮 */
.toggle-btn {
  width: 44px;
  height: 24px;
  border: 1px solid var(--border-color-1);
  border-radius: 12px;
  background: var(--app-bg-color-4);
  cursor: pointer;
  position: relative;
  transition: all 0.2s ease;
  padding: 2px;
}

.toggle-btn.active {
  background: var(--border-color-2);
  border-color: var(--border-color-2);
}

.toggle-slider {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--app-font-color-3);
  transition: all 0.2s ease;
  display: block;
}

.toggle-btn.active .toggle-slider {
  transform: translateX(20px);
  background: #fff;
}

/* 按钮样式 */
.btn-primary {
  padding: 0.4rem 0.85rem;
  border: 1px solid var(--border-color-2);
  border-radius: 3px;
  background: var(--border-color-2);
  color: #fff;
  font-size: 0.8rem;
  font-weight: 400;
  cursor: pointer;
  transition: all 0.15s ease;
  letter-spacing: 0.01em;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-danger {
  padding: 0.4rem 0.85rem;
  border: 1px solid #e74c3c;
  border-radius: 3px;
  background: #e74c3c;
  color: #fff;
  font-size: 0.8rem;
  font-weight: 400;
  cursor: pointer;
  transition: all 0.15s ease;
  letter-spacing: 0.01em;
}

.btn-danger:hover {
  background: #c0392b;
}

/* 关于内容 */
.about-content {
  text-align: center;
}

.about-icon {
  font-size: 2.5rem;
  margin-bottom: 0.4rem;
}

.app-name {
  margin: 0 0 0.25rem 0;
  font-size: 1.15rem;
  color: var(--app-font-color-base);
  font-weight: 500;
  letter-spacing: 0.02em;
}

.app-version {
  margin: 0 0 0.4rem 0;
  font-size: 0.82rem;
  color: var(--app-font-color-3);
  letter-spacing: 0.01em;
}

.app-author {
  margin: 0 0 0.85rem 0;
  font-size: 0.78rem;
  color: var(--app-font-color-2);
  letter-spacing: 0.01em;
}

.app-description {
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: var(--app-bg-color-3);
  border-radius: 4px;
  text-align: left;
}

.app-description p {
  margin: 0 0 0.4rem 0;
  font-size: 0.8rem;
  color: var(--app-font-color-2);
  line-height: 1.5;
  letter-spacing: 0.01em;
}

.app-description p:last-child {
  margin: 0;
}

.app-features {
  text-align: left;
}

.app-features h4 {
  margin: 0 0 0.4rem 0;
  font-size: 0.85rem;
  color: var(--app-font-color-base);
  font-weight: 500;
}

.app-features ul {
  margin: 0;
  padding-left: 1rem;
  list-style-type: disc;
}

.app-features li {
  font-size: 0.8rem;
  color: var(--app-font-color-2);
  line-height: 1.55;
  margin-bottom: 0.25rem;
  letter-spacing: 0.01em;
}
</style>
