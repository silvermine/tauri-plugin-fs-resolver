<template>
   <main class="container">
      <h1>tauri-plugin-fs-resolver</h1>
      <div v-if="os === 'android'" class="radio-button-container">
         <label class="radio-button">
            <input type="radio" value="path" v-model="androidPathType" /> Path
         </label>
         <label class="radio-button">
            <input type="radio" value="collection" v-model="androidPathType" /> Collection
         </label>
      </div>
      <div v-if="os === 'windows'">
         <label class="radio-button">
            <input type="radio" value="win32" v-model="windowsPathType" /> Win32
         </label>
         <label class="radio-button">
            <input type="radio" value="winmsix" v-model="windowsPathType" /> WinMsix
         </label>
      </div>
      <div class="path-container">
         <p class="selected-path">{{ selectedPath }}</p>
         <p class="selected-path-resolved" :class="{ 'failed': didPathResolutionFail }">{{ selectedPathResolved }}</p>
      </div>
      <div class="directory-grid">
         <button v-for="directory in directories" :key="directory" @click="resolvePath(directory)">
            {{ directory }}
         </button>
      </div>
   </main>
</template>

<script setup lang="ts">

import { ref, computed, watch } from 'vue';
import { platform } from '@tauri-apps/plugin-os';
import { AndroidPath, AndroidPathCollection, IosPath, LinuxPath, MacPath, Win32Path, WindowsApplicationDataPath, resolveAndroidPath, resolveAndroidPathCollection, resolveIosPath, resolveLinuxPath, resolveMacPath, resolveWindowsPath } from 'tauri-plugin-fs-resolver';

const selectedPath = ref<string>('none selected'),
      selectedPathResolved = ref<string>('none resolved'),
      androidPathType = ref<'path' | 'collection'>('path'),
      windowsPathType = ref<'win32' | 'winmsix'>('win32'),
      os = platform(),
      didPathResolutionFail = ref(false);

const resolvePath = async (path: string) => {
   selectedPath.value = path;
   try {
      didPathResolutionFail.value = false;
      let resolvedPath: string = '';
      if (os === 'android') {
         if (androidPathType.value === 'path') {
            resolvedPath = await resolveAndroidPath(path as AndroidPath);
         } else {
            resolvedPath = (await resolveAndroidPathCollection(path as AndroidPathCollection)).join(', ');
         }
      } else if (os === 'ios') {
         resolvedPath = await resolveIosPath(path as IosPath);
      } else if (os === 'linux') {
         resolvedPath = await resolveLinuxPath(path as LinuxPath);
      } else if (os === 'macos') {
         resolvedPath = await resolveMacPath(path as MacPath);
      } else if (os === 'windows') {
         if (windowsPathType.value === 'win32') {
            resolvedPath = await resolveWindowsPath({ win32: path as Win32Path });
         } else {
            resolvedPath = await resolveWindowsPath({ winMsix: path as WindowsApplicationDataPath });
         }
      } else {
         throw new Error(`Unsupported platform: ${os}`);
      }

      selectedPathResolved.value = resolvedPath;
   } catch (error) {
      didPathResolutionFail.value = true;
      selectedPathResolved.value = error instanceof Error ? error.message : (error as any).toString();
   }
};

const resetSelectedPath = () => {
   selectedPath.value = 'none selected';
   selectedPathResolved.value = 'none resolved';
   didPathResolutionFail.value = false;
};

const directories = computed(() => {
   if (os === 'android') {
      if (androidPathType.value === 'path') {
         return [...Object.values(AndroidPath) as string[]];
      } else {
         return [...Object.values(AndroidPathCollection) as string[]];
      }
   } else if (os === 'ios') {
      return [...Object.values(IosPath) as string[]];
   }  else if (os === 'linux') {
      return [...Object.values(LinuxPath) as string[]];
   } else if (os === 'macos') {
      return [...Object.values(MacPath) as string[]];
   } else if (os === 'windows') {
      if (windowsPathType.value === 'win32') {
         return [...Object.values(Win32Path) as string[]];
      } else {
         return [...Object.values(WindowsApplicationDataPath) as string[]];
      }
   } else {
      throw new Error(`Unsupported platform: ${os}`);
   }
});

watch (androidPathType, () => {
   resetSelectedPath();
});

watch (windowsPathType, () => {
   resetSelectedPath();
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
