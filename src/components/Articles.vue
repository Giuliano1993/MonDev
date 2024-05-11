<script setup lang="ts">
import { ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api';


defineProps([])
defineEmits([])
const articleText = ref("");
const corrections: Ref<null|Array<any>> = ref(null);
</script>

<template>
  <textarea v-model="articleText"></textarea>
  <button>Get Corrections</button>
</template>

<style scoped>
  textarea{
    width: 100%;
  }
</style>