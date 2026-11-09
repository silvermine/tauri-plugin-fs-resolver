<template>
   <main class="container">
      <h1>tauri-plugin-fs-resolver</h1>
      <div v-if="selectedEnvironment === 'android'" class="radio-button-container">
         <label class="radio-button">
            <input type="radio" value="path" v-model="androidPathType" /> Path
         </label>
         <label class="radio-button">
            <input type="radio" value="collection" v-model="androidPathType" /> Collection
         </label>
      </div>
      <div
         v-if="
            selectedEnvironment === 'win32'
            || selectedEnvironment === 'winpackaged'
            || selectedEnvironment === 'winlibrary'
         "
         class="radio-button-container"
      >
         <label class="radio-button">
            <input type="radio" value="win32" v-model="selectedEnvironment" /> Win32
         </label>
         <label class="radio-button">
            <input type="radio" value="winpackaged" v-model="selectedEnvironment" /> WinPackaged
         </label>
         <label class="radio-button">
            <input type="radio" value="winlibrary" v-model="selectedEnvironment" /> WinLibrary
         </label>
      </div>
      <h2 v-if="selectedEnvironment === 'winlibrary'" class="section-title">Windows Libraries</h2>
      <p v-if="selectedEnvironment === 'winlibrary'" class="section-hint">
         Member folders pinned in each library (not the .library-ms definition path). Buttons
         list each <code>WindowsLibrary</code> variant (including camera roll).
      </p>
      <div class="path-container">
         <p class="selected-path">{{ selectedPath }}</p>
         <p
            v-if="selectedEnvironment !== 'winlibrary' || didPathResolutionFail"
            class="selected-path-resolved"
            :class="{ 'failed': didPathResolutionFail }"
         >{{ selectedPathResolved }}</p>
         <p
            v-else-if="selectedEnvironment === 'winlibrary' && selectedPath !== 'none selected'"
            class="selected-path-resolved"
         >{{ selectedPathResolved }}</p>
         <ul
            v-if="selectedEnvironment === 'winlibrary' && resolvedLibraryDirs.length && !didPathResolutionFail"
            class="library-paths"
         >
            <li v-for="libraryDir in resolvedLibraryDirs" :key="libraryDir">{{ libraryDir }}</li>
         </ul>
      </div>
      <div class="directory-grid">
         <button v-for="directory in directories" :key="directory" @click="resolvePath(directory)">
            {{ directory }}
         </button>
      </div>
   </main>
</template>

<script setup lang="ts">

import { ref, computed, watch, onMounted, ComputedRef } from 'vue';
import {
   AndroidPath,
   AndroidPathCollection,
   IosPath,
   LinuxPath,
   MacPath,
   Win32Path,
   WindowsApplicationDataPath,
   WindowsLibrary,
   getFsEnvironment,
   resolveAndroidPath,
   resolveAndroidPathCollection,
   resolveIosPath,
   resolveLinuxPath,
   resolveMacPath,
   resolveWin32Path,
   resolveWindowsApplicationDataPath,
   resolveWindowsLibraryDirs,
   FsEnvironment,
} from 'tauri-plugin-fs-resolver';

type DemoEnvironment = FsEnvironment | 'winlibrary';

const selectedPath = ref<string>('none selected'),
      selectedPathResolved = ref<string>('none resolved'),
      resolvedLibraryDirs = ref<string[]>([]),
      androidPathType = ref<'path' | 'collection'>('path'),
      detectedEnvironment = ref<FsEnvironment>('android'),
      selectedEnvironment = ref<DemoEnvironment>('android'),
      didPathResolutionFail = ref(false);

const resolvePath = async (path: string) => {
   selectedPath.value = path;
   resolvedLibraryDirs.value = [];
   try {
      didPathResolutionFail.value = false;
      let resolvedPath: string = '';
      if (selectedEnvironment.value === 'android') {
         if (androidPathType.value === 'path') {
            resolvedPath = await resolveAndroidPath(path as AndroidPath);
         } else {
            resolvedPath = (await resolveAndroidPathCollection(path as AndroidPathCollection)).join(', ');
         }
      } else if (selectedEnvironment.value === 'ios') {
         resolvedPath = await resolveIosPath(path as IosPath);
      } else if (selectedEnvironment.value === 'linux') {
         resolvedPath = await resolveLinuxPath(path as LinuxPath);
      } else if (selectedEnvironment.value === 'macos') {
         resolvedPath = await resolveMacPath(path as MacPath);
      } else if (selectedEnvironment.value === 'win32') {
         resolvedPath = await resolveWin32Path(path as Win32Path);
      } else if (selectedEnvironment.value === 'winpackaged') {
         resolvedPath = await resolveWindowsApplicationDataPath(path as WindowsApplicationDataPath);
      } else if (selectedEnvironment.value === 'winlibrary') {
         const dirs = await resolveWindowsLibraryDirs(path as WindowsLibrary);
         resolvedLibraryDirs.value = dirs;
         resolvedPath = dirs.length ? `${dirs.length} folder(s)` : '(empty)';
      } else {
         throw new Error(`Unsupported platform: ${selectedEnvironment.value}`);
      }

      selectedPathResolved.value = resolvedPath;
   } catch (error) {
      didPathResolutionFail.value = true;
      resolvedLibraryDirs.value = [];
      selectedPathResolved.value = error instanceof Error ? error.message : (error as any).toString();
   }
};

const resetSelectedPath = () => {
   selectedPath.value = 'none selected';
   selectedPathResolved.value = 'none resolved';
   resolvedLibraryDirs.value = [];
   didPathResolutionFail.value = false;
};

const directories: ComputedRef<string[]> = computed(() => {

   switch (selectedEnvironment.value) {
      case 'android':
         return [...Object.values(AndroidPath) as string[]];
      case 'ios':
         return [...Object.values(IosPath) as string[]];
      case 'linux':
         return [...Object.values(LinuxPath) as string[]];
      case 'macos':
         return [...Object.values(MacPath) as string[]];
      case 'win32':
         return [...Object.values(Win32Path) as string[]];
      case 'winpackaged':
         return [...Object.values(WindowsApplicationDataPath) as string[]];
      case 'winlibrary':
         return [...Object.values(WindowsLibrary) as string[]];
      default:
         return [];
   }
});

watch (androidPathType, () => {
   resetSelectedPath();
});

watch (selectedEnvironment, () => {
   resetSelectedPath();
});

onMounted(async () => {
   detectedEnvironment.value = await getFsEnvironment();
   selectedEnvironment.value = detectedEnvironment.value;
});

</script>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

html, body {
  margin: 0;
  height: 100%;
  overflow: hidden;
}

#app {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.container {
  margin: 0;
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.directory-grid {
   display: grid;
   grid-template-columns: 1fr;
   gap: 0.5rem;
   flex: 1;
   min-height: 0;
   overflow-y: auto;
   padding-bottom: 0.5rem;
   padding-top: 0.5rem;
   align-content: start;
   overflow-y: auto;
}

.path-container {
   margin-bottom: 0.5rem;
}

.section-title {
  margin: 0.25rem 0 0;
  font-size: 1.25rem;
  font-weight: 600;
  text-align: left;
}

.section-hint {
  margin: 0 0 0.5rem;
  font-size: 0.875rem;
  text-align: left;
  opacity: 0.85;
}

.library-paths {
  margin: 0.25rem 0 0;
  padding-left: 1.25rem;
  text-align: left;
  overflow-wrap: break-word;
  max-height: 12rem;
  overflow-y: auto;
}

.selected-path {
  text-align: left;
  text-wrap: wrap;
  overflow-wrap: break-word;
  font-weight: bold;
}

.selected-path-resolved {
  text-align: left;
  text-wrap: wrap;
  overflow-wrap: break-word;
}

.selected-path-resolved.failed {
  color: #ff0000;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  height: 3rem;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button {
  outline: none;
}

.radio-button-container {
  display: flex;
  justify-content: left;
  gap: 0.2rem;
}

.radio-button {
  justify-content: center;
  background-color: #ffffff;
  border: 1px solid #0f0f0f;
  border-radius: 8px;
  padding: 0.2rem 0.5rem;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

.radio-button input:checked {
  background-color: #0f0f0f;
  color: #ffffff;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
