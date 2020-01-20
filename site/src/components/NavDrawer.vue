<template>
  <v-navigation-drawer
    :value="value"
    @input="$emit('input', $event)"
    app
    clipped
    bottom
    disable-resize-watcher
    :mobile-break-point="$vuetify.breakpoint.thresholds.sm"
  >
    <v-list dense>
      <v-list-item link to="/">
        <v-list-item-action>
          <v-icon>list_alt</v-icon>
        </v-list-item-action>
        <v-list-item-content>
          <v-list-item-title>Schedule</v-list-item-title>
        </v-list-item-content>
      </v-list-item>
      <v-list-item link to="/calendar">
        <v-list-item-action>
          <v-icon>calendar_today</v-icon>
        </v-list-item-action>
        <v-list-item-content>
          <v-list-item-title>Calendar</v-list-item-title>
        </v-list-item-content>
      </v-list-item>
      <v-list-item link to="/export">
        <v-list-item-action>
          <v-icon>mdi-export-variant</v-icon>
        </v-list-item-action>
        <v-list-item-content>
          <v-list-item-title>Export</v-list-item-title>
        </v-list-item-content>
      </v-list-item>
    </v-list>
    <v-divider/>
    <v-list>
      <v-list-item>
        <v-switch
          class="mt-0 pt-0"
          label="Dark Mode"
          hide-details
          v-model="darkMode"
        />
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class NavDrawer extends Vue {
  @Prop() private value!: boolean;

  get darkMode(): boolean {
    return this.$vuetify.theme.dark;
  }

  set darkMode(dark) {
    this.$vuetify.theme.dark = dark;
    localStorage.darkMode = dark;
  }
}
</script>
