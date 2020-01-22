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
          transition="fade-transition"
          :loading="loading"
          style="height: 100%"
        >
          <EventsList/>
        </v-skeleton-loader>
      </v-col>
      <v-col
        style="height: 100%"
        v-if="$vuetify.breakpoint.mdAndUp"
        cols="4"
      >
        <v-skeleton-loader
          type="table"
          transition="fade-transition"
          :loading="loading"
          style="height: 100%"
        >
          <AgendaCalendar initialView='day'/>
        </v-skeleton-loader>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import EventsList from '@/components/EventsList.vue';
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import { Component, Vue } from 'vue-property-decorator';

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

  get containerHeight(): string {
    return `calc(100vh - ${this.$vuetify.application.top}px`;
  }
}
</script>
