<script setup lang="ts">
import Modal from "./Modal.vue"
import { ref, Ref } from 'vue';

type InvoiceItem {
  description: string,
  qty: integer,
  single: number,
  total: number
}

const client = ref("");
const clients = ref([]);
const voices Ref<InvoceItem>=  ref([]);
const total = ref(0);

defineProps([])
defineEmits([])


</script>

<template>
  <Modal @close="$emit('close')" >
    <div>

    </div>
  </Modal>
</template>

<style scoped>
</style>