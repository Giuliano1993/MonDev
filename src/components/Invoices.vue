<script setup>
import { ref } from 'vue';


defineProps([])
defineEmits([])


const fatture = ref([]);

</script>

<template>
  <table>
    <thead>
      <tr>
        <th>N.</th>
        <th>Dat3</th>
        <th>Client</th>
        <th>Value</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="f in fatture">
        <td>{{ f.n }}</td>
        <td>{{ f.date }}</td>
        <td>{{ f.client }}</td>
        <td>{{ f.value }}</td>
      </tr>
    </tbody>
  </table>

  <button>New Invoice</button>
</template>

<style scoped>
</style>