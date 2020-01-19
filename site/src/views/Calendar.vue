<template>
  <v-container fluid :style="{ height: containerHeight }">
    <v-row dense style="height: 100%">
      <v-col cols="12" style="height: 100%">
        <v-skeleton-loader
          type="table-tbody"
          transition="scale-transition"
          :loading="loading"
          style="height: 100%"
        >
          <AgendaCalendar calType="custom-daily" :height="workspaceHeight"/>
        </v-skeleton-loader>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import { Component, Watch, Vue } from 'vue-property-decorator';

@Component({
  components: {
    AgendaCalendar,
  },
})
export default class Calendar extends Vue {
  public workspaceHeight = 700;

  get loading(): boolean {
    return Object.keys(this.$store.state.schedule).length < 1;
  }

  @Watch('loading')
  public triggerResize(): void {
    setTimeout(() => window.dispatchEvent(new Event('resize')), 500);
  }

  get containerHeight(): string {
    return `calc(100vh - ${this.$vuetify.application.top}px`;
  }
}
</script>
