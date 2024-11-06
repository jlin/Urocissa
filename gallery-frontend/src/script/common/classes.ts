// classes.ts

import { DisplayElement } from './types';

/**
 * Represents a sub-row containing display elements.
 */
export class SubRow {
  displayElements: DisplayElement[];

  constructor(displayElements: DisplayElement[]) {
    this.displayElements = displayElements;
  }
}
