<template>
  <DynamicScrollerItem
    :item="event"
    :active="active"
    :data-index="index"
    :size-dependencies="[ expanded, resizeTicker ]"
    :class="{ 'event-item-row': true, 'event-item-row-expanded': expanded }"
  >
    <v-container
      fluid
      :class="{ 'event-item-row-header': true, 'event-item-row-header-expanded': expanded }"
      @click="$emit('expand', event.code)"
    >
      <v-row no-gutters>
        <v-col cols="11">
          <v-row no-gutters>
            <v-col cols="12" sm="5">
              <span>{{ event.code }}: {{ event.title }}</span>
            </v-col>
            <v-col cols="12" sm="5">
              {{ event.system }}
            </v-col>
            <v-col cols="12" sm="1">
              <v-icon
                v-if="event.testType === 'FOCUS GROUP'"
                dense
              >
                group
              </v-icon>
              <span
                v-else
              >
                {{ testTypeText(event.testType) }}
              </span>
            </v-col>
            <v-col cols="12" sm="5">
              {{ event.startTime.format('ddd, h:mmA') }} - {{ event.endTime.format('h:mmA') }}
            </v-col>
            <v-col cols="12" sm="5">
              {{ event.presenters }}
            </v-col>
            <v-col cols="12" sm="1">
              <v-tooltip
                bottom
                v-if="event.filled"
              >
                <template v-slot:activator="{ on }">
                  <v-icon
                    dense
                    v-on="on"
                  >
                    lock
                  </v-icon>
                </template>
                <span>This event has been filled, you may sign up as an alternate at the convention.</span>
              </v-tooltip>
              <v-tooltip
                bottom
                v-if="event.hiTest"
              >
                <template v-slot:activator="{ on }">
                  <v-icon
                    dense
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
        <v-col cols="1">
          <v-layout row wrap>
            <v-col cols="12" sm="6">
              <v-btn
                text
                icon
                :aria-label="addButtonLabel(event)"
                @click.stop="toggleEvent(event)"
              >
                <v-icon
                  dense
                  :color="itemActionColor(event)"
                >
                  {{ itemActionIcon(event) }}
                </v-icon>
              </v-btn>
            </v-col>
            <v-col cols="12" sm="6">
              <v-btn
                text
                icon
                depressed
                :aria-label="expandButtonLabel(event)"
              >
                <v-icon class="event-item-row-header-expand-icon">
                  keyboard_arrow_down
                </v-icon>
              </v-btn>
            </v-col>
          </v-layout>
        </v-col>
      </v-row>
    </v-container>
    <v-slide-y-transition
      hide-on-leave
      @before-enter="triggerResize"
      @after-leave="triggerResize"
    >
      <div v-show="expanded" class="event-item-row-body">
        {{ event.description }}
      </div>
    </v-slide-y-transition>
  </DynamicScrollerItem>
</template>

<script lang="ts">
import Event from '@/models/event';
import { Component, Prop, Vue } from 'vue-property-decorator';
import log from 'loglevel';

@Component
export default class EventsListItem extends Vue {
  @Prop({ type: Object, required: true }) private event!: Event;
  @Prop({ type: Boolean, default: false }) private expanded!: boolean;
  @Prop({ type: Number, required: true }) private index!: boolean;
  @Prop({ type: Boolean, required: true }) private active!: boolean;

  public resizeTicker: boolean = false;

  public triggerResize(): void {
    this.resizeTicker = !this.resizeTicker;
  }

  public toggleEvent(event: Event) {
    this.$store.commit('toggleEvent', event.code);
  }

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

  public itemActionIcon(event: Event) {
    return this.$store.state.agenda.contains(event.code)
      ? 'remove_circle'
      : 'add_circle';
  }

  public itemActionColor(event: Event) {
    return this.$store.state.agenda.contains(event.code)
      ? 'error'
      : 'success';
  }

  public addButtonLabel(event: Event): string {
    return this.$store.state.agenda.contains(event.code)
      ? 'Remove from schedule'
      : 'Add to schedule';
  }

  public expandButtonLabel(event: Event): string {
    return this.expanded
      ? 'Collapse'
      : 'Expand';
  }
}
</script>

<style lang="scss">
.event-item-row {
  border-bottom: 1px solid rgba(0,0,0,0.12);

  &-expanded {
    margin-bottom: 10px;
  }

  &-header {
    padding: 12px 24px 12px 24px;
    cursor: pointer;
    
    &-expanded {
      .event-item-row-header-expand-icon {
        transform: rotateX(-180deg);
      }
    }
  }

  &-body {
    padding: 0px 24px 12px 24px;
  }
}
</style>
