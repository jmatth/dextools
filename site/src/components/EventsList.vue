<template>
  <v-card>
    <v-toolbar dark>
      <v-text-field
        v-model="filter"
        label="Filter"
        single-line
        single
        dark
        ></v-text-field>

      <v-spacer></v-spacer>

    <v-checkbox v-for="code in availableCodes" v-model="categories" :label="code" :value="code"></v-checkbox>

    </v-toolbar>

    <div :style="{ height: height, overflowY: 'scroll' }">
      <template v-for="(item, index) in items">
        <v-card>
          <v-card-title>
            <v-avatar color="red">
              <span class="white--text headline">{{ item.code[0] }}</span>
            </v-avatar>
            &nbsp;&nbsp;
            <span class="headline">{{ item.code }}: {{ item.title }}</span>
            <v-chip disabled class="ml-2" v-if="item.filled">
              <v-icon left>lock</v-icon>
              Filled
            </v-chip>
          </v-card-title>

          <v-card-text>
            {{ item.description }}
          </v-card-text>

          <v-card-actions>
            <v-list-tile class="grow">
              <v-list-tile-content>
                <v-list-tile-title>{{ item.presenters || item.authors }}</v-list-tile-title>
              </v-list-tile-content>

              <v-layout
                align-center
                justify-end
                >
                <span class="mr-2">{{ item.startTime.format('ddd, HH:mm') }} - {{ item.endTime.format('ddd, HH:mm') }}</span>
                <v-btn flat v-on:click="schedule.addEvent(item)">Add</v-btn>
              </v-layout>
            </v-list-tile>
          </v-card-actions>
        </v-card>
      </template>
    </div>
  </v-card>
</template>

<script lang="ts">
import Schedule from '../models/schedule';
import Event from '../models/event';
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class EventsList extends Vue {
  @Prop() private eventSchedule!: any;
  @Prop() private schedule!: Schedule;
  @Prop() private height!: string;

  public categories: string[] = ['L', 'R'];
  public filter: string = '';

  get availableCodes() {
    return this.eventSchedule.reduce((acc: string[], e: Event) => {
      const code = e.code[0];
      if (!acc.includes(code)) {
        acc.push(code);
      }
      return acc;
    }, []);
  }

  get items() {
    return this.eventSchedule
      .filter((e: Event) => this.categories.includes(e.code[0]))
      .filter((e: Event) => {
        if (!this.filter) {
          return true;
        }
        return [
          'title',
          'system',
          'description',
          'presenters',
          'authors',
        ].some((field) => (e[field] as string).toLowerCase().includes(this.filter.toLowerCase()));
      });
  }
}
</script>
