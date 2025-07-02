<template>
  <div class="background bg-contain bg-no-repeat w-44 lg:w-64 transition-all aspect-[295/557] relative shrink-0">
    <TheKey
      v-slot="{ selectedLabel }"
      :key-index="1"
    >
      <UButton size="sm">
        {{ selectedLabel || 'Right button' }}
      </UButton>
    </TheKey>
  </div>
</template>

<style>
.background {
  background-image: url('~/assets/mouse-top.png');
}
</style>
