<template>
  <v-layout row>
    <v-flex xs12 sm6 offset-sm3>
      <v-card>
        <v-toolbar color="cyan" dark>
          <v-toolbar-side-icon></v-toolbar-side-icon>

          <v-toolbar-title>Inbox</v-toolbar-title>

          <v-spacer></v-spacer>

          <v-btn icon>
            <v-icon>search</v-icon>
          </v-btn>
        </v-toolbar>

        <template v-for="(item, index) in items">
          <v-card>
            <v-card-title>
              <v-avatar color="red">
                <span class="white--text headline">{{ item.code[0] }}</span>
              </v-avatar>
              &nbsp;&nbsp;
              <span class="headline">{{ item.code }}: {{ item.title }}</span>
            </v-card-title>

            <v-card-text class="">
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
                  <v-btn flat>Add</v-btn>
                </v-layout>
              </v-list-tile>
            </v-card-actions>
          </v-card>
        </template>
      </v-card>
    </v-flex>
  </v-layout>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class EventsList extends Vue {
  public items = [
    {
      "code": "L002",
      "title": "Long Time Listener, Last Time Caller",
      "system": "",
      "description": "Often funny and always poignant, this LARP has players portray the host and listeners of a radio call in show at the end of the world. One of the 2017 Golden Cobra winners.",
      "presenters": "Sara Williamson",
      "authors": "Jeff Dieterle",
      "time": "Thursday, 8:00PM - 10:00PM",
      "tags": ""
    },
    {
      "code": "L020",
      "title": "AAAGH! You Blinded Me...With Science!",
      "system": "Bulldogs! LARP",
      "description": "A DREAMATION 2018 EXCLUSIVE! So no one told you life was gonna be this way... Your job's a joke, you're broke, your crewmate's DOA. You're stuck on Job Tower, the central hub of galactic transport. You and the crew of the starship Klendathu have royally pissed off Boss Fallon, meaning that your next delivery has few thug-shaped hurdles in between you and payday. It doesn't help that your cargo is marked highly volatile, a phrase that could easily be applied to the remote science station marked for drop off as well. Can you avoid mafia entanglements, space pirates, and wildly incompatible scientific theories to successfully deliver your cargo, get paid, and get the hell out there? To get in touch early for character choice and costuming tips, send an email to info@galileogames.com.",
      "presenters": "Brennan Taylor",
      "authors": "",
      "time": "Friday, 2:00PM - 6:00PM",
      "tags": ""
    },
    {
      "code": "G081",
      "title": "Madhouse",
      "system": "GREYHAWK REBORN",
      "description": "William Guardino, David Guerrieri and John Lent. GHR-KEO17-09: The vacuum at the top of House Neheli has hung in the air, ignored for almost two decades. Now fortune and fate have thrust you into Castle Draconis Imperius to find a way to fill the void. A four-hour Story-arc Universal Dungeon adventure for character levels 5-10, and Part 4 of the House Adrift story. Greyhawk Reborn uses the Fifth Edition D&D Rules Set.",
      "presenters": "",
      "authors": "Jay Stypinski",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R215",
      "title": "The Wreck",
      "system": "Psi*Run",
      "description": "An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! You're not sure why you're here. You're not sure why you're bleeding. You're not sure who you are. You have so many questions, but the one thing you know is that those people moving toward you are not your friends. You don't know why you recognize them, but you do, and you need to run.",
      "presenters": "Richard Ruane",
      "authors": "Night Sky Games",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R216",
      "title": "Evidence of Autumn",
      "system": "Ryuutama",
      "description": "An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! The changing seasons have brought tension and confusion to the Windy Expanse. The soil has turned sour and overflowing with life, threatening this year's harvest and the region's farming future. Under the watch of a Crimson Ryuujin, a handful of travelers find themselves in a position to save the harvest before nothing is left for the coming winter. Ryuutama is a natural fantasy RPG: The fullness of nature and the wonder of the changing seasons beckons the intrepid traveler",
      "presenters": "Rach Shelkey",
      "authors": "Kotodama Heavy Industries",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R217",
      "title": "A Queen's Wrath Is a Messenger of Death, Part One",
      "system": "The Art of Power",
      "description": "An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! The war does not go well, and with its fortunes decline the Queen's temper. Her wrath will be felt throughout the realm. Can you assuage her anger, or will you be fall victim? The Art of Power is a political game set in a medieval world. Take roles from the lowliest to the most high and see if you can achieve your goals. Players may play in any or all of the three parts of this event. Character preferences will be given to those who play in all three rounds.",
      "presenters": "Brennan Taylor",
      "authors": "Galileo Games",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R218",
      "title": "Becoming Doctors",
      "system": "The Ward (Acute Care Edition)",
      "description": "An Envoy Event! An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! A modern medical drama, Powered by the Apocalypse. Good evening, Doctor. We've got an FLK with PFH showing bilateral greenstick fractures in Two, a couple of puppies digging for worms on a GOMER in Five, and the Chief wants to talk to you about your charting. Oh, yeah. Your daughter left a message saying she needs someone to pick her up from practice; I guess your ex forgot to show up. Where shall we begin? (Scenario to be determined at the table.)",
      "presenters": "Kevin Petker",
      "authors": "Magpie Games",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R219",
      "title": "Join the Watch",
      "system": "The Watch",
      "description": "Andrew Medeiros. An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! Four months ago it attacked without warning, overwhelming the border clans with superior numbers and the unnatural strength of the Shadow-held. In a short time, you have lost so much that it sometimes leaves you breathless - clan, custom, land, and loved ones. You stand together with your comrades-in-arms in defense of the people, but though the clans were dissolved the old fault lines are always there, just below the surface – cracks into which the Shadow seeps, hoping to shatter the strength of the people once more. PLAYERS MUST PLAY IN ALL THREE ROUNDS.",
      "presenters": "Anna Kreider",
      "authors": "Northfire Games",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R220",
      "title": "My Kingdom For a Horse",
      "system": "Threadbare",
      "description": "An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! Threadbare, the Stitchpunk RPG is set in a broken world populated by broken toys. Rescue yourself and others and maybe leave the world a little bit better. CW: This game inherently contains some amount of body horror.",
      "presenters": "Timothy Rodriguez",
      "authors": "Stephanie Bryant",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R221",
      "title": "The Gene Hack\" presented by Scott Mina. On the edge of the vast Most Excellent and Shining Empire, lies the planet Gedron-Stein VII, commonly known as the \"Heap",
      "system": "",
      "description": "a junkyard planet where the detritus of the Galaxy eventually ends up. Players will be a band of mutants, freaks, and misfits who struggle to eke out a living here. The Gene Hack is a simple OSR system, based on the popular Black Hack. It's similar to old-school D&D. No experience necessary.",
      "presenters": "",
      "authors": "",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R222",
      "title": "Rememorex: Cold War Continuum 1987",
      "system": "Omnisystem",
      "description": "It's 1987. Alf is the king of late night TV, Michael Jackson is the king of pop, and you're stuck in Twin Mills, Pennsylvania for the summer when all the cool kids are going to the beach or the mountains. At least you get to go on a camping trip out in the Maitland Wilds with your friends. You've heard stories of the Patterson Shale wild man out there, and you've brought a net, some glue, enough Fruit Roll-ups to capture some kind of shaggy forest monster. What could go wrong? Nerdy City presents an epic four-part epic event especially for DREAMATION. Up to four tables will be running simultaneously, complete with audio and video augmentation. Players can play in any or all of the four rounds.",
      "presenters": "Sean Jaffe",
      "authors": "Nerdy City",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R223",
      "title": "The Absalom Lock",
      "system": "Starfinder",
      "description": "Calo Ahnt is an intermediary for powerful people in Absalom Station; he farms out assignments, pays good credits, and helps keep things running quietly from his court in the Shattered Drive. You are new blood - you've demonstrated your skill, but have not yet earned enough reputation to really be successful as freelancers, but you've performed well enough to be invited to Calo's private suite regarding a higher-profile assignment. Pre-generated characters will be provided. No experience with the Starfinder system necessary; rules will be taught.",
      "presenters": "John Gallo",
      "authors": "",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R224",
      "title": "Guild Wars 2018, Part One",
      "system": "World Wide Wrestling",
      "description": "Passions have been brewing all year, and It's time to prove whose Guild is the most dominant faction in the World Wide Wrestling. Gain allies and betray old friends as you fight, talk, and cheat your way to the top of the world of Wrestling! This exciting event hopes to focus on more on tag team wrestling, and factions working together to prove themselves and win the title of The Guild Masters! World Wide Wrestling is a pro wrestling RPG based on the Apocalypse World engine, featuring: Rich characters built on iconic wrestling archetypes, player decisions with real consequences that shape the unfolding storylines and feuds, and exciting narrative action, both in and out of the ring. It is a game that pays homage to and seeks to play out the classic Soap Opera stories from Professional Wrestling. Payers are encouraged to play all three rounds of this 3-part scenario, but it is not necessary.",
      "presenters": "Joseph Zantek",
      "authors": "ndpdesign",
      "time": "Thursday, 8:00PM - 12:00AM",
      "tags": ""
    },
    {
      "code": "R225",
      "title": "The Fate of Walmund's Patrol",
      "system": "Mouse Guard 2nd Edition",
      "description": "An INDEPENDENTLY PUBLISHED GAME - Part of the Indie Games Explosion! This scenario focuses on a Mouse guard patrol who is in over their head. After searching for many weeks in the hot sun, the players finally find the tattered cloak of one of Walmund's patrol. But as the players will soon find out that things are not as they seem. As the patrol leader has been given a secret objective to finish Walmund's patrol mission of gathering intelligence on the Minks that have been raiding in the area.",
      "presenters": "Evan Buchholz",
      "authors": "Burning Wheel Headquarters",
      "time": "Thursday, 9:00PM - 12:00AM",
      "tags": ""
    },
  ];
}
</script>
