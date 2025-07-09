<template>
  <div class="background bg-contain bg-center bg-no-repeat aspect-[295/557] w-56 relative">
    <TheKeysItem
      v-slot="{ selectedLabel }"
      :key-index="1"
      class="translate-1/2 right-1/4 top-20"
    >
      <UButton size="sm">
        {{ selectedLabel || 'Right button' }}
      </UButton>
    </TheKeysItem>

    <TheKeysItem
      v-slot="{ selectedLabel }"
      :key-index="2"
      class="-translate-1/2 left-1/2 top-14"
    >
      <UButton size="sm">
        {{ selectedLabel || 'Middle button' }}
      </UButton>
    </TheKeysItem>
  </div>
</template>

<style>
.background {
  background-image: url('~/assets/mouse-top.png');
}
</style>
