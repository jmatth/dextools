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

    <v-checkbox v-model="categories" label="L" value="L"></v-checkbox>
    <v-checkbox v-model="categories" label="R" value="R"></v-checkbox>
    <v-checkbox v-model="categories" label="B" value="B"></v-checkbox>
    <v-checkbox v-model="categories" label="G" value="G"></v-checkbox>
    </v-toolbar>

    <div style="max-height: 500px; overflow-y: scroll;">
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

  public categories: string[] = ['L', 'R'];
  public filter: string = '';

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
