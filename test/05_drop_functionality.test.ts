import { DateItem } from "../src/models/romcal-date-item";

/*
    The MIT License (MIT)

    Copyright (c) 2014 Pereira, Julian Matthew

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
    THE SOFTWARE.
*/

import moment from "moment";
import { Calendar } from "../src";

// eslint-disable-next-line quotes
describe('Testing the "drop" functionality for national calendars', () => {
    const testDates = Calendar.calendarFor({
        country: "slovakia",
        year: 2020,
    }) as Array<DateItem>;

    it("A dropped celebration should not be appended in the final calendar", () => {
        const date = testDates.find(d => {
            return d.moment.isSame(moment.utc({ year: 2020, month: 11, day: 25 }));
        });
        expect(date.key).not.toEqual("christmas");
    });
});