<template>
  <v-dialog
    :value="value"
    @input="$emit('input', $event)"
    scrollable
    :max-width="maxWidth"
    :fullscreen="$vuetify.breakpoint.xsOnly"
    :eager="eager"
  >
    <template v-slot:activator="{ on }">
      <slot name="activator" :on="on"/>
    </template>
    <v-card>
      <v-card-title>
        {{ title }}
      </v-card-title>
      <v-card-text>
        <slot/>
      </v-card-text>
      <v-divider/>
      <v-card-actions>
        <v-btn @click="$emit('input', false)">
          Close
        </v-btn>
        <v-spacer/>
        <slot name="actions"/>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class Dialog extends Vue {
  @Prop() private value!: boolean;
  @Prop() private title!: string;
  @Prop({ default: '700px' }) private maxWidth!: string;
  @Prop({ type: Boolean, default: false }) private eager!: boolean;
}
</script>
