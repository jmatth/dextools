<template>
  <v-card
    class="mx-auto"
    max-width="600"
  >
    <v-card-title
      class="blue-grey white--text"
    >
      <span class="title">Your Schedule</span>
      <v-spacer></v-spacer>
      <v-btn
        dark
        depressed
        @click="schedule.exportIcs()"
      >
        Export
      </v-btn>
    </v-card-title>
    <v-card-text class="py-0">
      <v-timeline dense>
        <v-slide-x-reverse-transition
          group
          hide-on-leave
        >
          <v-timeline-item
            v-for="event in schedule.events"
            :key="event.code"
            :color="'info'"
            small
            fill-dot
          >
            <v-alert
              :value="true"
              :color="'info'"
              :icon="'mdi-information'"
            >
              <v-btn
                depressed
                right
                icon
                color="red"
                @click="schedule.removeEvent(event.code)"
              >
                <v-icon>delete</v-icon>
              </v-btn>
              {{ event.code }} - {{ event.title }}
              </span>
            </v-alert>
          </v-timeline-item>
        </v-slide-x-reverse-transition>
      </v-timeline>
    </v-card-text>
  </v-card>
</template>
<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Schedule from '../models/schedule';

@Component
export default class EventsList extends Vue {
  @Prop() private schedule!: Schedule;

  public categories: string[] = ['L', 'R'];
  public filter: string = '';

  get items() {
    return this.schedule.events
      .filter(e => this.categories.includes(e.code[0]))
      .filter(e => {
        if (!this.filter) {
          return true;
        }
        return ['title', 'system', 'description', 'presenters', 'authors'].some(field => e[field].toLowerCase().includes(this.filter.toLowerCase()));
      });
  }
}
</script>

