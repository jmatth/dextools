<template>
  <v-container>
    <v-row>
      <v-col cols="12">
        <ExportDialog/>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import ExportDialog from '@/components/ExportDialog.vue';
import { Component, Vue } from 'vue-property-decorator';

@Component({
  components: {
    ExportDialog,
  },
})
export default class Export extends Vue {}
</script>

