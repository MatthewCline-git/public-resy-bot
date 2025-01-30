# Resy Reservation Bot

A Rust-based automation tool for securing hard-to-get restaurant reservations on Resy. This bot helps you grab reservations at your desired time by automatically attempting to book when reservations become available.

## Disclaimer

This is a fun, hacky project that helped me reverse engineer API endpoints and build in Rust. I don't hoard reservations, I don't sell them, 
I dislike when people do. I have strategically removed parts of this code to prevent it from being abused. 

## Features

- Configurable reservation parameters (date, party size, time windows)
- Support for both indoor and outdoor seating preferences
- Automatic retry mechanism with intelligent backoff
- Precise timing control for reservation releases
- Error handling and logging

## Error Handling

The bot handles several types of errors:
- `NoAvailableReservations`: No reservations found at the moment
- `CannotFindReservation`: No reservations matching your criteria
- `UnknownError`: Various API or network-related errors

## Architecture

The project is organized into several modules:

- `api.rs`: Low-level Resy API client
- `client.rs`: High-level reservation finding and booking logic
- `models.rs`: Data structures and error types
- `types.rs`: Configuration and data type definitions
- `workflow.rs`: Main booking workflow orchestration

## License

MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.