import moment, { Moment } from 'moment';

export default class Event {
  [key: string]: string|Moment;
  public code: string;
  public title: string;
  public description: string;
  public system: string;
  public presenters: string;
  public authors: string;
  public startTime: Moment;
  public endTime: Moment;

  constructor({
      code,
      title,
      description,
      system,
      presenters,
      authors,
      start_time,
      end_time,
      tags,
    }: {
      code: string,
      title: string,
      description: string,
      system: string,
      presenters: string,
      authors: string,
      start_time: string,
      end_time: string,
      tags: string,
    },
  ) {
    this.code = code;
    this.title = title;
    this.description = description;
    this.system = system;
    this.presenters = presenters;
    this.authors = authors;
    this.startTime = moment(start_time);
    this.endTime = moment(end_time);
  }
}
