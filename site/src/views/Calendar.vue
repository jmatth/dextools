<template>
  <v-container fluid :style="{ height: containerHeight }">
    <v-row dense style="height: 100%">
      <v-col cols="12" style="height: 100%">
        <v-skeleton-loader
          type="table-tbody"
          transition="fade-transition"
          :loading="loading"
          style="height: 100%"
        >
          <AgendaCalendar initialView="custom-daily"/>
        </v-skeleton-loader>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import { Component, Vue } from 'vue-property-decorator';

@Component({
  components: {
    AgendaCalendar,
  },
})
export default class Calendar extends Vue {
  get loading(): boolean {
    return this.$store.state.loading;
  }

  get containerHeight(): string {
    return `calc(100vh - ${this.$vuetify.application.top}px`;
  }
}
</script>
