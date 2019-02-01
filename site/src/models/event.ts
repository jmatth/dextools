export default class Event {
  public code: string;
  public title: string;
  public description: string;
  public system: string;
  public presenters: string;
  public authors: string;
  public startTime: Date;
  public endTime: Date;

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
    this.startTime = new Date(start_time);
    this.endTime = new Date(end_time);
  }
}
