<template>
  <v-expansion-panel>
    <v-expansion-panel-header>
      <v-container fluid class="event-item-row-header">
        <v-row no-gutters>
          <v-col v-once>
            <v-row no-gutters>
              <v-col cols="12" sm="5">
                <span>{{ item.code }}: {{ item.title }}</span>
              </v-col>
              <v-col cols="12" sm="5">
                {{ item.system }}
              </v-col>
              <v-col cols="12" sm="1">
                <v-icon
                  v-if="item.testType === 'FOCUS GROUP'"
                  size="20"
                >
                  group
                </v-icon>
                <span
                  v-else
                >
                  {{ testTypeText(item.testType) }}
                </span>
              </v-col>
              <v-col cols="12" sm="5">
                {{ item.startTime.format('ddd, h:mmA') }} - {{ item.endTime.format('h:mmA') }}
              </v-col>
              <v-col cols="12" sm="5">
                {{ item.presenters }}
              </v-col>
              <v-col cols="12" sm="1">
                <v-tooltip
                  bottom
                  v-if="item.filled"
                >
                  <template v-slot:activator="{ on }">
                    <v-icon
                      size="20"
                      v-on="on"
                    >
                      lock
                    </v-icon>
                  </template>
                  <span>This event has been filled, you may sign up as an alternate at the convention.</span>
                </v-tooltip>
                <v-tooltip
                  bottom
                  v-if="item.hiTest"
                >
                  <template v-slot:activator="{ on }">
                    <v-icon
                      size="20"
                      v-on="on"
                    >
                      error_outline
                    </v-icon>
                  </template>
                  <span>This is a HI-TEST session.</span>
                </v-tooltip>
              </v-col>
            </v-row>
          </v-col>
        </v-row>
      </v-container>
    </v-expansion-panel-header>
    <v-expansion-panel-content>
      {{ item.description }}
      <br/>
      <v-btn style="float: right;" :color="itemActionColor(item)" :loading="processing" @click="toggleEvent(item)">
        {{ itemActionText(item) }}
      </v-btn>
    </v-expansion-panel-content>
  </v-expansion-panel>
</template>

<script lang="ts">
import Event from '@/models/event';
import { Component, Prop, Vue } from 'vue-property-decorator';
import log from 'loglevel';

@Component
export default class EventsListItem extends Vue {
  @Prop({ type: Object }) private item!: Event;

  private processing: boolean = false;

  public testTypeText(type?: string): string {
    switch (type) {
      case 'ALPHA TEST': {
        return 'α';
      }
      case 'BETA TEST': {
        return 'β';
      }
      case '': {
        return '';
      }
      case undefined: {
        return '';
      }
    }
    log.error(`Found unknown test type ${type}`);
    return '';
  }

  public toggleEvent(event: Event) {
    this.processing = true;
    this.$nextTick(() => {
      this.$store.commit('toggleEvent', event.code);
      this.processing = false;
    });
  }

  public itemActionColor(event: Event) {
    return this.$store.state.agenda.contains(event.code)
      ? 'error'
      : 'success';
  }

  public itemActionText(event: Event) {
    return this.$store.state.agenda.contains(event.code)
      ? 'Remove'
      : 'Add';
  }
}
</script>
