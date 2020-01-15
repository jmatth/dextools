<template>
  <v-container fluid>
    <v-row dense>
      <v-col cols="12">
        <v-skeleton-loader
          type="table-tbody"
          transition="scale-transition"
          :loading="loading"
        >
          <AgendaCalendar calType="custom-daily" :height="workspaceHeight"/>
        </v-skeleton-loader>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import EventsList from '@/components/EventsList.vue';
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import Event from '@/models/event';
import Agenda from '@/models/agenda';
import { debounce } from 'lodash';
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({
  components: {
    EventsList,
    AgendaCalendar,
  },
})
export default class Calendar extends Vue {
  public workspaceHeight = 700;

  constructor() {
    super();
    this.handleResize = debounce(this.handleResize, 250);
  }

  public mounted(): void {
    window.addEventListener('resize', this.handleResize);
    this.handleResize();
  }

  get loading(): boolean {
    return Object.keys(this.$store.state.schedule).length < 1;
  }

  public handleResize() {
    const windowHeight = window.innerHeight;
    const workspacePaddingStr = window
      .getComputedStyle(document.querySelector('div.container')!, null)
      .getPropertyValue('padding-bottom');
    const workspacePadding = workspacePaddingStr.endsWith('px')
      ? parseInt(workspacePaddingStr.substring(0, workspacePaddingStr.length - 2), 10)
      : 16;
    // @ts-ignore
    const headerHeight = document.querySelector('div#app div header.v-sheet')!.offsetHeight;
    this.workspaceHeight = windowHeight - (workspacePadding * 3 + headerHeight);
  }
}
</script>
