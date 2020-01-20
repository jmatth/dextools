<template>
  <v-container fluid :style="{ height: containerHeight }">
    <v-row
      align="stretch"
      align-content="stretch"
      dense
      style="height: 100%"
    >
      <v-col cols="12" md="8" style="height: 100%">
        <v-skeleton-loader
          type="table"
          transition="scale-transition"
          :loading="loading"
          style="height: 100%"
        >
          <EventsList/>
        </v-skeleton-loader>
      </v-col>
      <v-col
        style="height: 100%"
        v-show="$vuetify.breakpoint.mdAndUp"
        cols="4"
      >
        <v-skeleton-loader
          type="table"
          transition="scale-transition"
          :loading="loading"
          style="height: 100%"
        >
          <AgendaCalendar calType='day'/>
        </v-skeleton-loader>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import EventsList from '@/components/EventsList.vue';
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import { Component, Watch, Vue } from 'vue-property-decorator';

@Component({
  components: {
    EventsList,
    AgendaCalendar,
  },
})
export default class Schedule extends Vue {
  get loading(): boolean {
    return this.$store.getters.loading;
  }

  @Watch('loading')
  public triggerResize(loading: boolean): void {
    if (!loading) {
      setTimeout(() => window.dispatchEvent(new Event('resize')), 1500);
    }
  }

  get containerHeight(): string {
    return `calc(100vh - ${this.$vuetify.application.top}px`;
  }
}
</script>
