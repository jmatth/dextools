import { expect } from 'chai';
import moment, { Moment } from 'moment';

import Agenda from '@/models/agenda';
import Event from '@/models/Event';
import { createEvent } from './common';

import '../setup';

const eventToCode = (e: Event) => e.code;

describe('models/agenda.ts', function() {
  let agenda: Agenda;
  beforeEach(function() {
    agenda = new Agenda();
  });

  it('starts with an empty events array', function() {
    expect(agenda.events).to.be.empty;
  });

  it('starts with lastAdded empty', function() {
    expect(agenda.lastAdded).to.be.undefined;
  });

  describe('add one event', function() {
    let firstEvent: Event;
    beforeEach(function() {
      firstEvent = createEvent({ code: 'T003' });
      agenda.addEvent(firstEvent);
    });

    it('adds the event to the array', function() {
      expect(agenda.events.length).to.equal(1);
      expect(agenda.events[0].code).to.eql(firstEvent.code);
    });

    it('populates lastAdded', function() {
      expect(agenda.lastAdded).to.exist;
      expect(agenda.lastAdded!.code).to.equal(firstEvent.code);
    });

    describe('add second event of same length', function() {
      let secondEvent: Event;
      beforeEach(function() {
        secondEvent = createEvent({
          code: 'T002',
          start_time: firstEvent.startTime.format(),
          end_time: firstEvent.endTime.format(),
        });
        agenda.addEvent(secondEvent);
      });

      it('sorts the events by code', function() {
        expect(agenda.events.map(eventToCode)).to.eql([secondEvent.code, firstEvent.code]);
      });

      it('updates lastAdded', function() {
        expect(agenda.lastAdded).to.eql(secondEvent);
      });

      describe('add a duplicate event', function() {
        let duplicateEvent: Event;
        beforeEach(function() {
          duplicateEvent = createEvent({ code: agenda.events[1].code });
          agenda.addEvent(duplicateEvent);
        });

        it('ignores the duplicate event', function() {
          expect(agenda.events.map(eventToCode)).to.eql([secondEvent.code, firstEvent.code]);
        });

        it('still updates lastAdded', function() {
          expect(agenda.lastAdded!.code).to.equal(duplicateEvent.code);
        });
      });
    });

    describe('add second event of shorter length', function() {
      let secondEvent: Event;
      beforeEach(function() {
        secondEvent = createEvent({
          code: 'T900',
          start_time: firstEvent.startTime.format(),
          end_time: moment(firstEvent.startTime).subtract(30, 'minutes').format(),
        });
        agenda.addEvent(secondEvent);
      });

      it('sorts the events by length', function() {
        expect(agenda.events.map(eventToCode)).to.eql([secondEvent.code, firstEvent.code]);
      });

      it('updates lastAdded', function() {
        expect(agenda.lastAdded).to.exist;
        expect(agenda.lastAdded!.code).to.eql(secondEvent.code);
      });
    });

    describe('add second event that starts earlier', function() {
      let secondEvent: Event;
      beforeEach(function() {
        secondEvent = createEvent({
          code: 'T900',
          start_time: moment(firstEvent.startTime).subtract(1, 'hours'),
          end_time: moment(firstEvent.endTime).subtract(1, 'hours'),
        });
        agenda.addEvent(secondEvent);
      });

      it('sorts the events by start time', function() {
        expect(agenda.events.map(eventToCode)).to.eql([secondEvent.code, firstEvent.code]);
      });

      it('updates lastAdded', function() {
        expect(agenda.lastAdded).to.exist;
        expect(agenda.lastAdded!.code).to.eql(secondEvent.code);
      });
    });
  });

  describe('remove an event', function() {
    let firstEvent: Event;
    let secondEvent: Event;
    let thirdEvent: Event;
    beforeEach(function() {
      firstEvent = createEvent({ code: 'T001' });
      agenda.addEvent(firstEvent);
      secondEvent = createEvent({ code: 'T002' });
      agenda.addEvent(secondEvent);
      thirdEvent = createEvent({ code: 'T003' });
      agenda.addEvent(thirdEvent);

      // The upcoming tests rely on eventTwo being in the middle,
      // ensure that is in fact the case.
      expect(agenda.events.map(eventToCode)).to.eql([
        firstEvent.code,
        secondEvent.code,
        thirdEvent.code,
      ]);

      agenda.removeEvent(secondEvent.code);
    });

    it('does not leave gaps in the agenda', function() {
      expect(agenda.events.map(eventToCode)).to.eql([
        firstEvent.code,
        thirdEvent.code,
      ]);
    });
  });
});
