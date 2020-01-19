[![](https://github.com/jmatth/dextools/workflows/Website%20deploy/badge.svg)](https://github.com/jmatth/dextools/actions?query=workflow%3A%22Website+deploy%22)
[![](https://github.com/jmatth/dextools/workflows/Scraper%20CI/badge.svg)](https://github.com/jmatth/dextools/actions?query=workflow%3A%22Scraper+CI%22)

Dextools
========
Dextools is a collection of tools for parsing and displaying convention
schedules from Double Exposure. It currently consists of two components:

1. A scraper that takes in the schedule's HTML and produces a JSON file.
2. A web frontend for displaying the JSON with convenient search features, a
   calendar view for building your own personal schedule for the convention, and
   export options for producing an ics file or email text to sign up for your
   selected events.

Dextools is currently hosted at [dextools.jmatth.com](https://dextools.jmatth.com).

## Technology
The scraper is written in Rust with various crates for parsing HTML and working
with times, etc.

The web frontend is written in Vue.js with Vuex and Vuetify.
