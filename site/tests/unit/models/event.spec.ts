import { expect } from 'chai';
import moment, { Moment } from 'moment';

import Event from '@/models/event';
import { createEvent } from './common';

describe('models/event.ts', function() {
  describe('conflicts', function() {
    it('returns false for the same event', function() {
      const event1 = createEvent({ code: 'R001' });
      const event2 = createEvent({ code: 'R001' });
      expect(event1.conflicts(event2)).to.be.false;
      expect(event2.conflicts(event1)).to.be.false;
    });

    it('returns true if two events have the same start and end', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 09:00',
        end_time: '2020-11-01 10:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 09:00',
        end_time: '2020-11-01 10:00',
      });
      expect(event1.conflicts(event2)).to.be.true;
      expect(event2.conflicts(event1)).to.be.true;
    });

    it('returns true if event1 runs into event2', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 09:00',
        end_time: '2020-11-01 11:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 10:00',
        end_time: '2020-11-01 12:00',
      });
      expect(event1.conflicts(event2)).to.be.true;
    });

    it('returns true if event1 starts during event2', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 10:00',
        end_time: '2020-11-01 12:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 09:00',
        end_time: '2020-11-01 11:00',
      });
      expect(event1.conflicts(event2)).to.be.true;
    });

    it('returns false if event1 is before event2', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 10:00',
        end_time: '2020-11-01 12:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 13:00',
        end_time: '2020-11-01 15:00',
      });
      expect(event1.conflicts(event2)).to.be.false;
    });

    it('returns false if event1 is after event2', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 13:00',
        end_time: '2020-11-01 15:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 10:00',
        end_time: '2020-11-01 12:00',
      });
      expect(event1.conflicts(event2)).to.be.false;
    });

    it('returns false if event1 ends when event2 begins', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 13:00',
        end_time: '2020-11-01 15:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 15:00',
        end_time: '2020-11-01 17:00',
      });
      expect(event1.conflicts(event2)).to.be.false;
    });

    it('returns false if event1 starts when event2 ends', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-11-01 15:00',
        end_time: '2020-11-01 17:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-11-01 13:00',
        end_time: '2020-11-01 15:00',
      });
      expect(event1.conflicts(event2)).to.be.false;
    });
  });

  describe('compare', function() {
    it('considers events with the same code to be equal', function() {
      const event1 = createEvent({ code: 'R001' });
      const event2 = createEvent({ code: 'R001' });
      expect(event1.compare(event2)).to.equal(0);
      expect(event2.compare(event1)).to.equal(0);
    });

    it('sorts earlier events before later events', function() {
      const event1 = createEvent({
        code: 'R001',
        start_time: '2020-02-19 09:00',
        end_time: '2020-02-19 13:00',
      });
      const event2 = createEvent({
        code: 'R002',
        start_time: '2020-02-19 10:00',
        end_time: '2020-02-19 12:00',
      });
      expect(event1.compare(event2)).to.be.below(0);
      expect(event2.compare(event1)).to.be.above(0);
    });

    it('sorts events with the same start time by code number', function() {
      const event1 = createEvent({ code: 'R001' });
      const event2 = createEvent({ code: 'R002' });
      expect(event1.compare(event2)).to.be.below(0);
      expect(event2.compare(event1)).to.be.above(0);
    });
  });
});
