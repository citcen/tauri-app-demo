<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

const showMsg = () => {
  invoke('show_msg', { invokeMessage: '桑桑!' })
  console.log('hi')
}
const initProcess = async () => {
  await invoke("init_process");
  await listen<string>('event-name', (event) => {
    console.log(event);
  });
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="输入一个名字..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>

  <div>
    <button @click="showMsg">rust打印测试</button>

    <button @click="initProcess">触发事件</button>
  </div>
</template>
