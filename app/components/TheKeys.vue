<template>
  <div class="background bg-contain bg-no-repeat transition-all aspect-[295/557] bg-center max-h-full relative shrink-0 flex-1">
    <TheKey
      v-slot="{ selectedLabel }"
      :key-index="1"
      class="translate-1/2 right-1/4 top-20"
    >
      <UButton size="sm">
        {{ selectedLabel || 'Right button' }}
      </UButton>
    </TheKey>

    <TheKey
      v-slot="{ selectedLabel }"
      :key-index="2"
      class="-translate-1/2 left-1/2 top-14"
    >
      <UButton size="sm">
        {{ selectedLabel || 'Middle button' }}
      </UButton>
    </TheKey>
  </div>
</template>

<style>
.background {
  background-image: url('~/assets/mouse-top.png');
}
</style>
