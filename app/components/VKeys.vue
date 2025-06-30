<script setup lang="ts">
</script>

<template>
  <div class="background bg-no-repeat w-[295px] aspect-[295/557] relative">
    <VKey
      v-slot="{ selectedLabel }"
      :key-index="1"
    >
      <UButton>
        {{ selectedLabel || 'Right button' }}
      </UButton>
    </VKey>
  </div>
</template>

<style>
.background {
  background-image: url('~/assets/mouse-top.png');
}
</style>
