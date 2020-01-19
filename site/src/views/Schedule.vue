<template>
  <v-container fluid :style="{ height: containerHeight }">
    <v-row
      align="stretch"
      align-content="stretch"
      v-if="Object.keys($store.state.schedule).length > 0"
      dense
      style="height: 100%"
    >
      <v-col cols="12" md="8" style="height: 100%">
        <EventsList/>
      </v-col>
      <v-col
        style="height: 100%"
        v-show="$vuetify.breakpoint.mdAndUp"
        cols="4"
        >
        <AgendaCalendar calType='day'/>
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
  get containerHeight(): string {
    return `calc(100vh - ${this.$vuetify.application.top}px`;
  }
}
</script>
