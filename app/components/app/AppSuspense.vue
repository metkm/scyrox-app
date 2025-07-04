<template>
  <Suspense>
    <slot />

    <template #fallback>
      <UIcon
        name="i-lucide-loader-circle"
        class="size-14"
      />
    </template>
  </Suspense>
</template>
