<template>
  <Dialog
    v-model="show"
    title="About"
    eager
  >
    <template v-slot:activator="{ on }">
      <v-btn
        fab
        depressed
        text
        aria-label="Feedback"
        v-on="on"
      >
        <v-icon>
          help_outline
        </v-icon>
      </v-btn>
    </template>
    <template>
      <h2>What's this?</h2>
      <p>
      This is a site to help with building your schedule for the excellent conventions run by Double Exposure Inc.
      It uses data scraped from the Double Exposure website so may be innacurrate or out of date.
      </p>
      <h2>How can I build my convention schedule?</h2>
      <p>
      Use the <b>+</b> buttons in the event list to add an event to your schedule. You can see your current schedule in the calendar view.
      To remove an event from your schedule, just click its entry in the calendar. When you are satisfied with your schedule, you can click
      the <b>Export</b> button at the top of the calendar to export your schedule to an email to sign up for events, or to a format for importing
      into calendar applications like Google Calendar.
      <b>
        Simply adding events to your schedule will not register you for those events, you must still send email to the
        appropriate email address for the convention. This site will just help you produce the text for that email.
      </b>
      </p>
      <h2>How do I use the search features?</h2>
      <p>
      It is possible to filter the events list by several criteria:
      <ul>
        <li>
          <b>Categories</b> can be used to filter by the first character in the event code.
          This usually corresponds to the event type in some way, such as R for RPGs, L for LARPs, B for board games, etc.
        </li>
        <li>
          The <b>Filter</b> box can be used to type an arbitrary query.
          Events that contain that string in any of their fields will be included in the results.
        </li>
      </ul>
      By clicking the
      <v-icon dense aria-label="Advanced filters">filter_list</v-icon>
      icon, you can view advanced search options:
      <ul>
        <li>
          <b>Days</b> can be used to restrict your search to events that take place on a particular day.
        </li>
        <li>
          <b>Start at</b> limits your search to events that start at <em>exactly</em> the specified time.
        </li>
        <li>
          <b>Hide filled</b> will hide events that have already been marked as filled.
          There can be significant lag between an event filling up and that information being reflected on the Double Exposure
          website, so information on which events are filled will often be out of date.
        </li>
        <li>
          <b>Hide Conflicting</b> will not show any events that overlap with events already added to your schedule.
          Useful for when you have your schedule mostly figured out and are just looking for something to fill some time.
        </li>
      </ul>
      </p>
      <h2>Questions/bug reports/feedback?</h2>
      <p>
      Any feedback via <a :href="this.$store.state.feedbackUrl" target="_blank">this anonymous form</a> is greatly appreciated.
      If you are so inclined, you can also open issues directly <a href="https://github.com/jmatth/dextools/issues/new" target="_blank">on the github repo</a>.
      </p>
      <h2>I should probably add...</h2>
      <p>
      This site is not associated with or endorsed by Double Exposure Inc.
      </p>
    </template>
  </Dialog>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Dialog from '@/components/Dialog.vue';

@Component({
  components: {
    Dialog,
  },
})
export default class AboutDialog extends Vue {
  private show: boolean = false;
}
</script>
